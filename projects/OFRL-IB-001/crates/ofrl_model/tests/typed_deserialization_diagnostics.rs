use ofrl_model::{validate_input_json, Diagnostic, DiagnosticCode, InputStatus, ValidationReport};
use serde_json::{json, Value};

const CASE_A: &str = include_str!("../../../../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json");
fn case_a()->Value{serde_json::from_str(CASE_A).expect("Case A JSON")}
fn validate_value(value:Value)->ValidationReport{validate_input_json(&serde_json::to_string(&value).expect("serialize fixture"))}
fn first(report:&ValidationReport)->&Diagnostic{assert_eq!(report.input_status,InputStatus::Rejected);report.diagnostics.first().expect("diagnostic")}
fn ptr(d:&Diagnostic,expected:&str){assert_eq!(d.json_pointer.as_deref(),Some(expected));}

#[test] fn root_type_error_uses_empty_json_pointer(){
 let report=validate_input_json("42");let d=first(&report);assert_eq!(d.code,DiagnosticCode::JsonTypeMismatch);ptr(d,"");
}
#[test] fn nested_unknown_field_escapes_pointer_and_keeps_period_context(){
 let mut v=case_a();v["historical_periods"][1]["income_statement"].as_object_mut().unwrap().insert("bad/key~".into(),json!("x"));
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::UnknownField);ptr(d,"/historical_periods/1/income_statement/bad~1key~0");assert_eq!(d.field.as_deref(),Some("bad/key~"));assert_eq!(d.period_id.as_deref(),Some("FY2024"));
}
#[test] fn nested_missing_field_has_exact_pointer(){
 let mut v=case_a();v["historical_periods"][0]["income_statement"].as_object_mut().unwrap().remove("revenue");
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::RequiredFieldMissing);ptr(d,"/historical_periods/0/income_statement/revenue");assert_eq!(d.period_id.as_deref(),Some("FY2023"));
}
#[test] fn numeric_object_key_is_not_treated_as_array_index(){
 let mut v=case_a();v["model"].as_object_mut().unwrap().insert("123".into(),json!("x"));
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::UnknownField);ptr(d,"/model/123");assert_eq!(d.field.as_deref(),Some("123"));
}
#[test] fn invalid_enum_reports_pointer(){
 let mut v=case_a();v["model"]["unit_scale"]=json!("billions");
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::InvalidEnumValue);ptr(d,"/model/unit_scale");
}
#[test] fn array_element_type_error_reports_index(){
 let mut v=case_a();v["forecast_periods"][2]=json!("not an object");
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::JsonTypeMismatch);ptr(d,"/forecast_periods/2");assert_eq!(d.field.as_deref(),Some("forecast_periods"));
}
#[test] fn decimal_number_token_reports_exact_field(){
 let mut v=case_a();v["historical_periods"][0]["income_statement"]["revenue"]=json!(1000.0);
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::DecimalStringRequired);ptr(d,"/historical_periods/0/income_statement/revenue");
}
#[test] fn malformed_decimal_text_reports_noncanonical_code(){
 let mut v=case_a();v["historical_periods"][0]["income_statement"]["revenue"]=json!("01.0");
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::DecimalTextNonCanonical);ptr(d,"/historical_periods/0/income_statement/revenue");
}
#[test] fn decimal_scale_excess_reports_scale_code(){
 let mut v=case_a();v["historical_periods"][0]["income_statement"]["revenue"]=json!(format!("0.{}","1".repeat(29)));
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::DecimalScaleExceeded);ptr(d,"/historical_periods/0/income_statement/revenue");
}
#[test] fn decimal_out_of_range_reports_range_code(){
 let mut v=case_a();v["historical_periods"][0]["income_statement"]["revenue"]=json!("79228162514264337593543950336");
 let d=first(&validate_value(v));assert_eq!(d.code,DiagnosticCode::DecimalOutOfRange);ptr(d,"/historical_periods/0/income_statement/revenue");
}
#[test] fn malformed_json_has_no_fabricated_pointer(){
 let report=validate_input_json(r#"{"schema_version":"0.1.0""#);let d=first(&report);assert_eq!(d.code,DiagnosticCode::JsonParseFailed);assert!(d.json_pointer.is_none());assert!(d.field.is_none());
}
