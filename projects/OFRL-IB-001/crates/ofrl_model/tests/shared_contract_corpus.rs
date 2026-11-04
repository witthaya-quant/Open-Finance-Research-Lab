use ofrl_model::{validate_input_json, InputStatus};
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

const CORPUS: &str =
    include_str!("../../../../../fixtures/contracts/ofrl-ib-001-shared-corpus.v0.1.0.json");

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContractCorpus {
    corpus_version: String,
    schema_id: String,
    cases: Vec<ContractCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
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
    assert_eq!(
        corpus.schema_id,
        "urn:ofrl:schema:ofrl-ib-001-model-input:0.1.0"
    );
    assert!(!corpus.cases.is_empty());

    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../../")
        .canonicalize()
        .expect("repository root must resolve");
    let fixtures_root = repository_root
        .join("fixtures")
        .canonicalize()
        .expect("fixtures directory must resolve");

    let mut case_ids = HashSet::new();
    let mut input_paths = HashSet::<PathBuf>::new();

    for case in corpus.cases {
        assert!(
            case_ids.insert(case.id.clone()),
            "duplicate shared contract case id: {}",
            case.id
        );

        let relative_path = Path::new(&case.input);
        assert!(
            !relative_path.is_absolute()
                && relative_path.starts_with("fixtures")
                && relative_path
                    .components()
                    .all(|component| matches!(component, Component::Normal(_))),
            "{}: input path must be a normal relative path under fixtures/",
            case.id
        );

        let input_path = repository_root
            .join(relative_path)
            .canonicalize()
            .unwrap_or_else(|error| {
                panic!(
                    "{}: cannot resolve {}: {error}",
                    case.id,
                    relative_path.display()
                )
            });
        assert!(
            input_path.starts_with(&fixtures_root) && input_path.is_file(),
            "{}: input must resolve to a regular file under fixtures/",
            case.id
        );
        assert!(
            input_paths.insert(input_path.clone()),
            "{}: shared corpus input path is duplicated",
            case.id
        );

        let input = fs::read_to_string(&input_path).unwrap_or_else(|error| {
            panic!("{}: cannot read {}: {error}", case.id, input_path.display())
        });
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

        if !case.schema_valid {
            assert_eq!(
                report.input_status,
                InputStatus::Rejected,
                "{}: a JSON Schema-invalid input must not be accepted by the Rust runtime",
                case.id
            );
        }

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
        expected_codes.dedup();

        assert_eq!(
            actual_codes, expected_codes,
            "{}: Rust diagnostic code contract changed; schema_valid={} diagnostics={:#?}",
            case.id, case.schema_valid, report.diagnostics
        );
    }
}
