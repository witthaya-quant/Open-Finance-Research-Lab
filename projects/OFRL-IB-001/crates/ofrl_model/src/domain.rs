use core::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModelInput {
    pub schema_version: String,
    pub model: ModelMetadata,
    pub opening_balance_sheet: OpeningBalanceSheet,
    pub historical_periods: Vec<HistoricalPeriod>,
    pub forecast_periods: Vec<ForecastPeriod>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ModelMetadata {
    pub model_id: String,
    pub model_name: String,
    pub reporting_currency: String,
    pub unit_scale: UnitScale,
    pub day_count_basis: u16,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum UnitScale {
    Units,
    Thousands,
    Millions,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OpeningBalanceSheet {
    pub as_of_date: String,
    pub balances: BalanceSheet,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HistoricalPeriod {
    pub period_id: String,
    pub start_date: String,
    pub end_date: String,
    pub source: HistoricalSource,
    pub income_statement: IncomeStatement,
    pub balance_sheet: BalanceSheet,
    pub cash_flow_statement: CashFlowStatement,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct HistoricalSource {
    pub classification: SourceClassification,
    pub source_reference: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) enum SourceClassification {
    #[serde(rename = "synthetic")]
    Synthetic,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IncomeStatement {
    #[serde(deserialize_with = "deserialize_decimal")]
    pub revenue: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub cogs: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub operating_expenses: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub depreciation: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub interest_expense: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub income_tax_expense: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub net_income: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BalanceSheet {
    #[serde(deserialize_with = "deserialize_decimal")]
    pub cash: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub accounts_receivable: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub inventory: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub gross_ppe: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub accumulated_depreciation: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub accounts_payable: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub interest_bearing_debt: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub share_capital: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub retained_earnings: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CashFlowStatement {
    #[serde(deserialize_with = "deserialize_decimal")]
    pub net_income: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub depreciation_addback: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub change_in_operating_nwc: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub cash_from_operations: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub capex: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub cash_from_investing: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub debt_drawdowns: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub principal_repayments: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub dividends: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub cash_from_financing: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub net_change_in_cash: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub opening_cash: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub closing_cash: Decimal,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ForecastPeriod {
    pub period_id: String,
    pub start_date: String,
    pub end_date: String,
    pub assumptions: ForecastAssumptions,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ForecastAssumptions {
    #[serde(deserialize_with = "deserialize_decimal")]
    pub revenue_growth: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub cogs_pct_revenue: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub operating_expenses_pct_revenue: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub dso: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub dio: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub dpo: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub capex_pct_revenue: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub depreciation_rate: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub debt_drawdowns: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub principal_repayments: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub interest_rate: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub tax_rate: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub dividend_payout_ratio: Decimal,
}

// Strict decimal-string deserialization is repeated here intentionally: the
// isolated POC remains self-contained, while the production model owns its
// application boundary and its own regression tests.
pub(crate) fn deserialize_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let text = String::deserialize(deserializer).map_err(|_| {
        serde::de::Error::custom(
            "DECIMAL_STRING_REQUIRED: Decimal values must be JSON string tokens",
        )
    })?;

    if !is_plain_decimal_text(&text) {
        return Err(serde::de::Error::custom(
            "DECIMAL_TEXT_NON_CANONICAL: amount must use canonical base-10 notation",
        ));
    }

    if let Some((_, fractional)) = text.split_once('.') {
        if fractional.len() > Decimal::MAX_SCALE as usize {
            return Err(serde::de::Error::custom(
                "DECIMAL_SCALE_EXCEEDED: fractional scale exceeds the selected Decimal maximum",
            ));
        }
    }

    let parsed = Decimal::from_str(&text).map_err(|_| {
        serde::de::Error::custom("DECIMAL_OUT_OF_RANGE: amount is outside the Decimal range")
    })?;

    if parsed.to_string() != text {
        return Err(serde::de::Error::custom(
            "DECIMAL_OUT_OF_RANGE: amount is not exactly representable by the selected Decimal type",
        ));
    }

    Ok(parsed)
}

fn is_plain_decimal_text(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    let is_negative = bytes[0] == b'-';
    let mut index = if is_negative { 1 } else { 0 };
    if index >= bytes.len() {
        return false;
    }

    let integer_is_zero = bytes[index] == b'0';
    match bytes[index] {
        b'0' => {
            index += 1;
            if index < bytes.len() && bytes[index].is_ascii_digit() {
                return false;
            }
        }
        b'1'..=b'9' => {
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_digit() {
                index += 1;
            }
        }
        _ => return false,
    }

    if index == bytes.len() {
        return !(is_negative && integer_is_zero);
    }

    if bytes[index] != b'.' {
        return false;
    }

    index += 1;
    let fractional_start = index;
    while index < bytes.len() && bytes[index].is_ascii_digit() {
        index += 1;
    }

    let fractional_is_nonzero = bytes[fractional_start..index]
        .iter()
        .any(|digit| *digit != b'0');

    index > fractional_start
        && index == bytes.len()
        && !(is_negative && integer_is_zero && !fractional_is_nonzero)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CalendarDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl CalendarDate {
    pub fn parse(value: &str) -> Option<Self> {
        let bytes = value.as_bytes();
        if bytes.len() != 10
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || !bytes[..4].iter().all(u8::is_ascii_digit)
            || !bytes[5..7].iter().all(u8::is_ascii_digit)
            || !bytes[8..].iter().all(u8::is_ascii_digit)
        {
            return None;
        }

        let year = value[0..4].parse::<u16>().ok()?;
        let month = value[5..7].parse::<u8>().ok()?;
        let day = value[8..10].parse::<u8>().ok()?;

        if year == 0 || !(1..=12).contains(&month) {
            return None;
        }

        let max_day = days_in_month(year, month);
        if day == 0 || day > max_day {
            return None;
        }

        Some(Self { year, month, day })
    }

    pub fn ordinal(self) -> i64 {
        let completed_years = i64::from(self.year) - 1;
        let leap_days = completed_years / 4 - completed_years / 100 + completed_years / 400;
        let days_before_year = completed_years * 365 + leap_days;
        let mut day_of_year = i64::from(self.day);

        for month in 1..self.month {
            day_of_year += i64::from(days_in_month(self.year, month));
        }

        days_before_year + day_of_year
    }

    pub fn next_day(self) -> Option<Self> {
        let month_length = days_in_month(self.year, self.month);
        if self.day < month_length {
            return Some(Self {
                day: self.day + 1,
                ..self
            });
        }

        if self.month < 12 {
            return Some(Self {
                month: self.month + 1,
                day: 1,
                ..self
            });
        }

        if self.year == 9999 {
            return None;
        }

        Some(Self {
            year: self.year + 1,
            month: 1,
            day: 1,
        })
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

impl core::fmt::Display for CalendarDate {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        )
    }
}
