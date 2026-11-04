use std::str::FromStr;

use bigdecimal::BigDecimal;
use num_rational::BigRational;
use num_traits::One;
use ofrl_model_numeric_envelope::{
    balance_sheet_differences_are_zero, compare_to_reference, forecast, load_case_a,
    parse_decimal_rational, round_half_even, stress_case_a, ArbitraryDecimal, ExactRational,
    FixedDecimal, Numeric,
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
    assert!(fractional_digits >= 50);
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

    assert_eq!(fixed_summary.compared_values, 160);
    assert_eq!(arbitrary_summary.compared_values, 160);
    assert_eq!(fixed_summary.display_mismatches, 0);
    assert_eq!(arbitrary_summary.display_mismatches, 0);
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
