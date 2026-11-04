use ofrl_model_numeric_envelope::{forecast, load_case_a, ExactRational, Numeric};
use serde_json::Value;

const FIVE_YEAR_GOLDEN: &str = include_str!("../golden/case-a-five-year.v0.1.0.json");

#[test]
fn case_a_five_year_forecast_matches_independent_exact_golden() {
    let expected: Value =
        serde_json::from_str(FIVE_YEAR_GOLDEN).expect("five-year golden JSON must be valid");

    let fixture = load_case_a();
    assert_eq!(
        fixture["schema_version"].as_str(),
        expected["schema_version"].as_str(),
        "schema version must match the benchmark"
    );
    assert_eq!(
        fixture["model"]["model_id"].as_str(),
        expected["model_id"].as_str(),
        "model ID must match the benchmark"
    );

    let expected_periods = expected["periods"]
        .as_array()
        .expect("golden periods must be an array");
    assert_eq!(expected_periods.len(), 5, "benchmark must cover five years");

    let actual_periods =
        forecast::<ExactRational>(&fixture).expect("exact-rational forecast must complete");
    assert_eq!(actual_periods.len(), expected_periods.len());

    let mut compared_metrics = 0usize;

    for (period_index, (actual_period, expected_period)) in actual_periods
        .iter()
        .zip(expected_periods.iter())
        .enumerate()
    {
        let expected_period_id = expected_period["period_id"]
            .as_str()
            .expect("golden period_id must be a string");
        assert_eq!(
            actual_period.period_id, expected_period_id,
            "period order mismatch at index {period_index}"
        );

        let expected_values = expected_period["values"]
            .as_object()
            .expect("golden values must be an object");
        assert_eq!(
            expected_values.len(),
            32,
            "{expected_period_id} must contain all 32 expected metrics"
        );
        assert_eq!(
            actual_period.values.len(),
            expected_values.len(),
            "{expected_period_id} actual metric count differs from the golden"
        );

        for (metric, expected_fraction) in expected_values {
            let expected_fraction = expected_fraction
                .as_str()
                .unwrap_or_else(|| panic!("{expected_period_id}.{metric} must be a string"));
            let actual = actual_period
                .values
                .get(metric)
                .unwrap_or_else(|| panic!("{expected_period_id} is missing {metric}"));

            assert_eq!(
                actual.raw_text(),
                expected_fraction,
                "independent exact benchmark mismatch at {expected_period_id}.{metric}"
            );
            compared_metrics += 1;
        }

        assert_eq!(
            actual_period.values["balance_difference"].raw_text(),
            "0/1",
            "{expected_period_id} Balance Sheet must reconcile exactly"
        );
    }

    assert_eq!(
        compared_metrics, 160,
        "benchmark coverage must remain exactly 5 years × 32 metrics"
    );
}
