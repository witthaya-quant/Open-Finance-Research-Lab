use ofrl_model::{validate_input_json, DiagnosticCode, InputStatus};
use serde_json::{json, Value};

const CASE_A: &str =
    include_str!("../../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json");

fn payload() -> Value {
    serde_json::from_str(CASE_A).expect("Case A fixture must be valid JSON")
}

fn validate(value: &Value) -> ofrl_model::ValidationReport {
    validate_input_json(&value.to_string())
}

fn has_code(report: &ofrl_model::ValidationReport, code: DiagnosticCode) -> bool {
    report.diagnostics.iter().any(|diagnostic| diagnostic.code == code)
}

#[test]
fn canonical_synthetic_case_a_passes_all_historical_semantic_checks() {
    let report = validate_input_json(CASE_A);
    assert_eq!(report.input_status, InputStatus::Accepted);
    assert!(report.diagnostics.is_empty(), "{:#?}", report.diagnostics);
}

#[test]
fn json_syntax_errors_are_rejected() {
    let report = validate_input_json("{not-json");
    assert_eq!(report.input_status, InputStatus::Rejected);
    assert!(has_code(&report, DiagnosticCode::JsonParseFailed));
}

#[test]
fn unsupported_schema_version_is_rejected() {
    let mut value = payload();
    value["schema_version"] = json!("9.9.9");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::SchemaVersionUnsupported));
}

#[test]
fn decimal_numeric_tokens_are_rejected() {
    let mut value = payload();
    value["historical_periods"][0]["income_statement"]["revenue"] = json!(1000.0);
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::JsonTypeMismatch));
}

#[test]
fn non_canonical_decimal_text_is_rejected() {
    let mut value = payload();
    value["historical_periods"][0]["income_statement"]["revenue"] = json!("01.0");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::DecimalTextNonCanonical));
}

#[test]
fn excessive_decimal_scale_is_rejected_before_parsing() {
    let mut value = payload();
    value["forecast_periods"][0]["assumptions"]["dso"] =
        json!("0.12345678901234567890123456789");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::DecimalScaleExceeded));
}

#[test]
fn negative_zero_is_rejected() {
    let mut value = payload();
    value["historical_periods"][0]["income_statement"]["net_income"] = json!("-0.000");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::DecimalTextNonCanonical));
}

#[test]
fn unknown_nested_fields_are_rejected() {
    let mut value = payload();
    value["forecast_periods"][0]["assumptions"]["unapproved_driver"] = json!("1");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::UnknownField));
}

#[test]
fn missing_historical_provenance_reference_is_rejected() {
    let mut value = payload();
    value["historical_periods"][0]["source"]
        .as_object_mut()
        .expect("source object")
        .remove("source_reference");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::RequiredFieldMissing));
}

#[test]
fn invalid_gregorian_date_is_rejected_by_runtime() {
    let mut value = payload();
    value["historical_periods"][0]["end_date"] = json!("2023-02-30");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::InvalidDate));
}

#[test]
fn reporting_period_gap_is_rejected() {
    let mut value = payload();
    value["forecast_periods"][0]["start_date"] = json!("2026-01-02");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::PeriodGap));
}

#[test]
fn period_identifier_must_match_period_end_year() {
    let mut value = payload();
    value["historical_periods"][0]["period_id"] = json!("FY2024");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::PeriodIdYearMismatch));
}

#[test]
fn opening_balance_sheet_must_balance() {
    let mut value = payload();
    value["opening_balance_sheet"]["balances"]["cash"] = json!("999.0");
    let report = validate(&value);
    assert!(has_code(
        &report,
        DiagnosticCode::HistoricalBalanceSheetUnbalanced
    ));
}

#[test]
fn historical_net_income_must_reconcile_to_income_statement() {
    let mut value = payload();
    value["historical_periods"][0]["income_statement"]["net_income"] = json!("243.0");
    let report = validate(&value);
    assert!(has_code(
        &report,
        DiagnosticCode::HistoricalNetIncomeMismatch
    ));
}

#[test]
fn historical_nwc_movement_must_reconcile_to_balance_sheets() {
    let mut value = payload();
    value["historical_periods"][0]["cash_flow_statement"]["change_in_operating_nwc"] =
        json!("9.0");
    let report = validate(&value);
    assert!(has_code(
        &report,
        DiagnosticCode::HistoricalNwcRollforwardFailed
    ));
}

#[test]
fn historical_ppe_must_roll_forward_without_disposals() {
    let mut value = payload();
    value["historical_periods"][0]["balance_sheet"]["gross_ppe"] = json!("649.0");
    let report = validate(&value);
    assert!(has_code(
        &report,
        DiagnosticCode::HistoricalPpeRollforwardFailed
    ));
}

#[test]
fn historical_debt_repayment_cannot_exceed_available_debt() {
    let mut value = payload();
    value["historical_periods"][0]["cash_flow_statement"]["principal_repayments"] =
        json!("400.0");
    let report = validate(&value);
    assert!(has_code(
        &report,
        DiagnosticCode::DebtRepaymentExceedsAvailable
    ));
}

#[test]
fn forecast_debt_repayment_cannot_exceed_available_debt() {
    let mut value = payload();
    value["forecast_periods"][0]["assumptions"]["principal_repayments"] = json!("1000.0");
    let report = validate(&value);
    assert!(has_code(
        &report,
        DiagnosticCode::DebtRepaymentExceedsAvailable
    ));
}

#[test]
fn unsupported_forecast_assumption_domain_is_rejected() {
    let mut value = payload();
    value["forecast_periods"][0]["assumptions"]["tax_rate"] = json!("1.01");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::NumericDomainInvalid));
}

#[test]
fn zero_revenue_growth_floor_is_valid_and_not_rejected_as_a_bad_input() {
    let mut value = payload();
    value["forecast_periods"][0]["assumptions"]["revenue_growth"] = json!("-1.0");
    let report = validate(&value);
    assert_eq!(report.input_status, InputStatus::Accepted, "{:#?}", report.diagnostics);
}

#[test]
fn currency_must_have_three_uppercase_ascii_letters() {
    let mut value = payload();
    value["model"]["reporting_currency"] = json!("usd");
    let report = validate(&value);
    assert!(has_code(&report, DiagnosticCode::InvalidCurrencyFormat));
}
