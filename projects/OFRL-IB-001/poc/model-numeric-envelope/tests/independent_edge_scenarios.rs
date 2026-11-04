use ofrl_model_numeric_envelope::{cash_deficit_case_a, forecast, load_case_a, ExactRational, Numeric};
use num_traits::Signed;
use serde_json::Value;

const EDGE_GOLDEN: &str = include_str!("../golden/case-a-edge-scenarios.v0.1.0.json");

fn set_all_assumptions(input: &mut Value, key: &str, value: &str) {
    for period in input["forecast_periods"]
        .as_array_mut()
        .expect("forecast_periods must be an array")
    {
        period["assumptions"][key] = Value::String(value.to_owned());
    }
}

fn edge_scenarios() -> Vec<(&'static str, Value)> {
    let high_capex = cash_deficit_case_a();

    let mut loss_making = load_case_a();
    set_all_assumptions(&mut loss_making, "cogs_pct_revenue", "0.65");
    set_all_assumptions(&mut loss_making, "operating_expenses_pct_revenue", "0.45");

    let mut full_repayment = load_case_a();
    full_repayment["forecast_periods"][0]["assumptions"]["principal_repayments"] =
        Value::String("180".to_owned());
    for period in full_repayment["forecast_periods"]
        .as_array_mut()
        .expect("forecast_periods must be an array")
        .iter_mut()
        .skip(1)
    {
        period["assumptions"]["principal_repayments"] = Value::String("0".to_owned());
    }

    let mut zero_activity = load_case_a();
    zero_activity["forecast_periods"][0]["assumptions"]["revenue_growth"] =
        Value::String("-1".to_owned());
    for period in zero_activity["forecast_periods"]
        .as_array_mut()
        .expect("forecast_periods must be an array")
        .iter_mut()
        .skip(1)
    {
        period["assumptions"]["revenue_growth"] = Value::String("0".to_owned());
    }
    set_all_assumptions(&mut zero_activity, "capex_pct_revenue", "0");
    set_all_assumptions(&mut zero_activity, "debt_drawdowns", "0");
    set_all_assumptions(&mut zero_activity, "principal_repayments", "0");
    set_all_assumptions(&mut zero_activity, "interest_rate", "0");

    vec![
        ("high_capex_funding_deficit", high_capex),
        ("loss_making_no_tax_no_dividend", loss_making),
        ("full_debt_repayment", full_repayment),
        ("zero_revenue_zero_capex", zero_activity),
    ]
}

#[test]
fn four_edge_scenarios_match_independent_exact_golden_for_all_five_years() {
    let expected: Value =
        serde_json::from_str(EDGE_GOLDEN).expect("BM-003 golden JSON must be valid");
    assert_eq!(expected["benchmark_id"].as_str(), Some("OFRL-IB-001-BM-003"));
    assert_eq!(expected["coverage"]["total_expected_values"].as_u64(), Some(640));

    let expected_cases = expected["cases"]
        .as_array()
        .expect("BM-003 cases must be an array");
    let actual_cases = edge_scenarios();
    assert_eq!(actual_cases.len(), expected_cases.len());
    assert_eq!(actual_cases.len(), 4);

    let mut compared_metrics = 0usize;

    for ((actual_case_id, input), expected_case) in actual_cases.iter().zip(expected_cases) {
        assert_eq!(
            Some(*actual_case_id),
            expected_case["case_id"].as_str(),
            "edge-case order and ID must match the versioned golden"
        );

        let actual_periods =
            forecast::<ExactRational>(input).expect("edge scenario exact forecast must complete");
        let expected_periods = expected_case["periods"]
            .as_array()
            .expect("edge-case periods must be an array");
        assert_eq!(actual_periods.len(), 5);
        assert_eq!(expected_periods.len(), 5);

        for (actual, expected) in actual_periods.iter().zip(expected_periods) {
            assert_eq!(
                actual.period_id,
                expected["period_id"].as_str().expect("period_id must be a string")
            );
            let expected_values = expected["values"]
                .as_object()
                .expect("golden values must be an object");
            assert_eq!(expected_values.len(), 32);
            assert_eq!(actual.values.len(), expected_values.len());

            for (metric, expected_fraction) in expected_values {
                let expected_fraction = expected_fraction.as_str().unwrap_or_else(|| {
                    panic!("{actual_case_id}.{}.{metric} must be a string", actual.period_id)
                });
                let actual_fraction = actual.values.get(metric).unwrap_or_else(|| {
                    panic!("{actual_case_id}.{} is missing {metric}", actual.period_id)
                });
                assert_eq!(
                    actual_fraction.raw_text(),
                    expected_fraction,
                    "independent exact mismatch at {actual_case_id}.{}.{metric}",
                    actual.period_id
                );
                compared_metrics += 1;
            }

            assert_eq!(
                actual.values["balance_difference"].raw_text(),
                "0/1",
                "{actual_case_id}.{} Balance Sheet must reconcile exactly",
                actual.period_id
            );
        }
    }

    assert_eq!(compared_metrics, 640);
}

#[test]
fn loss_making_case_floors_tax_and_dividends_without_suppressing_losses() {
    let (_, input) = edge_scenarios()
        .into_iter()
        .find(|(case_id, _)| *case_id == "loss_making_no_tax_no_dividend")
        .expect("loss-making edge scenario must exist");
    let periods = forecast::<ExactRational>(&input).expect("loss-making forecast must complete");

    for period in periods {
        assert!(
            period.values["net_income"].0.is_negative(),
            "{} must retain a negative Net Income result",
            period.period_id
        );
        assert_eq!(period.values["income_tax_expense"].raw_text(), "0/1");
        assert_eq!(period.values["dividends"].raw_text(), "0/1");
    }
}

#[test]
fn full_debt_repayment_and_later_redraws_use_average_balance_interest() {
    let (_, input) = edge_scenarios()
        .into_iter()
        .find(|(case_id, _)| *case_id == "full_debt_repayment")
        .expect("debt repayment edge scenario must exist");
    let periods = forecast::<ExactRational>(&input).expect("debt repayment forecast must complete");

    assert_eq!(periods[0].values["closing_debt"].raw_text(), "0/1");
    assert_eq!(periods[0].values["interest_expense"].raw_text(), "9/2");
    assert_eq!(periods[1].values["closing_debt"].raw_text(), "20/1");
    assert_eq!(periods[1].values["interest_expense"].raw_text(), "1/2");
    assert_eq!(periods[2].values["closing_debt"].raw_text(), "20/1");
    assert_eq!(periods[2].values["interest_expense"].raw_text(), "1/1");
}

#[test]
fn zero_revenue_and_zero_capex_preserve_exact_zero_operating_lines() {
    let (_, input) = edge_scenarios()
        .into_iter()
        .find(|(case_id, _)| *case_id == "zero_revenue_zero_capex")
        .expect("zero-activity edge scenario must exist");
    let periods = forecast::<ExactRational>(&input).expect("zero-activity forecast must complete");

    for period in periods {
        for metric in [
            "revenue", "cogs", "gross_profit", "operating_expenses", "ebitda",
            "accounts_receivable", "inventory", "accounts_payable", "net_working_capital",
            "capex", "cash_from_investing", "cash_from_financing",
        ] {
            assert_eq!(
                period.values[metric].raw_text(),
                "0/1",
                "{}.{metric} must remain exact zero",
                period.period_id
            );
        }
    }
}

#[test]
fn high_capex_funding_deficit_does_not_create_automatic_debt() {
    let (_, input) = edge_scenarios()
        .into_iter()
        .find(|(case_id, _)| *case_id == "high_capex_funding_deficit")
        .expect("high-CapEx edge scenario must exist");
    let periods = forecast::<ExactRational>(&input).expect("high-CapEx forecast must complete");

    assert_eq!(periods[0].values["closing_cash"].raw_text(), "-1531866007/292000");
    assert_eq!(periods[0].values["closing_debt"].raw_text(), "140/1");
    assert!(periods[0].values["closing_cash"].0.is_negative());
}
