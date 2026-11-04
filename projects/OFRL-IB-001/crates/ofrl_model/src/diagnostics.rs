use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InputStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCategory {
    InputSyntax,
    Schema,
    SemanticInput,
    Calculation,
    StructuralIntegrityFinding,
    ScenarioFeasibilityFinding,
    Warning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DiagnosticCode {
    JsonParseFailed,
    JsonTypeMismatch,
    DecimalStringRequired,
    DecimalTextNonCanonical,
    DecimalScaleExceeded,
    DecimalOutOfRange,
    SchemaVersionUnsupported,
    RequiredFieldMissing,
    UnknownField,
    InvalidEnumValue,
    InvalidDate,
    InvalidCurrencyFormat,
    InvalidIdentifierFormat,
    StringConstraintViolation,
    PeriodCountInvalid,
    PeriodIdDuplicate,
    PeriodOrderInvalid,
    PeriodOverlap,
    PeriodGap,
    PeriodHandoffInvalid,
    PeriodDurationInvalid,
    PeriodIdYearMismatch,
    NumericDomainInvalid,
    UnitMetadataInvalid,
    DebtRepaymentExceedsAvailable,
    HistoricalNetIncomeMismatch,
    HistoricalBalanceSheetUnbalanced,
    HistoricalCashFlowReconciliationFailed,
    HistoricalCashTieoutFailed,
    HistoricalNwcRollforwardFailed,
    HistoricalPpeRollforwardFailed,
    HistoricalDebtRollforwardFailed,
    HistoricalRetainedEarningsFailed,
    ShareCapitalMovementUnsupported,
    DecimalOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub category: DiagnosticCategory,
    pub severity: Severity,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_pointer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
}

impl Diagnostic {
    pub(crate) fn error(
        code: DiagnosticCode,
        category: DiagnosticCategory,
        message: impl Into<String>,
        json_pointer: Option<String>,
        period_id: Option<String>,
        field: Option<String>,
        expected: Option<String>,
        actual: Option<String>,
    ) -> Self {
        Self {
            code,
            category,
            severity: Severity::Error,
            message: message.into(),
            json_pointer,
            period_id,
            field,
            expected,
            actual,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationReport {
    pub input_status: InputStatus,
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationReport {
    pub(crate) fn accepted() -> Self {
        Self {
            input_status: InputStatus::Accepted,
            diagnostics: Vec::new(),
        }
    }

    pub(crate) fn rejected(diagnostic: Diagnostic) -> Self {
        Self {
            input_status: InputStatus::Rejected,
            diagnostics: vec![diagnostic],
        }
    }

    pub(crate) fn from_diagnostics(diagnostics: Vec<Diagnostic>) -> Self {
        let input_status = if diagnostics
            .iter()
            .any(|item| item.severity == Severity::Error)
        {
            InputStatus::Rejected
        } else {
            InputStatus::Accepted
        };

        Self {
            input_status,
            diagnostics,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.input_status == InputStatus::Accepted
    }
}
