use ofrl_model::{validate_input_json, DiagnosticCode, InputStatus};
use serde::Deserialize;
use std::fs;

const CORPUS: &str =
    include_str!("../../../../../fixtures/contracts/ofrl-ib-001-shared-corpus.v0.1.0.json");

#[derive(Debug, Deserialize)]
struct ContractCorpus {
    corpus_version: String,
    cases: Vec<ContractCase>,
}

#[derive(Debug, Deserialize)]
struct ContractCase {
    id: String,
    input: String,
    schema_valid: bool,
    runtime_status: String,
    diagnostic_codes: Vec<String>,
}

#[test]
fn shared_contract_corpus_matches_rust_runtime_expectations() {
    let corpus: ContractCorpus =
        serde_json::from_str(CORPUS).expect("shared contract corpus must be valid JSON");

    assert_eq!(corpus.corpus_version, "0.1.0");
    assert!(!corpus.cases.is_empty());

    for case in corpus.cases {
        let input = fs::read_to_string(&case.input)
            .unwrap_or_else(|error| panic!("{}: cannot read {}: {error}", case.id, case.input));
        let report = validate_input_json(&input);

        let expected_status = match case.runtime_status.as_str() {
            "accepted" => InputStatus::Accepted,
            "rejected" => InputStatus::Rejected,
            other => panic!("{}: unsupported expected runtime status {other}", case.id),
        };
        assert_eq!(
            report.input_status, expected_status,
            "{}: unexpected Rust runtime status: {:#?}",
            case.id, report.diagnostics
        );

        let mut actual_codes = report
            .diagnostics
            .iter()
            .map(|diagnostic| {
                serde_json::to_value(diagnostic.code)
                    .expect("diagnostic code serializes")
                    .as_str()
                    .expect("diagnostic code is serialized as a string")
                    .to_owned()
            })
            .collect::<Vec<_>>();
        actual_codes.sort();

        let mut expected_codes = case.diagnostic_codes;
        expected_codes.sort();

        assert_eq!(
            actual_codes, expected_codes,
            "{}: Rust diagnostic code contract changed; schema_valid={} diagnostics={:#?}",
            case.id, case.schema_valid, report.diagnostics
        );
    }
}
