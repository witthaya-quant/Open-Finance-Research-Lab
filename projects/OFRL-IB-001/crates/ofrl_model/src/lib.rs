#![forbid(unsafe_code)]

//! Versioned input and historical semantic validation for OFRL-IB-001.
//!
//! This crate does not calculate forecasts. It prevents malformed,
//! semantically inconsistent, or unsupported V1 inputs from reaching a
//! future calculation engine.

mod diagnostics;
mod domain;
mod validation;

pub use diagnostics::{
    Diagnostic, DiagnosticCategory, DiagnosticCode, InputStatus, Severity, ValidationReport,
};
pub use validation::validate_input_json;
