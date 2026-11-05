use ofrl_model::{validate_input_json, Diagnostic, DiagnosticCode, InputStatus, ValidationReport};
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

    let report = validate_value(value);
    let diagnostic = first(&report);
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

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::RequiredFieldMissing);
    ptr(diagnostic, "/historical_periods/0/income_statement/revenue");
    assert_eq!(diagnostic.period_id.as_deref(), Some("FY2023"));
}

#[test]
fn numeric_object_key_is_not_treated_as_array_index() {
    let mut value = case_a();
    value["model"]
        .as_object_mut()
        .unwrap()
        .insert("123".into(), json!("x"));

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::UnknownField);
    ptr(diagnostic, "/model/123");
    assert_eq!(diagnostic.field.as_deref(), Some("123"));
}

#[test]
fn invalid_enum_reports_pointer() {
    let mut value = case_a();
    value["model"]["unit_scale"] = json!("billions");

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::InvalidEnumValue);
    ptr(diagnostic, "/model/unit_scale");
}

#[test]
fn array_element_type_error_reports_index() {
    let mut value = case_a();
    value["forecast_periods"][2] = json!("not an object");

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::JsonTypeMismatch);
    ptr(diagnostic, "/forecast_periods/2");
    assert_eq!(diagnostic.field.as_deref(), Some("forecast_periods"));
}

#[test]
fn decimal_number_token_reports_exact_field() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] = json!(1000.0);

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalStringRequired);
    ptr(diagnostic, "/historical_periods/0/income_statement/revenue");
}

#[test]
fn malformed_decimal_text_reports_noncanonical_code() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] = json!("01.0");

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalTextNonCanonical);
    ptr(diagnostic, "/historical_periods/0/income_statement/revenue");
}

#[test]
fn decimal_scale_excess_reports_scale_code() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] =
        json!(format!("0.{}", "1".repeat(29)));

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalScaleExceeded);
    ptr(diagnostic, "/historical_periods/0/income_statement/revenue");
}

#[test]
fn decimal_out_of_range_reports_range_code() {
    let mut value = case_a();
    value["historical_periods"][0]["income_statement"]["revenue"] =
        json!("79228162514264337593543950336");

    let report = validate_value(value);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::DecimalOutOfRange);
    ptr(diagnostic, "/historical_periods/0/income_statement/revenue");
}


fn value_at_pointer_mut<'a>(root: &'a mut Value, pointer: &str) -> Option<&'a mut Value> {
    let mut current = root;
    if pointer.is_empty() {
        return Some(current);
    }
    if !pointer.starts_with('/') {
        return None;
    }

    for encoded in pointer[1..].split('/') {
        let segment = encoded.replace("~1", "/").replace("~0", "~");
        current = match current {
            Value::Object(object) => object.get_mut(&segment)?,
            Value::Array(array) => array.get_mut(segment.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current)
}

fn apply_fixture_mutation(root: &mut Value, case: &Value) {
    let operation = case["operation"].as_str().expect("fixture operation");
    let parent_pointer = case["parent_pointer"].as_str().expect("fixture parent pointer");
    let field = case["field"].as_str().expect("fixture field");
    let parent = value_at_pointer_mut(root, parent_pointer).expect("fixture parent must resolve");

    match (operation, parent) {
        ("insert", Value::Object(object)) => {
            assert!(
                object.insert(field.to_owned(), case["value"].clone()).is_none(),
                "fixture insert target must not already exist"
            );
        }
        ("remove", Value::Object(object)) => {
            assert!(object.remove(field).is_some(), "fixture remove target must exist");
        }
        ("set", Value::Object(object)) => {
            *object.get_mut(field).expect("fixture set target must exist") =
                case["value"].clone();
        }
        ("set", Value::Array(array)) => {
            let index = field.parse::<usize>().expect("array fixture field is an index");
            *array.get_mut(index).expect("fixture array index must exist") =
                case["value"].clone();
        }
        (other, _) => panic!("unsupported fixture operation or parent type: {other}"),
    }
}

#[test]
fn versioned_deserialization_fixtures_match_expected_diagnostics() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../../../fixtures/contracts/ofrl-ib-001-deserialization-diagnostics.v0.1.0.json"
    ))
    .expect("typed deserialization fixture manifest must be valid JSON");

    assert_eq!(fixtures["fixture_version"], "0.1.0");
    assert_eq!(
        fixtures["base_fixture"],
        "fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json"
    );

    let cases = fixtures["cases"]
        .as_array()
        .expect("fixture manifest must contain an array of cases");
    assert_eq!(cases.len(), 9);

    for case in cases {
        let case_id = case["id"].as_str().expect("fixture case id");
        let mut input = case_a();
        apply_fixture_mutation(&mut input, case);

        let report = validate_value(input);
        let diagnostic = first(&report);
        let actual_code = serde_json::to_value(diagnostic.code)
            .expect("diagnostic code serializes")
            .as_str()
            .expect("diagnostic code is a string")
            .to_owned();

        assert_eq!(
            actual_code,
            case["expected"]["code"].as_str().expect("expected code"),
            "{case_id}: diagnostic code mismatch"
        );
        assert_eq!(
            diagnostic.json_pointer.as_deref(),
            case["expected"]["json_pointer"].as_str(),
            "{case_id}: JSON Pointer mismatch"
        );
        assert_eq!(
            diagnostic.field.as_deref(),
            case["expected"]["field"].as_str(),
            "{case_id}: field context mismatch"
        );
        assert_eq!(
            diagnostic.period_id.as_deref(),
            case["expected"]["period_id"].as_str(),
            "{case_id}: period context mismatch"
        );
    }
}

#[test]
fn malformed_json_has_no_fabricated_pointer() {
    let report = validate_input_json(r#"{"schema_version":"0.1.0""#);
    let diagnostic = first(&report);
    assert_eq!(diagnostic.code, DiagnosticCode::JsonParseFailed);
    assert!(diagnostic.json_pointer.is_none());
    assert!(diagnostic.field.is_none());
}
