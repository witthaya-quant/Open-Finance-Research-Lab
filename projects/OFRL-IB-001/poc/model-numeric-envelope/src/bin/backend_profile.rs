use std::hint::black_box;
use std::time::Instant;

use ofrl_model_numeric_envelope::{
    forecast, intermediate_stress_case_a, load_case_a, ArbitraryDecimal, ExactRational,
    FixedDecimal, Numeric,
};
use serde_json::Value;

const WARMUP_ITERATIONS: usize = 5;
const SAMPLE_COUNT: usize = 9;
const ITERATIONS_PER_SAMPLE: usize = 40;

struct TimingSummary {
    min_ns_per_forecast: u128,
    median_ns_per_forecast: u128,
    max_ns_per_forecast: u128,
}

fn measure<N: Numeric>(input: &Value) -> Result<TimingSummary, String> {
    for _ in 0..WARMUP_ITERATIONS {
        let output = forecast::<N>(black_box(input))?;
        black_box(output);
    }

    let mut samples = Vec::with_capacity(SAMPLE_COUNT);
    for _ in 0..SAMPLE_COUNT {
        let start = Instant::now();
        for _ in 0..ITERATIONS_PER_SAMPLE {
            let output = forecast::<N>(black_box(input))?;
            black_box(output);
        }
        let elapsed = start.elapsed().as_nanos();
        samples.push(elapsed / ITERATIONS_PER_SAMPLE as u128);
    }

    samples.sort_unstable();
    let median = samples[samples.len() / 2];

    Ok(TimingSummary {
        min_ns_per_forecast: samples[0],
        median_ns_per_forecast: median,
        max_ns_per_forecast: samples[samples.len() - 1],
    })
}

fn emit<N: Numeric>(scenario: &str, candidate: &str, input: &Value) -> Result<(), String> {
    let timing = measure::<N>(input).map_err(|error| {
        format!("{scenario}/{candidate}: benchmark calculation failed: {error}")
    })?;

    println!(
        "{scenario},{candidate},{ITERATIONS_PER_SAMPLE},{SAMPLE_COUNT},{},{},{}",
        timing.min_ns_per_forecast, timing.median_ns_per_forecast, timing.max_ns_per_forecast
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let scenarios = [
        ("case_a", load_case_a()),
        ("stress_1e18", intermediate_stress_case_a()),
    ];

    println!("# benchmark=OFRL-IB-001 numeric backend timing; release profile; Rust toolchain matrix");
    println!(
        "# warmup_iterations={WARMUP_ITERATIONS}; samples={SAMPLE_COUNT}; iterations_per_sample={ITERATIONS_PER_SAMPLE}"
    );
    println!(
        "# interpretation=observational only; shared-runner timings are not release thresholds"
    );
    println!(
        "scenario,candidate,iterations_per_sample,samples,min_ns_per_forecast,median_ns_per_forecast,max_ns_per_forecast"
    );

    for (scenario, input) in &scenarios {
        emit::<FixedDecimal>(scenario, "rust_decimal_1_43_0", input)?;
        emit::<ArbitraryDecimal>(scenario, "bigdecimal_0_4_11", input)?;
        emit::<ExactRational>(scenario, "exact_rational_reference", input)?;
    }

    Ok(())
}
