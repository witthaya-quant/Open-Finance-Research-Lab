use ofrl_model::{
    validate_input_json, Diagnostic, DiagnosticCode, InputStatus, ValidationReport,
};
use serde_json::{json, Value};

const CASE_A: &str =
    include_str!("../../../../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json");

fn case_a() -> Value {
    serde_json::from_str(CASE_A).expect("Case A JSON")
}

fn validate_value(value: Value) -> ValidationReport {
    validate_input_json(&serde_json::to_string(&value).expect("serialize fixture"))
}

fn first(report: &ValidationReport) -> &Diagnostic {
    assert_eq!(report.input_status, InputStatus::Rejected);
    report.diagnostics.first().expect("diagnostic")
}

fn ptr(diagnostic: &Diagnostic, expected: &str) {
    assert_eq!(diagnostic.json_pointer.as_deref(), Some(expected));
}

#[test]
fn root_type_error_uses_empty_json_pointer() {
    let report = validate_input_json("42");
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::JsonTypeMismatch);
    ptr(diagnostic, "");
}

#[test]
fn nested_unknown_field_escapes_pointer_and_keeps_period_context() {
    let mut value = case_a();
    value["historical_periods"][1]["income_statement"]
        .as_object_mut()
        .unwrap()
        .insert("bad/key~".into(), json!("x"));

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::UnknownField);
    ptr(
        diagnostic,
        "/historical_periods/1/income_statement/bad~1key~0",
    );
    assert_eq!(diagnostic.field.as_deref(), Some("bad/key~"));
    assert_eq!(diagnostic.period_id.as_deref(), Some("FY2024"));
}

#[test]
fn nested_missing_field_has_exact_pointer() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]
        .as_object_mut()
        .unwrap()
        .remove("revenue");

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::RequiredFieldMissing);
    ptr(
        diagnostic,
        "/historical_periods/0/income_statement/revenue",
    );
    assert_eq!(diagnostic.period_id.as_deref(), Some("FY2023"));
}

#[test]
fn numeric_object_key_is_not_treated_as_array_index() {
    let mut value = case_a();
    value["model"]
        .as_object_mut()
        .unwrap()
        .insert("123".into(), json!("x"));

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::UnknownField);
    ptr(diagnostic, "/model/123");
    assert_eq!(diagnostic.field.as_deref(), Some("123"));
}

#[test]
fn invalid_enum_reports_pointer() {
    let mut value = case_a();
    value["model"]["unit_scale"] = json!("billions");

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::InvalidEnumValue);
    ptr(diagnostic, "/model/unit_scale");
}

#[test]
fn array_element_type_error_reports_index() {
    let mut value = case_a();
    value["forecast_periods"][2] = json!("not an object");

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::JsonTypeMismatch);
    ptr(diagnostic, "/forecast_periods/2");
    assert_eq!(diagnostic.field.as_deref(), Some("forecast_periods"));
}

#[test]
fn decimal_number_token_reports_exact_field() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] = json!(1000.0);

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalStringRequired);
    ptr(
        diagnostic,
        "/historical_periods/0/income_statement/revenue",
    );
}

#[test]
fn malformed_decimal_text_reports_noncanonical_code() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] = json!("01.0");

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalTextNonCanonical);
    ptr(
        diagnostic,
        "/historical_periods/0/income_statement/revenue",
    );
}

#[test]
fn decimal_scale_excess_reports_scale_code() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] =
        json!(format!("0.{}", "1".repeat(29)));

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalScaleExceeded);
    ptr(
        diagnostic,
        "/historical_periods/0/income_statement/revenue",
    );
}

#[test]
fn decimal_out_of_range_reports_range_code() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] =
        json!("79228162514264337593543950336");

    let diagnostic = first(&validate_value(value));
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalOutOfRange);
    ptr(
        diagnostic,
        "/historical_periods/0/income_statement/revenue",
    );
}

#[test]
fn malformed_json_has_no_fabricated_pointer() {
    let report = validate_input_json(r#"{"schema_version":"0.1.0""#);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::JsonParseFailed);
    assert!(diagnostic.json_pointer.is_none());
    assert!(diagnostic.field.is_none());
}
