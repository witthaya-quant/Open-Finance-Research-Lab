use american_put_workbench::{
    lattice_node_count, price_american_put, verify_optimality_certificate, AmericanPutModel,
    ModelError, NodeDecision, Rational, VerificationError,
};
use num_bigint::BigInt;
use num_rational::BigRational;
use std::cmp::Ordering;

fn r(numerator: i64, denominator: i64) -> Rational {
    BigRational::new(BigInt::from(numerator), BigInt::from(denominator))
}

fn model(periods: usize) -> AmericanPutModel {
    AmericanPutModel {
        initial_spot: r(100, 1),
        strike: r(100, 1),
        up_factor: r(6, 5),
        down_factor: r(4, 5),
        risk_free_gross: r(21, 20),
        periods,
    }
}

fn node_index(time: usize, up_moves: usize) -> usize {
    let (left, right) = if time % 2 == 0 {
        (time / 2, time + 1)
    } else {
        (time, (time + 1) / 2)
    };
    left * right + up_moves
}

fn payoff(strike: &Rational, spot: &Rational) -> Rational {
    if strike > spot {
        strike - spot
    } else {
        r(0, 1)
    }
}

fn rational_power(base: &Rational, exponent: usize) -> Rational {
    let mut result = r(1, 1);
    for _ in 0..exponent {
        result *= base;
    }
    result
}

#[test]
fn exact_three_period_reference_and_decisions() {
    let input = model(3);
    let result = price_american_put(&input, 10).expect("valid reference model");

    assert_eq!(result.risk_neutral_up_probability, r(5, 8));
    assert_eq!(result.initial_value, r(18_325, 2_058));
    assert_eq!(result.lattice_node_count, 10);
    assert_eq!(result.nodes.len(), 10);

    assert_eq!(
        result.nodes[node_index(0, 0)].decision,
        NodeDecision::Continue
    );
    assert_eq!(
        result.nodes[node_index(1, 0)].decision,
        NodeDecision::Exercise
    );
    assert_eq!(
        result.nodes[node_index(1, 1)].decision,
        NodeDecision::Continue
    );
    assert_eq!(
        result.nodes[node_index(2, 0)].decision,
        NodeDecision::Exercise
    );
    assert_eq!(
        result.nodes[node_index(2, 1)].decision,
        NodeDecision::Continue
    );
    assert_eq!(
        result.nodes[node_index(2, 2)].decision,
        NodeDecision::Indifferent
    );

    verify_optimality_certificate(&input, &result, 10)
        .expect("exact reference certificate must verify");
}

#[test]
fn maturity_only_model_returns_intrinsic_value() {
    let input = model(0);
    let result = price_american_put(&input, 1).expect("zero-period model is valid");

    assert_eq!(result.initial_value, r(0, 1));
    assert_eq!(result.nodes.len(), 1);
    assert_eq!(result.nodes[0].decision, NodeDecision::Terminal);
    verify_optimality_certificate(&input, &result, 1).expect("terminal certificate");
}

#[test]
fn rejects_non_positive_parameters_and_arbitrage_domain() {
    let mut non_positive = model(3);
    non_positive.initial_spot = r(0, 1);
    assert!(matches!(
        price_american_put(&non_positive, 100),
        Err(ModelError::NonPositiveParameter {
            name: "initial_spot"
        })
    ));

    let mut arbitrage = model(3);
    arbitrage.risk_free_gross = r(6, 5);
    assert_eq!(
        price_american_put(&arbitrage, 100),
        Err(ModelError::NoArbitrageViolation)
    );
}

#[test]
fn rejects_resource_limit_before_building_lattice() {
    assert_eq!(lattice_node_count(3), Ok(10));
    assert_eq!(
        price_american_put(&model(3), 9),
        Err(ModelError::ResourceLimitExceeded {
            required_nodes: 10,
            maximum_nodes: 9
        })
    );
}

#[test]
fn independent_verifier_rejects_corrupted_node_value() {
    let input = model(3);
    let mut result = price_american_put(&input, 10).expect("valid model");

    let corrupted_index = node_index(2, 0);
    result.nodes[corrupted_index].option_value += r(1, 1);

    assert!(matches!(
        verify_optimality_certificate(&input, &result, 10),
        Err(VerificationError::BellmanDominanceViolation { index })
            if index == corrupted_index
    ));
}

#[test]
fn independent_verifier_rejects_corrupted_metadata() {
    let input = model(3);
    let mut result = price_american_put(&input, 10).expect("valid model");
    result.nodes[4].up_moves = 99;

    assert_eq!(
        verify_optimality_certificate(&input, &result, 10),
        Err(VerificationError::NodeMetadataMismatch { index: 4 })
    );
}

#[test]
fn exhaustive_history_and_markov_policy_spaces_match_bellman_value() {
    let input = model(3);
    let result = price_american_put(&input, 10).expect("valid model");
    let q = r(5, 8);
    let down_probability = r(3, 8);
    let inverse_discount = r(20, 21);

    // Each terminal path receives its full path probability, even when a
    // selected policy exercises before the path is completely observed.
    let history_policy_count = 1_usize << 7;
    let history_path_count = 1_usize << 3;
    let mut history_max = r(-1, 1);
    let mut history_maximizers = 0_usize;

    for policy in 0..history_policy_count {
        let mut expected_value = r(0, 1);

        for path in 0..history_path_count {
            let mut path_probability = r(1, 1);
            for time in 0..input.periods {
                let shift = input.periods - time - 1;
                let is_up = ((path >> shift) & 1) == 1;
                path_probability *= if is_up { &q } else { &down_probability };
            }

            let mut spot = input.initial_spot.clone();
            let mut exercise_time = None;

            for time in 0..input.periods {
                let history_code = path >> (input.periods - time);
                let history_index = (1_usize << time) - 1 + history_code;
                if ((policy >> history_index) & 1) == 1 {
                    exercise_time = Some(time);
                    break;
                }

                let shift = input.periods - time - 1;
                let is_up = ((path >> shift) & 1) == 1;
                if is_up {
                    spot *= &input.up_factor;
                } else {
                    spot *= &input.down_factor;
                }
            }

            let (time, payoff_value) = match exercise_time {
                Some(time) => (time, payoff(&input.strike, &spot)),
                None => (input.periods, payoff(&input.strike, &spot)),
            };

            expected_value +=
                path_probability * payoff_value * rational_power(&inverse_discount, time);
        }

        match expected_value.cmp(&history_max) {
            Ordering::Greater => {
                history_max = expected_value;
                history_maximizers = 1;
            }
            Ordering::Equal => history_maximizers += 1,
            Ordering::Less => {}
        }
    }

    // Six recombining non-terminal state nodes: 1 + 2 + 3.
    let markov_policy_count = 1_usize << 6;
    let mut markov_max = r(-1, 1);
    let mut markov_maximizers = 0_usize;

    for policy in 0..markov_policy_count {
        let mut expected_value = r(0, 1);

        for path in 0..history_path_count {
            let mut path_probability = r(1, 1);
            for time in 0..input.periods {
                let shift = input.periods - time - 1;
                let is_up = ((path >> shift) & 1) == 1;
                path_probability *= if is_up { &q } else { &down_probability };
            }

            let mut spot = input.initial_spot.clone();
            let mut up_moves = 0_usize;
            let mut exercise_time = None;

            for time in 0..input.periods {
                let state_index = node_index(time, up_moves);
                if ((policy >> state_index) & 1) == 1 {
                    exercise_time = Some(time);
                    break;
                }

                let shift = input.periods - time - 1;
                let is_up = ((path >> shift) & 1) == 1;
                if is_up {
                    spot *= &input.up_factor;
                    up_moves += 1;
                } else {
                    spot *= &input.down_factor;
                }
            }

            let (time, payoff_value) = match exercise_time {
                Some(time) => (time, payoff(&input.strike, &spot)),
                None => (input.periods, payoff(&input.strike, &spot)),
            };

            expected_value +=
                path_probability * payoff_value * rational_power(&inverse_discount, time);
        }

        match expected_value.cmp(&markov_max) {
            Ordering::Greater => {
                markov_max = expected_value;
                markov_maximizers = 1;
            }
            Ordering::Equal => markov_maximizers += 1,
            Ordering::Less => {}
        }
    }

    assert_eq!(history_policy_count, 128);
    assert_eq!(markov_policy_count, 64);
    assert_eq!(history_max, result.initial_value);
    assert_eq!(markov_max, result.initial_value);
    assert_eq!(history_maximizers, 8);
    assert_eq!(markov_maximizers, 4);
}
