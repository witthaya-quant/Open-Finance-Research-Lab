//! Exact-rational finite-binomial American put valuation.
//!
//! This research package is case-specific. It does not fetch market data,
//! calibrate parameters, or expose a trading or production pricing interface.

use num_bigint::BigInt;
use num_rational::BigRational;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt;

pub type Rational = BigRational;

/// Finite-horizon, no-dividend binomial American put parameters.
///
/// All numeric inputs are exact rationals. risk_free_gross is the gross
/// accumulation factor per model period, not an annual percentage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmericanPutModel {
    pub initial_spot: Rational,
    pub strike: Rational,
    pub up_factor: Rational,
    pub down_factor: Rational,
    pub risk_free_gross: Rational,
    pub periods: usize,
}

impl AmericanPutModel {
    pub fn validate(&self) -> Result<(), ModelError> {
        for (name, value) in [
            ("initial_spot", &self.initial_spot),
            ("strike", &self.strike),
            ("up_factor", &self.up_factor),
            ("down_factor", &self.down_factor),
            ("risk_free_gross", &self.risk_free_gross),
        ] {
            if value <= &zero() {
                return Err(ModelError::NonPositiveParameter { name });
            }
        }

        if !(self.down_factor < self.risk_free_gross && self.risk_free_gross < self.up_factor) {
            return Err(ModelError::NoArbitrageViolation);
        }

        let _ = lattice_node_count(self.periods)?;
        Ok(())
    }

    pub fn risk_neutral_up_probability(&self) -> Result<Rational, ModelError> {
        self.validate()?;
        Ok((&self.risk_free_gross - &self.down_factor) / (&self.up_factor - &self.down_factor))
    }
}

/// A node's optimal action under exact comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeDecision {
    Exercise,
    Continue,
    Indifferent,
    Terminal,
}

/// Exact value and policy information for one recombining lattice node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeValuation {
    pub time: usize,
    pub up_moves: usize,
    pub spot_price: Rational,
    pub exercise_value: Rational,
    pub continuation_value: Option<Rational>,
    pub option_value: Rational,
    pub decision: NodeDecision,
}

/// Full node-value and stopping-policy certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Valuation {
    pub risk_neutral_up_probability: Rational,
    pub initial_value: Rational,
    pub lattice_node_count: usize,
    /// Nodes are stored by increasing time, then increasing up-move count.
    pub nodes: Vec<NodeValuation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModelError {
    NonPositiveParameter {
        name: &'static str,
    },
    NoArbitrageViolation,
    NodeCountOverflow,
    ResourceLimitExceeded {
        required_nodes: usize,
        maximum_nodes: usize,
    },
    AllocationFailed,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ModelError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerificationError {
    InvalidModel(ModelError),
    NodeCountMismatch {
        expected: usize,
        actual: usize,
    },
    ResourceLimitExceeded {
        required_nodes: usize,
        maximum_nodes: usize,
    },
    RiskNeutralProbabilityMismatch,
    NodeMetadataMismatch {
        index: usize,
    },
    SpotMismatch {
        index: usize,
    },
    ExerciseValueMismatch {
        index: usize,
    },
    ContinuationMismatch {
        index: usize,
    },
    TerminalConditionViolation {
        index: usize,
    },
    BellmanDominanceViolation {
        index: usize,
    },
    PolicyAttainabilityViolation {
        index: usize,
    },
    RootValueMismatch,
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for VerificationError {}

/// Return the total number of nodes in a recombining binomial lattice.
///
/// The triangular-number calculation divides an even factor before
/// multiplication and uses checked arithmetic.
pub fn lattice_node_count(periods: usize) -> Result<usize, ModelError> {
    let mut left = periods
        .checked_add(1)
        .ok_or(ModelError::NodeCountOverflow)?;
    let mut right = periods
        .checked_add(2)
        .ok_or(ModelError::NodeCountOverflow)?;

    if left % 2 == 0 {
        left /= 2;
    } else {
        right /= 2;
    }

    left.checked_mul(right).ok_or(ModelError::NodeCountOverflow)
}

/// Price the American put by exact-rational backward induction.
///
/// maximum_nodes is an explicit resource ceiling. It is checked before
/// lattice storage is allocated. The function returns all nodes so that the
/// result can be audited and independently verified.
pub fn price_american_put(
    model: &AmericanPutModel,
    maximum_nodes: usize,
) -> Result<Valuation, ModelError> {
    model.validate()?;
    let count = lattice_node_count(model.periods)?;

    if count > maximum_nodes {
        return Err(ModelError::ResourceLimitExceeded {
            required_nodes: count,
            maximum_nodes,
        });
    }

    let q = (&model.risk_free_gross - &model.down_factor) / (&model.up_factor - &model.down_factor);
    let one_minus_q = one() - &q;

    let mut spots = Vec::new();
    spots
        .try_reserve_exact(count)
        .map_err(|_| ModelError::AllocationFailed)?;
    spots.resize(count, zero());

    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| ModelError::AllocationFailed)?;
    values.resize(count, zero());

    spots[0] = model.initial_spot.clone();

    for time in 1..=model.periods {
        for up_moves in 0..=time {
            let index = node_index(time, up_moves);
            let (parent_time, parent_up, factor) = if up_moves == time {
                (time - 1, up_moves - 1, &model.up_factor)
            } else {
                (time - 1, up_moves, &model.down_factor)
            };
            let parent_spot = &spots[node_index(parent_time, parent_up)];
            spots[index] = parent_spot * factor;
        }
    }

    let terminal_time = model.periods;
    for up_moves in 0..=terminal_time {
        let index = node_index(terminal_time, up_moves);
        values[index] = put_payoff(&model.strike, &spots[index]);
    }

    for time in (0..model.periods).rev() {
        for up_moves in 0..=time {
            let index = node_index(time, up_moves);
            let up_child = node_index(time + 1, up_moves + 1);
            let down_child = node_index(time + 1, up_moves);

            let continuation = (&q * &values[up_child] + &one_minus_q * &values[down_child])
                / &model.risk_free_gross;
            let exercise = put_payoff(&model.strike, &spots[index]);

            values[index] = if exercise >= continuation {
                exercise
            } else {
                continuation
            };
        }
    }

    let mut nodes = Vec::new();
    nodes
        .try_reserve_exact(count)
        .map_err(|_| ModelError::AllocationFailed)?;

    for time in 0..=model.periods {
        for up_moves in 0..=time {
            let index = node_index(time, up_moves);
            let exercise = put_payoff(&model.strike, &spots[index]);

            if time == model.periods {
                nodes.push(NodeValuation {
                    time,
                    up_moves,
                    spot_price: spots[index].clone(),
                    exercise_value: exercise,
                    continuation_value: None,
                    option_value: values[index].clone(),
                    decision: NodeDecision::Terminal,
                });
            } else {
                let up_child = node_index(time + 1, up_moves + 1);
                let down_child = node_index(time + 1, up_moves);
                let continuation = (&q * &values[up_child] + &one_minus_q * &values[down_child])
                    / &model.risk_free_gross;

                let decision = match exercise.cmp(&continuation) {
                    Ordering::Greater => NodeDecision::Exercise,
                    Ordering::Less => NodeDecision::Continue,
                    Ordering::Equal => NodeDecision::Indifferent,
                };

                nodes.push(NodeValuation {
                    time,
                    up_moves,
                    spot_price: spots[index].clone(),
                    exercise_value: exercise,
                    continuation_value: Some(continuation),
                    option_value: values[index].clone(),
                    decision,
                });
            }
        }
    }

    Ok(Valuation {
        risk_neutral_up_probability: q,
        initial_value: values[0].clone(),
        lattice_node_count: count,
        nodes,
    })
}

/// Verify a valuation certificate without calling the pricing routine.
///
/// The verifier independently recomputes model states, payoffs and
/// continuation bounds. It rejects a certificate whose values do not
/// dominate exercise and continuation or whose stated policy fails to
/// attain the candidate value.
pub fn verify_optimality_certificate(
    model: &AmericanPutModel,
    valuation: &Valuation,
    maximum_nodes: usize,
) -> Result<(), VerificationError> {
    model.validate().map_err(VerificationError::InvalidModel)?;

    let expected_count =
        lattice_node_count(model.periods).map_err(VerificationError::InvalidModel)?;

    if expected_count > maximum_nodes {
        return Err(VerificationError::ResourceLimitExceeded {
            required_nodes: expected_count,
            maximum_nodes,
        });
    }

    if valuation.nodes.len() != expected_count || valuation.lattice_node_count != expected_count {
        return Err(VerificationError::NodeCountMismatch {
            expected: expected_count,
            actual: valuation.nodes.len(),
        });
    }

    let q = (&model.risk_free_gross - &model.down_factor) / (&model.up_factor - &model.down_factor);
    let one_minus_q = one() - &q;

    if valuation.risk_neutral_up_probability != q {
        return Err(VerificationError::RiskNeutralProbabilityMismatch);
    }

    for time in 0..=model.periods {
        for up_moves in 0..=time {
            let index = node_index(time, up_moves);
            let node = &valuation.nodes[index];

            if node.time != time || node.up_moves != up_moves {
                return Err(VerificationError::NodeMetadataMismatch { index });
            }

            // Recompute state from the closed-form model definition rather
            // than using the valuation routine's precomputed spot lattice.
            let expected_spot = &model.initial_spot
                * rational_power(&model.up_factor, up_moves)
                * rational_power(&model.down_factor, time - up_moves);

            if node.spot_price != expected_spot {
                return Err(VerificationError::SpotMismatch { index });
            }

            let expected_exercise = put_payoff(&model.strike, &expected_spot);
            if node.exercise_value != expected_exercise {
                return Err(VerificationError::ExerciseValueMismatch { index });
            }

            if time == model.periods {
                if node.continuation_value.is_some()
                    || node.option_value != expected_exercise
                    || node.decision != NodeDecision::Terminal
                {
                    return Err(VerificationError::TerminalConditionViolation { index });
                }
                continue;
            }

            let up_child = &valuation.nodes[node_index(time + 1, up_moves + 1)];
            let down_child = &valuation.nodes[node_index(time + 1, up_moves)];
            let expected_continuation = (&q * &up_child.option_value
                + &one_minus_q * &down_child.option_value)
                / &model.risk_free_gross;

            let Some(reported_continuation) = &node.continuation_value else {
                return Err(VerificationError::ContinuationMismatch { index });
            };

            if reported_continuation != &expected_continuation {
                return Err(VerificationError::ContinuationMismatch { index });
            }

            if node.option_value < expected_exercise || node.option_value < expected_continuation {
                return Err(VerificationError::BellmanDominanceViolation { index });
            }

            let expected_decision = match expected_exercise.cmp(&expected_continuation) {
                Ordering::Greater => NodeDecision::Exercise,
                Ordering::Less => NodeDecision::Continue,
                Ordering::Equal => NodeDecision::Indifferent,
            };

            let attains_value = match expected_decision {
                NodeDecision::Exercise => node.option_value == expected_exercise,
                NodeDecision::Continue => node.option_value == expected_continuation,
                NodeDecision::Indifferent => {
                    node.option_value == expected_exercise
                        && node.option_value == expected_continuation
                }
                NodeDecision::Terminal => false,
            };

            if node.decision != expected_decision || !attains_value {
                return Err(VerificationError::PolicyAttainabilityViolation { index });
            }
        }
    }

    if valuation.nodes[0].option_value != valuation.initial_value {
        return Err(VerificationError::RootValueMismatch);
    }

    Ok(())
}

fn zero() -> Rational {
    Rational::from_integer(BigInt::from(0_u8))
}

fn one() -> Rational {
    Rational::from_integer(BigInt::from(1_u8))
}

fn put_payoff(strike: &Rational, spot: &Rational) -> Rational {
    if strike > spot {
        strike - spot
    } else {
        zero()
    }
}

// The divide-before-multiply form is safe whenever the checked triangular
// node count itself fits in usize.
fn node_index(time: usize, up_moves: usize) -> usize {
    let (left, right) = if time % 2 == 0 {
        (time / 2, time + 1)
    } else {
        (time, time.div_ceil(2))
    };
    left * right + up_moves
}

fn rational_power(base: &Rational, exponent: usize) -> Rational {
    let mut power = exponent;
    let mut factor = base.clone();
    let mut result = one();

    while power > 0 {
        if power % 2 == 1 {
            result *= &factor;
        }
        power /= 2;
        if power > 0 {
            factor = &factor * &factor;
        }
    }

    result
}
