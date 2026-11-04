use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use ofrl_model::validate_input_json;

const MAX_INPUT_BYTES: usize = 1_048_576;

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
    let file = File::open(&path)
        .map_err(|error| format!("Unable to open input file {path:?}: {error}"))?;
    let content = read_bounded_utf8(file)
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

/// Reads at most MAX_INPUT_BYTES + 1 bytes so oversized documents are rejected
/// without first allocating memory proportional to an untrusted file size.
fn read_bounded_utf8<R: Read>(reader: R) -> Result<String, String> {
    let read_limit = u64::try_from(MAX_INPUT_BYTES)
        .map_err(|_| "Configured input limit cannot be represented as u64".to_owned())?
        + 1;
    let mut limited_reader = reader.take(read_limit);
    let mut bytes = Vec::with_capacity(8 * 1024);

    limited_reader
        .read_to_end(&mut bytes)
        .map_err(|error| format!("I/O error while reading input: {error}"))?;

    if bytes.len() > MAX_INPUT_BYTES {
        return Err(format!(
            "Input document exceeds the V1 limit of {MAX_INPUT_BYTES} bytes"
        ));
    }

    String::from_utf8(bytes)
        .map_err(|error| format!("Input document is not valid UTF-8: {error}"))
}

#[cfg(test)]
mod tests {
    use super::{read_bounded_utf8, MAX_INPUT_BYTES};
    use std::io::Cursor;

    #[test]
    fn bounded_reader_accepts_document_at_the_limit() {
        let input = vec![b'a'; MAX_INPUT_BYTES];
        let output = read_bounded_utf8(Cursor::new(input)).expect("limit-sized input is allowed");

        assert_eq!(output.len(), MAX_INPUT_BYTES);
    }

    #[test]
    fn bounded_reader_rejects_document_over_the_limit() {
        let input = vec![b'a'; MAX_INPUT_BYTES + 1];
        let error = read_bounded_utf8(Cursor::new(input))
            .expect_err("input larger than the V1 limit must be rejected");

        assert!(error.contains("exceeds the V1 limit"));
    }

    #[test]
    fn bounded_reader_rejects_invalid_utf8() {
        let error = read_bounded_utf8(Cursor::new(vec![0xff, 0xfe]))
            .expect_err("invalid UTF-8 must not reach JSON parsing");

        assert!(error.contains("not valid UTF-8"));
    }

    #[test]
    fn bounded_reader_accepts_utf8_json_text() {
        let input = r#"{"schema_version":"0.1.0","model_name":"บริษัท"}"#;
        let output = read_bounded_utf8(Cursor::new(input.as_bytes()))
            .expect("valid UTF-8 should be preserved");

        assert!(output.contains("บริษัท"));
    }
}
