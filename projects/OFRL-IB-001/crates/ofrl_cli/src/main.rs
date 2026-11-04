use std::env;
use std::fs;
use std::process::ExitCode;

use ofrl_model::validate_input_json;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let mut arguments = env::args().skip(1);
    let command = arguments.next();
    let path = arguments.next();

    if command.as_deref() != Some("validate") || path.is_none() || arguments.next().is_some() {
        return Err("Usage: ofrl validate <model-input.json>\n\
            Validates the V1 input contract and historical accounting semantics."
            .to_owned());
    }

    let path = path.ok_or_else(|| "Usage: ofrl validate <model-input.json>".to_owned())?;
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("Unable to read input file {path:?}: {error}"))?;

    let report = validate_input_json(&content);
    let output = serde_json::to_string_pretty(&report)
        .map_err(|error| format!("Unable to serialize validation report: {error}"))?;

    println!("{output}");

    if report.is_valid() {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::from(2))
    }
}
