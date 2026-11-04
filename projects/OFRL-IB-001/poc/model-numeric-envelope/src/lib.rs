use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};
use rust_decimal::Decimal;
use serde_json::Value;

const CASE_A: &str =
    include_str!("../../../../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json");

const MONEY_FIELDS: &[&str] = &[
    "cash",
    "accounts_receivable",
    "inventory",
    "gross_ppe",
    "accumulated_depreciation",
    "accounts_payable",
    "interest_bearing_debt",
    "share_capital",
    "retained_earnings",
    "revenue",
    "cogs",
    "operating_expenses",
    "depreciation",
    "interest_expense",
    "income_tax_expense",
    "net_income",
    "depreciation_addback",
    "change_in_operating_nwc",
    "cash_from_operations",
    "capex",
    "cash_from_investing",
    "debt_drawdowns",
    "principal_repayments",
    "dividends",
    "cash_from_financing",
    "net_change_in_cash",
    "opening_cash",
    "closing_cash",
];

pub trait Numeric: Clone + std::fmt::Debug {
    fn parse(value: &str) -> Result<Self, String>;
    fn zero() -> Self;
    fn one() -> Self;
    fn from_i64(value: i64) -> Self;
    fn add(&self, rhs: &Self) -> Result<Self, String>;
    fn sub(&self, rhs: &Self) -> Result<Self, String>;
    fn mul(&self, rhs: &Self) -> Result<Self, String>;
    fn div(&self, rhs: &Self) -> Result<Self, String>;
    fn compare(&self, rhs: &Self) -> Ordering;
    fn as_rational(&self) -> BigRational;
    fn raw_text(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct FixedDecimal(pub Decimal);

impl Numeric for FixedDecimal {
    fn parse(value: &str) -> Result<Self, String> {
        if let Some((_, fractional)) = value.split_once('.') {
            if fractional.len() > Decimal::MAX_SCALE as usize {
                return Err(format!("scale exceeds Decimal maximum: {value}"));
            }
        }

        let parsed = Decimal::from_str(value).map_err(|error| error.to_string())?;
        if parsed.to_string() != value {
            return Err(format!("Decimal parse changed input text: {value}"));
        }
        Ok(Self(parsed))
    }

    fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    fn one() -> Self {
        Self(Decimal::ONE)
    }

    fn from_i64(value: i64) -> Self {
        Self(Decimal::from(value))
    }

    fn add(&self, rhs: &Self) -> Result<Self, String> {
        self.0
            .checked_add(rhs.0)
            .map(Self)
            .ok_or_else(|| format!("checked Decimal addition overflow: {} + {}", self.0, rhs.0))
    }

    fn sub(&self, rhs: &Self) -> Result<Self, String> {
        self.0
            .checked_sub(rhs.0)
            .map(Self)
            .ok_or_else(|| format!("checked Decimal subtraction overflow: {} - {}", self.0, rhs.0))
    }

    fn mul(&self, rhs: &Self) -> Result<Self, String> {
        self.0
            .checked_mul(rhs.0)
            .map(Self)
            .ok_or_else(|| format!("checked Decimal multiplication overflow: {} * {}", self.0, rhs.0))
    }

    fn div(&self, rhs: &Self) -> Result<Self, String> {
        self.0
            .checked_div(rhs.0)
            .map(Self)
            .ok_or_else(|| format!("checked Decimal division failed: {} / {}", self.0, rhs.0))
    }

    fn compare(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }

    fn as_rational(&self) -> BigRational {
        parse_decimal_rational(&self.0.to_string())
            .expect("a Rust Decimal string must parse as an exact rational")
    }

    fn raw_text(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ArbitraryDecimal(pub BigDecimal);

impl Numeric for ArbitraryDecimal {
    fn parse(value: &str) -> Result<Self, String> {
        BigDecimal::from_str(value)
            .map(Self)
            .map_err(|error| error.to_string())
    }

    fn zero() -> Self {
        Self(BigDecimal::zero())
    }

    fn one() -> Self {
        Self(BigDecimal::one())
    }

    fn from_i64(value: i64) -> Self {
        Self(BigDecimal::from(value))
    }

    fn add(&self, rhs: &Self) -> Result<Self, String> {
        Ok(Self(&self.0 + &rhs.0))
    }

    fn sub(&self, rhs: &Self) -> Result<Self, String> {
        Ok(Self(&self.0 - &rhs.0))
    }

    fn mul(&self, rhs: &Self) -> Result<Self, String> {
        Ok(Self(&self.0 * &rhs.0))
    }

    fn div(&self, rhs: &Self) -> Result<Self, String> {
        if rhs.0.is_zero() {
            return Err("BigDecimal division by zero".to_owned());
        }
        Ok(Self(&self.0 / &rhs.0))
    }

    fn compare(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }

    fn as_rational(&self) -> BigRational {
        parse_decimal_rational(&self.0.to_string())
            .expect("a BigDecimal string must parse as an exact rational")
    }

    fn raw_text(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct ExactRational(pub BigRational);

impl Numeric for ExactRational {
    fn parse(value: &str) -> Result<Self, String> {
        parse_decimal_rational(value).map(Self)
    }

    fn zero() -> Self {
        Self(BigRational::zero())
    }

    fn one() -> Self {
        Self(BigRational::one())
    }

    fn from_i64(value: i64) -> Self {
        Self(BigRational::from_integer(BigInt::from(value)))
    }

    fn add(&self, rhs: &Self) -> Result<Self, String> {
        Ok(Self(&self.0 + &rhs.0))
    }

    fn sub(&self, rhs: &Self) -> Result<Self, String> {
        Ok(Self(&self.0 - &rhs.0))
    }

    fn mul(&self, rhs: &Self) -> Result<Self, String> {
        Ok(Self(&self.0 * &rhs.0))
    }

    fn div(&self, rhs: &Self) -> Result<Self, String> {
        if rhs.0.is_zero() {
            return Err("exact rational division by zero".to_owned());
        }
        Ok(Self(&self.0 / &rhs.0))
    }

    fn compare(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }

    fn as_rational(&self) -> BigRational {
        self.0.clone()
    }

    fn raw_text(&self) -> String {
        format!("{}/{}", self.0.numer(), self.0.denom())
    }
}

#[derive(Debug, Clone)]
pub struct ForecastResult<N: Numeric> {
    pub period_id: String,
    pub values: BTreeMap<String, N>,
}

#[derive(Debug, Clone)]
struct BalanceState<N: Numeric> {
    cash: N,
    accounts_receivable: N,
    inventory: N,
    gross_ppe: N,
    accumulated_depreciation: N,
    accounts_payable: N,
    interest_bearing_debt: N,
    share_capital: N,
    retained_earnings: N,
}

impl<N: Numeric> BalanceState<N> {
    fn from_json(value: &Value) -> Result<Self, String> {
        Ok(Self {
            cash: parse_field(value, "cash")?,
            accounts_receivable: parse_field(value, "accounts_receivable")?,
            inventory: parse_field(value, "inventory")?,
            gross_ppe: parse_field(value, "gross_ppe")?,
            accumulated_depreciation: parse_field(value, "accumulated_depreciation")?,
            accounts_payable: parse_field(value, "accounts_payable")?,
            interest_bearing_debt: parse_field(value, "interest_bearing_debt")?,
            share_capital: parse_field(value, "share_capital")?,
            retained_earnings: parse_field(value, "retained_earnings")?,
        })
    }

    fn net_working_capital(&self) -> Result<N, String> {
        add(
            &add(
                &self.accounts_receivable,
                &self.inventory,
                "opening working capital",
            )?,
            &negate(&self.accounts_payable)?,
            "opening net working capital",
        )
    }
}

fn parse_field<N: Numeric>(object: &Value, field: &str) -> Result<N, String> {
    let text = object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing decimal string field: {field}"))?;
    N::parse(text).map_err(|error| format!("{field}: {error}"))
}

fn add<N: Numeric>(a: &N, b: &N, context: &str) -> Result<N, String> {
    a.add(b).map_err(|error| format!("{context}: {error}"))
}

fn sub<N: Numeric>(a: &N, b: &N, context: &str) -> Result<N, String> {
    a.sub(b).map_err(|error| format!("{context}: {error}"))
}

fn mul<N: Numeric>(a: &N, b: &N, context: &str) -> Result<N, String> {
    a.mul(b).map_err(|error| format!("{context}: {error}"))
}

fn div<N: Numeric>(a: &N, b: &N, context: &str) -> Result<N, String> {
    a.div(b).map_err(|error| format!("{context}: {error}"))
}

fn negate<N: Numeric>(value: &N) -> Result<N, String> {
    sub(&N::zero(), value, "negation")
}

fn min<N: Numeric>(left: N, right: N) -> N {
    if left.compare(&right) == Ordering::Less {
        left
    } else {
        right
    }
}

fn max_zero<N: Numeric>(value: N) -> N {
    if value.compare(&N::zero()) == Ordering::Less {
        N::zero()
    } else {
        value
    }
}

pub fn load_case_a() -> Value {
    serde_json::from_str(CASE_A).expect("canonical Case A fixture must remain valid JSON")
}

pub fn stress_case_a() -> Value {
    let mut value = load_case_a();
    scale_money_fields(&mut value, 11);

    for period in value["forecast_periods"]
        .as_array_mut()
        .expect("forecast_periods must be an array")
    {
        let assumptions = period
            .get_mut("assumptions")
            .expect("forecast assumptions must exist");
        assumptions["revenue_growth"] = Value::String("1".to_owned());
        assumptions["cogs_pct_revenue"] = Value::String("5".to_owned());
        assumptions["operating_expenses_pct_revenue"] = Value::String("5".to_owned());
        assumptions["dso"] = Value::String("3650.0000".to_owned());
        assumptions["dio"] = Value::String("3650.0000".to_owned());
        assumptions["dpo"] = Value::String("3650.0000".to_owned());
        assumptions["capex_pct_revenue"] = Value::String("5".to_owned());
        assumptions["depreciation_rate"] = Value::String("1".to_owned());
        assumptions["interest_rate"] = Value::String("1".to_owned());
        assumptions["tax_rate"] = Value::String("1".to_owned());
        assumptions["dividend_payout_ratio"] = Value::String("1".to_owned());
    }

    value
}

fn scale_money_fields(value: &mut Value, decimal_places: usize) {
    match value {
        Value::Object(object) => {
            for (key, child) in object.iter_mut() {
                if MONEY_FIELDS.contains(&key.as_str()) {
                    if let Value::String(amount) = child {
                        *amount = shift_decimal_right(amount, decimal_places);
                    }
                } else {
                    scale_money_fields(child, decimal_places);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                scale_money_fields(item, decimal_places);
            }
        }
        _ => {}
    }
}

fn shift_decimal_right(value: &str, places: usize) -> String {
    let (negative, unsigned) = value
        .strip_prefix('-')
        .map(|rest| (true, rest))
        .unwrap_or((false, value));
    let (integer, fractional) = unsigned.split_once('.').unwrap_or((unsigned, ""));
    let mut digits = format!("{integer}{fractional}");
    let point = integer.len() + places;

    if point >= digits.len() {
        digits.push_str(&"0".repeat(point - digits.len()));
        let integer_part = digits.trim_start_matches('0');
        let integer_part = if integer_part.is_empty() { "0" } else { integer_part };
        return format!("{}{}", if negative { "-" } else { "" }, integer_part);
    }

    let raw_integer = &digits[..point];
    let raw_fractional = &digits[point..];
    let integer_part = raw_integer.trim_start_matches('0');
    let integer_part = if integer_part.is_empty() { "0" } else { integer_part };
    format!(
        "{}{}.{}",
        if negative { "-" } else { "" },
        integer_part,
        raw_fractional
    )
}

pub fn forecast<N: Numeric>(input: &Value) -> Result<Vec<ForecastResult<N>>, String> {
    let historical = input
        .get("historical_periods")
        .and_then(Value::as_array)
        .ok_or_else(|| "historical_periods must be an array".to_owned())?;
    let last_historical = historical
        .last()
        .ok_or_else(|| "historical_periods is empty".to_owned())?;
    let previous_balance_sheet = last_historical
        .get("balance_sheet")
        .ok_or_else(|| "last historical balance_sheet is missing".to_owned())?;
    let previous_income_statement = last_historical
        .get("income_statement")
        .ok_or_else(|| "last historical income_statement is missing".to_owned())?;

    let mut previous = BalanceState::<N>::from_json(previous_balance_sheet)?;
    let mut previous_revenue = parse_field(previous_income_statement, "revenue")?;
    let mut previous_nwc = previous.net_working_capital()?;
    let forecast_periods = input
        .get("forecast_periods")
        .and_then(Value::as_array)
        .ok_or_else(|| "forecast_periods must be an array".to_owned())?;
    let mut outputs = Vec::with_capacity(forecast_periods.len());

    for period in forecast_periods {
        let period_id = period
            .get("period_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "forecast period_id is missing".to_owned())?
            .to_owned();
        let assumptions = period
            .get("assumptions")
            .ok_or_else(|| format!("{period_id}: assumptions are missing"))?;

        let revenue_growth = parse_field(assumptions, "revenue_growth")?;
        let cogs_ratio = parse_field(assumptions, "cogs_pct_revenue")?;
        let operating_expenses_ratio =
            parse_field(assumptions, "operating_expenses_pct_revenue")?;
        let dso = parse_field(assumptions, "dso")?;
        let dio = parse_field(assumptions, "dio")?;
        let dpo = parse_field(assumptions, "dpo")?;
        let capex_ratio = parse_field(assumptions, "capex_pct_revenue")?;
        let depreciation_rate = parse_field(assumptions, "depreciation_rate")?;
        let debt_drawdowns = parse_field(assumptions, "debt_drawdowns")?;
        let principal_repayments = parse_field(assumptions, "principal_repayments")?;
        let interest_rate = parse_field(assumptions, "interest_rate")?;
        let tax_rate = parse_field(assumptions, "tax_rate")?;
        let dividend_payout_ratio = parse_field(assumptions, "dividend_payout_ratio")?;

        let one_plus_growth = add(
            &N::one(),
            &revenue_growth,
            &format!("{period_id}: 1 + revenue growth"),
        )?;
        let revenue = mul(
            &previous_revenue,
            &one_plus_growth,
            &format!("{period_id}: revenue"),
        )?;
        let cogs = mul(&revenue, &cogs_ratio, &format!("{period_id}: COGS"))?;
        let operating_expenses = mul(
            &revenue,
            &operating_expenses_ratio,
            &format!("{period_id}: operating expenses"),
        )?;
        let capex = mul(
            &revenue,
            &capex_ratio,
            &format!("{period_id}: CapEx"),
        )?;

        let gross_ppe = add(
            &previous.gross_ppe,
            &capex,
            &format!("{period_id}: gross PP&E"),
        )?;
        let average_gross_ppe = div(
            &add(
                &previous.gross_ppe,
                &gross_ppe,
                &format!("{period_id}: average gross PP&E numerator"),
            )?,
            &N::from_i64(2),
            &format!("{period_id}: average gross PP&E"),
        )?;
        let opening_net_ppe = sub(
            &previous.gross_ppe,
            &previous.accumulated_depreciation,
            &format!("{period_id}: opening net PP&E"),
        )?;
        let depreciation_by_rate = mul(
            &average_gross_ppe,
            &depreciation_rate,
            &format!("{period_id}: depreciation by rate"),
        )?;
        let depreciation_capacity = add(
            &opening_net_ppe,
            &capex,
            &format!("{period_id}: depreciation capacity"),
        )?;
        let depreciation = min(depreciation_by_rate, depreciation_capacity);
        let accumulated_depreciation = add(
            &previous.accumulated_depreciation,
            &depreciation,
            &format!("{period_id}: accumulated depreciation"),
        )?;
        let net_ppe = sub(
            &gross_ppe,
            &accumulated_depreciation,
            &format!("{period_id}: net PP&E"),
        )?;

        let debt_after_drawdown = add(
            &previous.interest_bearing_debt,
            &debt_drawdowns,
            &format!("{period_id}: debt after drawdown"),
        )?;
        let closing_debt = sub(
            &debt_after_drawdown,
            &principal_repayments,
            &format!("{period_id}: closing debt"),
        )?;
        let average_debt = div(
            &add(
                &previous.interest_bearing_debt,
                &closing_debt,
                &format!("{period_id}: average debt numerator"),
            )?,
            &N::from_i64(2),
            &format!("{period_id}: average debt"),
        )?;
        let interest_expense = mul(
            &average_debt,
            &interest_rate,
            &format!("{period_id}: interest expense"),
        )?;

        let gross_profit = sub(&revenue, &cogs, &format!("{period_id}: gross profit"))?;
        let ebitda = sub(
            &gross_profit,
            &operating_expenses,
            &format!("{period_id}: EBITDA"),
        )?;
        let ebit = sub(&ebitda, &depreciation, &format!("{period_id}: EBIT"))?;
        let ebt = sub(&ebit, &interest_expense, &format!("{period_id}: EBT"))?;
        let tax_before_floor = mul(&ebt, &tax_rate, &format!("{period_id}: tax"))?;
        let income_tax_expense = max_zero(tax_before_floor);
        let net_income = sub(
            &ebt,
            &income_tax_expense,
            &format!("{period_id}: net income"),
        )?;

        let accounts_receivable = div(
            &mul(&revenue, &dso, &format!("{period_id}: AR numerator"))?,
            &N::from_i64(365),
            &format!("{period_id}: accounts receivable"),
        )?;
        let inventory = div(
            &mul(&cogs, &dio, &format!("{period_id}: inventory numerator"))?,
            &N::from_i64(365),
            &format!("{period_id}: inventory"),
        )?;
        let accounts_payable = div(
            &mul(&cogs, &dpo, &format!("{period_id}: AP numerator"))?,
            &N::from_i64(365),
            &format!("{period_id}: accounts payable"),
        )?;
        let net_working_capital = sub(
            &add(
                &accounts_receivable,
                &inventory,
                &format!("{period_id}: working capital assets"),
            )?,
            &accounts_payable,
            &format!("{period_id}: net working capital"),
        )?;
        let change_in_nwc = sub(
            &net_working_capital,
            &previous_nwc,
            &format!("{period_id}: change in NWC"),
        )?;

        let positive_net_income = max_zero(net_income.clone());
        let dividends = mul(
            &positive_net_income,
            &dividend_payout_ratio,
            &format!("{period_id}: dividends"),
        )?;
        let cfo = sub(
            &add(
                &net_income,
                &depreciation,
                &format!("{period_id}: CFO before working capital"),
            )?,
            &change_in_nwc,
            &format!("{period_id}: CFO"),
        )?;
        let cfi = negate(&capex)?;
        let cff = sub(
            &sub(
                &debt_drawdowns,
                &principal_repayments,
                &format!("{period_id}: financing before dividends"),
            )?,
            &dividends,
            &format!("{period_id}: CFF"),
        )?;
        let net_change_in_cash = add(
            &add(&cfo, &cfi, &format!("{period_id}: CFO + CFI"))?,
            &cff,
            &format!("{period_id}: net change in cash"),
        )?;
        let closing_cash = add(
            &previous.cash,
            &net_change_in_cash,
            &format!("{period_id}: closing cash"),
        )?;
        let retained_earnings = sub(
            &add(
                &previous.retained_earnings,
                &net_income,
                &format!("{period_id}: retained earnings before dividends"),
            )?,
            &dividends,
            &format!("{period_id}: retained earnings"),
        )?;
        let total_assets = add(
            &add(
                &add(&closing_cash, &accounts_receivable, &format!("{period_id}: assets 1"))?,
                &inventory,
                &format!("{period_id}: assets 2"),
            )?,
            &net_ppe,
            &format!("{period_id}: total assets"),
        )?;
        let total_liabilities_equity = add(
            &add(
                &add(&accounts_payable, &closing_debt, &format!("{period_id}: liabilities 1"))?,
                &previous.share_capital,
                &format!("{period_id}: liabilities 2"),
            )?,
            &retained_earnings,
            &format!("{period_id}: liabilities and equity"),
        )?;
        let balance_difference = sub(
            &total_assets,
            &total_liabilities_equity,
            &format!("{period_id}: Balance Sheet difference"),
        )?;

        let mut values = BTreeMap::new();
        macro_rules! record {
            ($name:literal, $value:expr) => {
                values.insert($name.to_owned(), $value);
            };
        }
        record!("revenue", revenue.clone());
        record!("cogs", cogs.clone());
        record!("gross_profit", gross_profit);
        record!("operating_expenses", operating_expenses.clone());
        record!("ebitda", ebitda);
        record!("depreciation", depreciation.clone());
        record!("ebit", ebit);
        record!("interest_expense", interest_expense);
        record!("ebt", ebt);
        record!("income_tax_expense", income_tax_expense);
        record!("net_income", net_income.clone());
        record!("accounts_receivable", accounts_receivable.clone());
        record!("inventory", inventory.clone());
        record!("accounts_payable", accounts_payable.clone());
        record!("net_working_capital", net_working_capital.clone());
        record!("change_in_nwc", change_in_nwc);
        record!("capex", capex);
        record!("gross_ppe", gross_ppe);
        record!("accumulated_depreciation", accumulated_depreciation);
        record!("net_ppe", net_ppe);
        record!("debt_drawdowns", debt_drawdowns);
        record!("principal_repayments", principal_repayments);
        record!("closing_debt", closing_debt.clone());
        record!("dividends", dividends);
        record!("cash_from_operations", cfo);
        record!("cash_from_investing", cfi);
        record!("cash_from_financing", cff);
        record!("net_change_in_cash", net_change_in_cash);
        record!("closing_cash", closing_cash.clone());
        record!("total_assets", total_assets);
        record!("total_liabilities_equity", total_liabilities_equity);
        record!("balance_difference", balance_difference);

        outputs.push(ForecastResult { period_id, values });

        previous = BalanceState {
            cash: closing_cash,
            accounts_receivable,
            inventory,
            gross_ppe,
            accumulated_depreciation,
            accounts_payable,
            interest_bearing_debt: closing_debt,
            share_capital: previous.share_capital,
            retained_earnings,
        };
        previous_revenue = revenue;
        previous_nwc = net_working_capital;
    }

    Ok(outputs)
}

pub fn parse_decimal_rational(value: &str) -> Result<BigRational, String> {
    let (mantissa, exponent) = match value.find(['e', 'E']) {
        Some(index) => {
            let exponent = value[index + 1..]
                .parse::<i32>()
                .map_err(|error| format!("invalid decimal exponent: {error}"))?;
            (&value[..index], exponent)
        }
        None => (value, 0),
    };

    let (negative, unsigned) = mantissa
        .strip_prefix('-')
        .map(|rest| (true, rest))
        .unwrap_or((false, mantissa));
    let (integer, fractional) = unsigned.split_once('.').unwrap_or((unsigned, ""));

    if integer.is_empty()
        || !integer.bytes().all(|byte| byte.is_ascii_digit())
        || !fractional.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(format!("invalid decimal mantissa: {value}"));
    }

    let digits = format!("{integer}{fractional}");
    let mut numerator = BigInt::parse_bytes(digits.as_bytes(), 10)
        .ok_or_else(|| format!("invalid decimal digits: {value}"))?;
    if negative {
        numerator = -numerator;
    }

    let scale = i32::try_from(fractional.len())
        .map_err(|error| format!("decimal scale is too large: {error}"))?
        - exponent;

    if scale >= 0 {
        let denominator = BigInt::from(10u8).pow(scale as u32);
        Ok(BigRational::new(numerator, denominator))
    } else {
        let multiplier = BigInt::from(10u8).pow(scale.unsigned_abs());
        Ok(BigRational::from_integer(numerator * multiplier))
    }
}

pub fn round_half_even(value: &BigRational, scale: u32) -> String {
    let factor = BigInt::from(10u8).pow(scale);
    let scaled_numerator = value.numer().abs() * &factor;
    let denominator = value.denom();
    let (mut quotient, remainder) = scaled_numerator.div_rem(denominator);
    let twice_remainder = &remainder * 2;

    if twice_remainder > *denominator
        || (twice_remainder == *denominator && quotient.is_odd())
    {
        quotient += 1;
    }

    let is_negative = value.is_negative() && !quotient.is_zero();
    let digits = quotient.to_str_radix(10);
    let sign = if is_negative { "-" } else { "" };

    if scale == 0 {
        return format!("{sign}{digits}");
    }

    let scale = scale as usize;
    let padded = if digits.len() <= scale {
        format!("{}{}", "0".repeat(scale + 1 - digits.len()), digits)
    } else {
        digits
    };
    let split_at = padded.len() - scale;
    format!("{sign}{}.{}", &padded[..split_at], &padded[split_at..])
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonSummary {
    pub candidate: String,
    pub compared_values: usize,
    pub display_mismatches: usize,
    pub maximum_absolute_error: String,
    pub maximum_error_period: String,
    pub maximum_error_metric: String,
}

pub fn compare_to_reference<N: Numeric>(
    candidate_name: &str,
    candidate: &[ForecastResult<N>],
    reference: &[ForecastResult<ExactRational>],
    display_scale: u32,
) -> Result<ComparisonSummary, String> {
    if candidate.len() != reference.len() {
        return Err(format!(
            "period count differs: {} vs {}",
            candidate.len(),
            reference.len()
        ));
    }

    let mut compared_values = 0;
    let mut display_mismatches = 0;
    let mut maximum_error = BigRational::zero();
    let mut maximum_error_period = String::new();
    let mut maximum_error_metric = String::new();

    for (candidate_period, reference_period) in candidate.iter().zip(reference) {
        if candidate_period.period_id != reference_period.period_id {
            return Err(format!(
                "period id differs: {} vs {}",
                candidate_period.period_id, reference_period.period_id
            ));
        }

        for (metric, candidate_value) in &candidate_period.values {
            let reference_value = reference_period
                .values
                .get(metric)
                .ok_or_else(|| format!("reference is missing metric {metric}"))?;
            let candidate_rational = candidate_value.as_rational();
            let reference_rational = reference_value.as_rational();

            if round_half_even(&candidate_rational, display_scale)
                != round_half_even(&reference_rational, display_scale)
            {
                display_mismatches += 1;
            }

            let error = (candidate_rational - reference_rational).abs();
            if error > maximum_error {
                maximum_error = error;
                maximum_error_period = candidate_period.period_id.clone();
                maximum_error_metric = metric.clone();
            }
            compared_values += 1;
        }
    }

    Ok(ComparisonSummary {
        candidate: candidate_name.to_owned(),
        compared_values,
        display_mismatches,
        maximum_absolute_error: round_half_even(&maximum_error, 12),
        maximum_error_period,
        maximum_error_metric,
    })
}

pub fn balance_sheet_differences_are_zero(
    forecast: &[ForecastResult<ExactRational>],
) -> bool {
    forecast.iter().all(|period| {
        period
            .values
            .get("balance_difference")
            .is_some_and(|value| value.0.is_zero())
    })
}

pub fn maximum_absolute_value<N: Numeric>(
    forecast: &[ForecastResult<N>],
) -> Option<(String, String, String)> {
    let mut maximum = BigRational::zero();
    let mut maximum_period = String::new();
    let mut maximum_metric = String::new();

    for period in forecast {
        for (metric, value) in &period.values {
            let absolute = value.as_rational().abs();
            if absolute > maximum {
                maximum = absolute;
                maximum_period = period.period_id.clone();
                maximum_metric = metric.clone();
            }
        }
    }

    if maximum_metric.is_empty() {
        None
    } else {
        Some((
            round_half_even(&maximum, 2),
            maximum_period,
            maximum_metric,
        ))
    }
}
