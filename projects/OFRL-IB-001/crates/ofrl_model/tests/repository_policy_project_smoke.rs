use ofrl_model::{validate_input_json, InputStatus};

const CASE_A: &str =
    include_str!("../../../../../fixtures/synthetic/ofrl-ib-001-case-a.input.v0.1.0.json");

#[test]
fn public_input_validation_api_accepts_the_canonical_reference_case() {
    let report = validate_input_json(CASE_A);

    assert_eq!(report.input_status, InputStatus::Accepted);
    assert!(report.is_valid());
    assert!(
        report.diagnostics.is_empty(),
        "canonical reference input must not emit diagnostics: {:#?}",
        report.diagnostics
    );
}
