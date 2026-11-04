use std::str::FromStr;

use num_rational::BigRational;
use num_traits::{One, Signed};
use ofrl_model_numeric_envelope::{
    balance_sheet_differences_are_zero, cash_deficit_case_a, compare_to_reference, forecast, intermediate_stress_case_a,
    load_case_a, maximum_absolute_metric, parse_decimal_rational, round_half_even, stress_case_a,
    ArbitraryDecimal, ExactRational, FixedDecimal, Numeric,
};
use rust_decimal::Decimal;

#[test]
fn exact_rational_reference_preserves_repeating_division() {
    let one = parse_decimal_rational("1").unwrap();
    let three = parse_decimal_rational("3").unwrap();
    let three_sixty_five = parse_decimal_rational("365").unwrap();

    let one_third = &one / &three;
    assert_eq!(&one_third * &three, one);
    let one_over_365 = parse_decimal_rational("1").unwrap() / three_sixty_five.clone();
    assert_eq!(one_over_365 * three_sixty_five, BigRational::one());
}

#[test]
fn exact_reference_rounds_half_even_symmetrically() {
    assert_eq!(
        round_half_even(&parse_decimal_rational("1.225").unwrap(), 2),
        "1.22"
    );
    assert_eq!(
        round_half_even(&parse_decimal_rational("1.235").unwrap(), 2),
        "1.24"
    );
    assert_eq!(
        round_half_even(&parse_decimal_rational("-1.225").unwrap(), 2),
        "-1.22"
    );
    assert_eq!(
        round_half_even(&parse_decimal_rational("-1.235").unwrap(), 2),
        "-1.24"
    );
}

#[test]
fn exact_reference_is_not_a_floating_point_or_decimal_oracle() {
    let exact = parse_decimal_rational("1").unwrap() / parse_decimal_rational("365").unwrap();
    let decimal = FixedDecimal::one()
        .div(&FixedDecimal::from_i64(365))
        .unwrap()
        .as_rational();

    assert_ne!(decimal, exact);
    assert_eq!(round_half_even(&decimal, 12), round_half_even(&exact, 12));
}

#[test]
fn rust_decimal_candidate_rejects_scale_that_changes_the_input_value() {
    let text = "0.00000000000000000000000000001";
    assert!(FixedDecimal::parse(text).is_err());

    let raw = Decimal::from_str(text).expect("Decimal parser should accept a representable input");
    assert_ne!(raw.to_string(), text);
}

#[test]
fn arbitrary_decimal_context_is_visible_and_repeating_division_is_finite() {
    let one = ArbitraryDecimal::one();
    let three = ArbitraryDecimal::from_i64(3);
    let one_third = one.div(&three).unwrap();
    let rendered = one_third.raw_text();
    let fractional_digits = rendered.split_once('.').map_or(0, |(_, frac)| frac.len());

    println!("BIGDECIMAL_ONE_THIRD={rendered}");
    println!("BIGDECIMAL_ONE_THIRD_FRACTIONAL_DIGITS={fractional_digits}");
    assert_eq!(
        option_env!("RUST_BIGDECIMAL_DEFAULT_PRECISION"),
        Some("80"),
        "the POC must compile BigDecimal with an explicit 80-digit default context"
    );
    assert_eq!(
        option_env!("RUST_BIGDECIMAL_DEFAULT_ROUNDING_MODE"),
        Some("HalfEven"),
        "the POC must compile BigDecimal with the declared HalfEven rounding mode"
    );
    assert_eq!(
        fractional_digits, 80,
        "the compile-time context must be effective"
    );
}

#[test]
fn rust_decimal_large_amount_times_day_count_is_checked() {
    let amount = FixedDecimal::parse("16000000000000000.00000").unwrap();
    let days = FixedDecimal::parse("3650.0000").unwrap();
    let inventory_numerator = amount.mul(&days).unwrap();

    println!(
        "RUST_DECIMAL_LARGE_AMOUNT_DAY_PRODUCT={}",
        inventory_numerator.raw_text()
    );
    assert_eq!(
        round_half_even(&inventory_numerator.as_rational(), 0),
        "58400000000000000000"
    );
}

#[test]
fn canonical_case_a_has_exactly_balanced_rational_forecasts() {
    let fixture = load_case_a();
    let reference = forecast::<ExactRational>(&fixture).unwrap();

    assert_eq!(reference.len(), 5);
    assert!(balance_sheet_differences_are_zero(&reference));

    for period in &reference {
        let difference = period.values.get("balance_difference").unwrap();
        assert_eq!(difference.raw_text(), "0/1");
    }
}

#[test]
fn all_candidates_match_case_a_at_the_two_decimal_test_boundary() {
    let fixture = load_case_a();
    let reference = forecast::<ExactRational>(&fixture).unwrap();
    let fixed = forecast::<FixedDecimal>(&fixture).unwrap();
    let arbitrary = forecast::<ArbitraryDecimal>(&fixture).unwrap();

    let fixed_summary = compare_to_reference("rust_decimal", &fixed, &reference, 2).unwrap();
    let arbitrary_summary = compare_to_reference("bigdecimal", &arbitrary, &reference, 2).unwrap();

    println!("CASE_A_FIXED={fixed_summary:#?}");
    println!("CASE_A_BIGDECIMAL={arbitrary_summary:#?}");

    let fixed_balance_residual = maximum_absolute_metric(&fixed, "balance_difference").unwrap();
    let arbitrary_balance_residual =
        maximum_absolute_metric(&arbitrary, "balance_difference").unwrap();
    println!("CASE_A_FIXED_MAX_ABS_BS_RESIDUAL={fixed_balance_residual:?}");
    println!("CASE_A_BIGDECIMAL_MAX_ABS_BS_RESIDUAL={arbitrary_balance_residual:?}");

    assert_eq!(fixed_summary.compared_values, 160);
    assert_eq!(arbitrary_summary.compared_values, 160);
    assert_eq!(fixed_summary.display_mismatches, 0);
    assert_eq!(arbitrary_summary.display_mismatches, 0);
    assert_ne!(fixed_summary.maximum_absolute_error, "0/1");
    assert_ne!(arbitrary_summary.maximum_absolute_error, "0/1");
}

#[test]
fn stress_case_exact_reference_remains_balanced_and_candidates_report_capacity() {
    let fixture = stress_case_a();
    let reference = forecast::<ExactRational>(&fixture).unwrap();
    assert_eq!(reference.len(), 5);
    assert!(balance_sheet_differences_are_zero(&reference));

    let fixed_result = forecast::<FixedDecimal>(&fixture);
    match fixed_result {
        Ok(fixed) => {
            let summary = compare_to_reference("rust_decimal", &fixed, &reference, 2).unwrap();
            println!("STRESS_FIXED=accepted {summary:#?}");
        }
        Err(error) => println!("STRESS_FIXED=rejected {error}"),
    }

    let arbitrary = forecast::<ArbitraryDecimal>(&fixture).unwrap();
    let summary = compare_to_reference("bigdecimal", &arbitrary, &reference, 2).unwrap();
    println!("STRESS_BIGDECIMAL={summary:#?}");

    let maximum = ofrl_model_numeric_envelope::maximum_absolute_value(&reference).unwrap();
    println!("STRESS_EXACT_REFERENCE_MAXIMUM={maximum:#?}");
}

#[test]
fn negative_cash_is_reported_as_infeasible_without_fabricating_financing() {
    let fixture = cash_deficit_case_a();
    let forecast = forecast::<ExactRational>(&fixture).unwrap();

    assert_eq!(forecast.len(), 5);
    assert!(
        balance_sheet_differences_are_zero(&forecast),
        "negative cash must not be hidden by a balancing plug"
    );
    let fy2026 = forecast
        .iter()
        .find(|period| period.period_id == "FY2026")
        .expect("FY2026 diagnostic period must exist");
    assert_eq!(
        fy2026.values.get("closing_cash").unwrap().raw_text(),
        "-1531866007/292000"
    );
    assert!(
        forecast.iter().any(|period| {
            period
                .values
                .get("closing_cash")
                .is_some_and(|cash| cash.0.is_negative())
        }),
        "the deliberately high-CapEx scenario must demonstrate a funding deficit"
    );

    // Opening debt is 180; first-year drawdown is 0 and principal repayment is 40.
    // A cash deficit must not trigger automatic borrowing.
    assert_eq!(
        forecast[0].values.get("closing_debt").unwrap().raw_text(),
        "140/1"
    );
}

#[test]
fn case_a_fy2026_matches_independently_derived_exact_benchmark() {
    // These constants were derived separately from the shared generic forecast
    // function using the published Case A opening balances and FY2026 assumptions.
    // They are exact rational values, not rounded display outputs.
    let result = forecast::<ExactRational>(&load_case_a()).unwrap();
    assert_eq!(result.len(), 5);

    let fy2026 = result
        .iter()
        .find(|period| period.period_id == "FY2026")
        .expect("FY2026 benchmark period must exist");

    let expected = [
        ("revenue", "2835/2"),
        ("cogs", "567/1"),
        ("gross_profit", "1701/2"),
        ("operating_expenses", "2835/8"),
        ("ebitda", "3969/8"),
        ("depreciation", "1797081/40000"),
        ("ebit", "18047919/40000"),
        ("interest_expense", "8/1"),
        ("ebt", "17727919/40000"),
        ("income_tax_expense", "17727919/200000"),
        ("net_income", "17727919/50000"),
        ("accounts_receivable", "12474/73"),
        ("inventory", "567/5"),
        ("accounts_payable", "1701/20"),
        ("net_working_capital", "290871/1460"),
        ("change_in_nwc", "14931/1460"),
        ("capex", "7371/100"),
        ("gross_ppe", "85371/100"),
        ("accumulated_depreciation", "13997081/40000"),
        ("net_ppe", "20151319/40000"),
        ("debt_drawdowns", "0/1"),
        ("principal_repayments", "40/1"),
        ("closing_debt", "140/1"),
        ("dividends", "17727919/250000"),
        ("cash_from_operations", "5683176913/14600000"),
        ("cash_from_investing", "-7371/100"),
        ("cash_from_financing", "-27727919/250000"),
        ("net_change_in_cash", "14938502217/73000000"),
        ("closing_cash", "123971302217/73000000"),
        ("total_assets", "38848386/15625"),
        ("total_liabilities_equity", "38848386/15625"),
        ("balance_difference", "0/1"),
    ];

    assert_eq!(fy2026.values.len(), expected.len());

    for (metric, expected_fraction) in expected {
        let actual = fy2026
            .values
            .get(metric)
            .unwrap_or_else(|| panic!("FY2026 output is missing metric {metric}"));
        assert_eq!(
            actual.raw_text(),
            expected_fraction,
            "independent FY2026 benchmark mismatch for {metric}"
        );
    }
}


#[test]
fn intermediate_stress_case_completes_on_all_numeric_candidates() {
    let fixture = intermediate_stress_case_a();
    let reference = forecast::<ExactRational>(&fixture).unwrap();
    let fixed = forecast::<FixedDecimal>(&fixture)
        .expect("10^18 stress point must either be supported or explicitly rejected by the test");
    let arbitrary = forecast::<ArbitraryDecimal>(&fixture).unwrap();

    assert_eq!(reference.len(), 5);
    assert!(balance_sheet_differences_are_zero(&reference));

    let fixed_summary =
        compare_to_reference("rust_decimal", &fixed, &reference, 2).unwrap();
    let arbitrary_summary =
        compare_to_reference("bigdecimal", &arbitrary, &reference, 2).unwrap();

    println!("INTERMEDIATE_STRESS_FIXED={fixed_summary:#?}");
    println!("INTERMEDIATE_STRESS_BIGDECIMAL={arbitrary_summary:#?}");
    println!(
        "INTERMEDIATE_STRESS_FIXED_MAX_BS_RESIDUAL={:?}",
        maximum_absolute_metric(&fixed, "balance_difference").unwrap()
    );
    println!(
        "INTERMEDIATE_STRESS_BIGDECIMAL_MAX_BS_RESIDUAL={:?}",
        maximum_absolute_metric(&arbitrary, "balance_difference").unwrap()
    );
    println!(
        "INTERMEDIATE_STRESS_EXACT_MAXIMUM={:?}",
        ofrl_model_numeric_envelope::maximum_absolute_value(&reference).unwrap()
    );

    assert_eq!(fixed_summary.compared_values, 160);
    assert_eq!(arbitrary_summary.compared_values, 160);
    assert_eq!(fixed_summary.display_mismatches, 0);
    assert_eq!(arbitrary_summary.display_mismatches, 0);
}
