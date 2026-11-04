#![forbid(unsafe_code)]

//! Isolated Decimal and JSON/CSV contract proof of concept for OFRL-IB-001.
//!
//! This crate tests numeric representation and interchange only. It is not
//! the financial calculation engine and contains no three-statement logic.

use core::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Current version of this proof-of-concept interchange record.
pub const SCHEMA_VERSION: &str = "0.1.0";

/// Display unit scale attached to an amount.
///
/// The amount itself is expressed in the named unit. For example, when
/// unit_scale is Thousands, an amount of 125.00 represents 125 thousand
/// units of the declared currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitScale {
    /// Amount is expressed in single currency units.
    Units,
    /// Amount is expressed in thousands of currency units.
    Thousands,
    /// Amount is expressed in millions of currency units.
    Millions,
}

/// A minimal, versioned amount record for JSON and CSV interchange.
///
/// Decimal values are serialized as strings. Deserialization rejects JSON
/// number tokens, exponent notation, whitespace, separators, and other
/// non-canonical decimal text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmountRecord {
    /// Interchange schema version.
    pub schema_version: String,
    /// Stable model or project identifier.
    pub model_id: String,
    /// Fiscal or reporting period identifier.
    pub period: String,
    /// Stable financial line-item identifier.
    pub line_item: String,
    /// Three-letter uppercase currency code.
    pub currency: String,
    /// Unit scale used by the amount.
    pub unit_scale: UnitScale,
    /// Canonical decimal amount, represented as a JSON/CSV string.
    #[serde(
        serialize_with = "serialize_decimal",
        deserialize_with = "deserialize_decimal"
    )]
    pub amount: Decimal,
}

impl AmountRecord {
    /// Create an amount record using the current POC schema version.
    pub fn new(
        model_id: impl Into<String>,
        period: impl Into<String>,
        line_item: impl Into<String>,
        currency: impl Into<String>,
        unit_scale: UnitScale,
        amount: Decimal,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.to_owned(),
            model_id: model_id.into(),
            period: period.into(),
            line_item: line_item.into(),
            currency: currency.into(),
            unit_scale,
            amount,
        }
    }
}

fn serialize_decimal<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Use rust_decimal's official string adapter, preserving Decimal scale.
    rust_decimal::serde::str::serialize(value, serializer)
}

fn deserialize_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserializing through String deliberately rejects JSON numeric tokens.
    let text = String::deserialize(deserializer)?;

    if !is_plain_decimal_text(&text) {
        return Err(serde::de::Error::custom(
            "amount must use canonical base-10 string notation",
        ));
    }

    Decimal::from_str(&text).map_err(serde::de::Error::custom)
}

/// Check the canonical lexical grammar before Decimal parsing.
///
/// Accepted examples: 0, -12, 123.4500, -0.005.
/// Rejected examples: +1, 01, 1., .5, 1e3, 1,000, and whitespace.
fn is_plain_decimal_text(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.is_empty() {
        return false;
    }

    let mut index = if bytes[0] == b'-' { 1 } else { 0 };
    if index >= bytes.len() {
        return false;
    }

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
        return true;
    }

    if bytes[index] != b'.' {
        return false;
    }

    index += 1;
    let fractional_start = index;

    while index < bytes.len() && bytes[index].is_ascii_digit() {
        index += 1;
    }

    index > fractional_start && index == bytes.len()
}

#[cfg(test)]
mod tests {
    use super::{AmountRecord, UnitScale};
    use core::str::FromStr;
    use rust_decimal::Decimal;

    fn sample_record(amount: &str) -> AmountRecord {
        AmountRecord::new(
            "OFRL-IB-001",
            "FY2026",
            "revenue",
            "USD",
            UnitScale::Thousands,
            Decimal::from_str(amount).expect("test amount must fit Decimal"),
        )
    }

    #[test]
    fn json_serializes_decimal_as_string_and_preserves_scale() {
        let record = sample_record("123456789012345.678900");

        let json = serde_json::to_string(&record).expect("JSON serialization should succeed");
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("serialized JSON should be valid");

        assert_eq!(
            value["amount"],
            serde_json::Value::String("123456789012345.678900".to_owned())
        );

        let decoded: AmountRecord =
            serde_json::from_str(&json).expect("decimal string should deserialize");
        assert_eq!(decoded.amount.to_string(), "123456789012345.678900");
        assert_eq!(decoded.amount.scale(), 6);
        assert_eq!(decoded, record);
    }

    #[test]
    fn json_numeric_token_is_rejected() {
        let json = r#"{
            "schema_version":"0.1.0",
            "model_id":"OFRL-IB-001",
            "period":"FY2026",
            "line_item":"revenue",
            "currency":"USD",
            "unit_scale":"thousands",
            "amount":123456.78
        }"#;

        assert!(
            serde_json::from_str::<AmountRecord>(json).is_err(),
            "JSON numeric tokens must not be accepted for Decimal fields"
        );
    }

    #[test]
    fn non_canonical_decimal_strings_are_rejected() {
        for invalid in [
            "", "+1", "01", "-01", ".5", "1.", "1e3", "1E3", "1,000", " 1.0", "1.0 ", "NaN",
            "Infinity", "--1",
        ] {
            let json = format!(
                r#"{{"schema_version":"0.1.0","model_id":"OFRL-IB-001","period":"FY2026","line_item":"revenue","currency":"USD","unit_scale":"thousands","amount":"{invalid}"}}"#
            );

            assert!(
                serde_json::from_str::<AmountRecord>(&json).is_err(),
                "non-canonical amount {invalid:?} should be rejected"
            );
        }
    }

    #[test]
    fn negative_amounts_round_trip_without_sign_or_scale_loss() {
        let record = sample_record("-9876.5432100");
        let json = serde_json::to_string(&record).expect("negative amount should serialize");
        assert!(json.contains(r#""amount":"-9876.5432100""#));

        let decoded: AmountRecord =
            serde_json::from_str(&json).expect("negative decimal string should deserialize");
        assert_eq!(decoded.amount.to_string(), "-9876.5432100");
        assert_eq!(decoded.amount.scale(), 7);
    }

    #[test]
    fn csv_round_trip_preserves_decimal_text_and_metadata() {
        let record = sample_record("9876543210.120000");

        let mut writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(Vec::new());

        writer
            .serialize(&record)
            .expect("CSV serialization should succeed");
        let bytes = writer
            .into_inner()
            .expect("CSV writer should flush successfully");
        let csv_text = String::from_utf8(bytes).expect("CSV output should be UTF-8");

        assert!(
            csv_text.contains("9876543210.120000"),
            "CSV should preserve the decimal text and trailing zeros"
        );

        let mut reader = csv::ReaderBuilder::new().from_reader(csv_text.as_bytes());
        let decoded = reader
            .deserialize::<AmountRecord>()
            .next()
            .expect("CSV should contain one data row")
            .expect("CSV row should deserialize");

        assert_eq!(decoded, record);
        assert_eq!(decoded.amount.to_string(), "9876543210.120000");
        assert_eq!(decoded.amount.scale(), 6);
    }

    #[test]
    fn decimal_limits_and_checked_arithmetic_are_explicit() {
        assert!(Decimal::MAX.checked_add(Decimal::ONE).is_none());
        assert!(Decimal::MIN.checked_sub(Decimal::ONE).is_none());
        assert!(Decimal::MAX.checked_mul(Decimal::from(2u32)).is_none());
        assert!(Decimal::ONE.checked_div(Decimal::ZERO).is_none());

        let max_round_trip = sample_record(&Decimal::MAX.to_string());
        let json =
            serde_json::to_string(&max_round_trip).expect("maximum Decimal should serialize");
        let decoded: AmountRecord =
            serde_json::from_str(&json).expect("maximum Decimal string should deserialize");
        assert_eq!(decoded.amount, Decimal::MAX);
    }

    #[test]
    fn division_scale_and_non_terminating_results_are_observable() {
        let one_over_365 = Decimal::ONE
            .checked_div(Decimal::from(365u32))
            .expect("1 divided by 365 should be within Decimal range");
        let one_over_three = Decimal::ONE
            .checked_div(Decimal::from(3u32))
            .expect("1 divided by 3 should be within Decimal range");

        assert!(one_over_365.scale() <= Decimal::MAX_SCALE);
        assert!(one_over_three.scale() <= Decimal::MAX_SCALE);

        let reconstructed = one_over_three
            .checked_mul(Decimal::from(3u32))
            .expect("multiplication should remain within Decimal range");

        // Decimal is finite precision; a repeating quotient must not be
        // represented or documented as exact rational arithmetic.
        assert_ne!(reconstructed, Decimal::ONE);

        println!(
            "POC decimal 1/365 = {one_over_365} (scale {})",
            one_over_365.scale()
        );
        println!(
            "POC decimal 1/3   = {one_over_three} (scale {})",
            one_over_three.scale()
        );
        println!("POC decimal (1/3)*3 = {reconstructed}");
    }

    #[test]
    fn values_beyond_supported_scale_or_range_never_silently_change() {
        let beyond_scale = "0.00000000000000000000000000001";
        if let Ok(parsed) = Decimal::from_str(beyond_scale) {
            assert_eq!(
                parsed.to_string(),
                beyond_scale,
                "a parsed value must not silently round or truncate the source text"
            );
        }

        let beyond_range = r#"{
            "schema_version":"0.1.0",
            "model_id":"OFRL-IB-001",
            "period":"FY2026",
            "line_item":"revenue",
            "currency":"USD",
            "unit_scale":"thousands",
            "amount":"999999999999999999999999999999"
        }"#;

        assert!(
            serde_json::from_str::<AmountRecord>(beyond_range).is_err(),
            "amounts outside the selected Decimal range must be rejected"
        );
    }
}
