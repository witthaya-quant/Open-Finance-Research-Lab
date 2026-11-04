use std::collections::BTreeSet;

use rust_decimal::Decimal;
use serde_json::Value;

use crate::diagnostics::{
    Diagnostic, DiagnosticCategory, DiagnosticCode, ValidationReport,
};
use crate::domain::{
    BalanceSheet, CalendarDate, CashFlowStatement, ForecastAssumptions, HistoricalPeriod,
    IncomeStatement, ModelInput,
};

const V1_SCHEMA_VERSION: &str = "0.1.0";
const V1_DAY_COUNT_BASIS: u16 = 365;
const EXPECTED_HISTORICAL_PERIODS: usize = 3;
const EXPECTED_FORECAST_PERIODS: usize = 5;

#[derive(Debug, Clone)]
struct PeriodWindow {
    id: String,
    start: CalendarDate,
    end: CalendarDate,
    pointer: String,
}

pub fn validate_input_json(input: &str) -> ValidationReport {
    let value = match serde_json::from_str::<Value>(input) {
        Ok(value) => value,
        Err(error) => {
            return ValidationReport::rejected(Diagnostic::error(
                DiagnosticCode::JsonParseFailed,
                DiagnosticCategory::InputSyntax,
                format!("Input is not valid JSON: {error}"),
                None,
                None,
                None,
                Some("valid JSON document".to_owned()),
                Some(error.to_string()),
            ));
        }
    };

    let model = match serde_json::from_value::<ModelInput>(value) {
        Ok(model) => model,
        Err(error) => return deserialize_failure(error.to_string()),
    };

    if model.schema_version != V1_SCHEMA_VERSION {
        return ValidationReport::rejected(Diagnostic::error(
            DiagnosticCode::SchemaVersionUnsupported,
            DiagnosticCategory::Schema,
            "The supplied schema version is not supported by this validator.",
            Some("/schema_version".to_owned()),
            None,
            Some("schema_version".to_owned()),
            Some(V1_SCHEMA_VERSION.to_owned()),
            Some(model.schema_version),
        ));
    }

    let mut diagnostics = Vec::new();

    validate_metadata(&model, &mut diagnostics);
    validate_periods(&model, &mut diagnostics);
    validate_opening_balance_sheet(&model, &mut diagnostics);
    validate_historical_periods(&model, &mut diagnostics);
    validate_forecast_assumptions(&model, &mut diagnostics);

    ValidationReport::from_diagnostics(diagnostics)
}

fn deserialize_failure(message: String) -> ValidationReport {
    let (code, category, human_message) =
        if message.contains("DECIMAL_STRING_REQUIRED") || message.contains("JSON_TYPE_MISMATCH") {
            (
                DiagnosticCode::JsonTypeMismatch,
                DiagnosticCategory::InputSyntax,
                "A field has the wrong JSON type. Decimal inputs must be JSON strings.".to_owned(),
            )
        } else if message.contains("DECIMAL_TEXT_NON_CANONICAL") {
            (
                DiagnosticCode::DecimalTextNonCanonical,
                DiagnosticCategory::InputSyntax,
                "A Decimal string does not use the canonical base-10 grammar.".to_owned(),
            )
        } else if message.contains("DECIMAL_SCALE_EXCEEDED") {
            (
                DiagnosticCode::DecimalScaleExceeded,
                DiagnosticCategory::InputSyntax,
                "A Decimal string exceeds the maximum supported fractional scale.".to_owned(),
            )
        } else if message.contains("DECIMAL_OUT_OF_RANGE") {
            (
                DiagnosticCode::DecimalOutOfRange,
                DiagnosticCategory::InputSyntax,
                "A Decimal string is outside the supported representation or loses information when parsed.".to_owned(),
            )
        } else if message.contains("unknown field") {
            (
                DiagnosticCode::UnknownField,
                DiagnosticCategory::Schema,
                "The input contains a field that is not declared in this schema version.".to_owned(),
            )
        } else if message.contains("missing field") {
            (
                DiagnosticCode::RequiredFieldMissing,
                DiagnosticCategory::Schema,
                "The input is missing a required field.".to_owned(),
            )
        } else if message.contains("invalid value") || message.contains("unknown variant") {
            (
                DiagnosticCode::InvalidEnumValue,
                DiagnosticCategory::Schema,
                "The input contains a value outside the declared enumeration.".to_owned(),
            )
        } else if message.contains("invalid type") {
            (
                DiagnosticCode::JsonTypeMismatch,
                DiagnosticCategory::InputSyntax,
                "A field has the wrong JSON type.".to_owned(),
            )
        } else {
            (
                DiagnosticCode::InvalidEnumValue,
                DiagnosticCategory::Schema,
                "The input could not be deserialized into the closed V1 model contract.".to_owned(),
            )
        };

    ValidationReport::rejected(Diagnostic::error(
        code,
        category,
        human_message,
        None,
        None,
        None,
        None,
        Some(message),
    ))
}

fn push(
    diagnostics: &mut Vec<Diagnostic>,
    code: DiagnosticCode,
    category: DiagnosticCategory,
    message: impl Into<String>,
    pointer: impl Into<String>,
    period_id: Option<&str>,
    field: Option<&str>,
    expected: Option<String>,
    actual: Option<String>,
) {
    diagnostics.push(Diagnostic::error(
        code,
        category,
        message,
        Some(pointer.into()),
        period_id.map(str::to_owned),
        field.map(str::to_owned),
        expected,
        actual,
    ));
}

fn push_semantic(
    diagnostics: &mut Vec<Diagnostic>,
    code: DiagnosticCode,
    message: impl Into<String>,
    pointer: impl Into<String>,
    period_id: Option<&str>,
    field: Option<&str>,
    expected: Option<String>,
    actual: Option<String>,
) {
    push(
        diagnostics,
        code,
        DiagnosticCategory::SemanticInput,
        message,
        pointer,
        period_id,
        field,
        expected,
        actual,
    );
}

fn validate_metadata(model: &ModelInput, diagnostics: &mut Vec<Diagnostic>) {
    if model.model.model_id.trim().is_empty() {
        push_semantic(
            diagnostics,
            DiagnosticCode::RequiredFieldMissing,
            "model_id must not be empty.",
            "/model/model_id",
            None,
            Some("model_id"),
            Some("non-empty identifier".to_owned()),
            Some(model.model.model_id.clone()),
        );
    }

    if model.model.model_name.trim().is_empty() {
        push_semantic(
            diagnostics,
            DiagnosticCode::RequiredFieldMissing,
            "model_name must not be empty.",
            "/model/model_name",
            None,
            Some("model_name"),
            Some("non-empty name".to_owned()),
            Some(model.model.model_name.clone()),
        );
    }

    let currency = model.model.reporting_currency.as_bytes();
    if currency.len() != 3 || !currency.iter().all(u8::is_ascii_uppercase) {
        push_semantic(
            diagnostics,
            DiagnosticCode::InvalidCurrencyFormat,
            "reporting_currency must be three uppercase ASCII letters. This validates syntax, not ISO 4217 membership.",
            "/model/reporting_currency",
            None,
            Some("reporting_currency"),
            Some("three uppercase ASCII letters".to_owned()),
            Some(model.model.reporting_currency.clone()),
        );
    }

    if model.model.day_count_basis != V1_DAY_COUNT_BASIS {
        push_semantic(
            diagnostics,
            DiagnosticCode::NumericDomainInvalid,
            "V1 working-capital calculations require day_count_basis = 365.",
            "/model/day_count_basis",
            None,
            Some("day_count_basis"),
            Some(V1_DAY_COUNT_BASIS.to_string()),
            Some(model.model.day_count_basis.to_string()),
        );
    }

    // Reading the enum here makes the allowed unit set explicit in the
    // runtime boundary. No conversion occurs in this validator.
    match model.model.unit_scale {
        crate::domain::UnitScale::Units
        | crate::domain::UnitScale::Thousands
        | crate::domain::UnitScale::Millions => {}
    }
}

fn validate_periods(model: &ModelInput, diagnostics: &mut Vec<Diagnostic>) {
    if model.historical_periods.len() != EXPECTED_HISTORICAL_PERIODS {
        push_semantic(
            diagnostics,
            DiagnosticCode::PeriodCountInvalid,
            "V1 requires exactly three annual historical periods.",
            "/historical_periods",
            None,
            Some("historical_periods"),
            Some(EXPECTED_HISTORICAL_PERIODS.to_string()),
            Some(model.historical_periods.len().to_string()),
        );
    }

    if model.forecast_periods.len() != EXPECTED_FORECAST_PERIODS {
        push_semantic(
            diagnostics,
            DiagnosticCode::PeriodCountInvalid,
            "V1 requires exactly five annual forecast periods.",
            "/forecast_periods",
            None,
            Some("forecast_periods"),
            Some(EXPECTED_FORECAST_PERIODS.to_string()),
            Some(model.forecast_periods.len().to_string()),
        );
    }

    let mut seen = BTreeSet::new();
    let mut windows: Vec<Option<PeriodWindow>> = Vec::new();

    for (index, period) in model.historical_periods.iter().enumerate() {
        let pointer = format!("/historical_periods/{index}");
        let window = inspect_period(
            &period.period_id,
            &period.start_date,
            &period.end_date,
            &pointer,
            diagnostics,
        );
        register_period_id(&period.period_id, &pointer, &mut seen, diagnostics);
        windows.push(window);
    }

    for (index, period) in model.forecast_periods.iter().enumerate() {
        let pointer = format!("/forecast_periods/{index}");
        let window = inspect_period(
            &period.period_id,
            &period.start_date,
            &period.end_date,
            &pointer,
            diagnostics,
        );
        register_period_id(&period.period_id, &pointer, &mut seen, diagnostics);
        windows.push(window);
    }

    for index in 1..windows.len() {
        if let (Some(previous), Some(current)) = (&windows[index - 1], &windows[index]) {
            validate_adjacent_periods(previous, current, diagnostics);
        }
    }

    if let Some(first_historical) = windows.first().and_then(Option::as_ref) {
        match CalendarDate::parse(&model.opening_balance_sheet.as_of_date) {
            Some(opening_date) if opening_date.next_day() == Some(first_historical.start) => {}
            Some(opening_date) => push_semantic(
                diagnostics,
                DiagnosticCode::PeriodHandoffInvalid,
                "The first historical period must begin on the day immediately after the opening Balance Sheet date.",
                "/opening_balance_sheet/as_of_date",
                Some(&first_historical.id),
                Some("as_of_date"),
                opening_date
                    .next_day()
                    .map(|date| date.to_string())
                    .or_else(|| Some("a date after the maximum supported calendar date".to_owned())),
                Some(model.opening_balance_sheet.as_of_date.clone()),
            ),
            None => push_semantic(
                diagnostics,
                DiagnosticCode::InvalidDate,
                "opening_balance_sheet.as_of_date is not a valid Gregorian calendar date.",
                "/opening_balance_sheet/as_of_date",
                None,
                Some("as_of_date"),
                Some("YYYY-MM-DD".to_owned()),
                Some(model.opening_balance_sheet.as_of_date.clone()),
            ),
        }
    }

    if !model.historical_periods.is_empty() && !model.forecast_periods.is_empty() {
        let last_historical_index = model.historical_periods.len() - 1;
        let first_forecast_index = model.historical_periods.len();
        if let (Some(last_historical), Some(first_forecast)) = (
            windows.get(last_historical_index).and_then(Option::as_ref),
            windows.get(first_forecast_index).and_then(Option::as_ref),
        ) {
            validate_adjacent_periods(last_historical, first_forecast, diagnostics);
        }
    }
}

fn register_period_id(
    period_id: &str,
    pointer: &str,
    seen: &mut BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if !seen.insert(period_id.to_owned()) {
        push_semantic(
            diagnostics,
            DiagnosticCode::PeriodIdDuplicate,
            "Every historical and forecast period_id must be unique.",
            pointer,
            Some(period_id),
            Some("period_id"),
            Some("unique identifier".to_owned()),
            Some(period_id.to_owned()),
        );
    }
}

fn inspect_period(
    period_id: &str,
    start_text: &str,
    end_text: &str,
    pointer: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<PeriodWindow> {
    let start = match CalendarDate::parse(start_text) {
        Some(date) => date,
        None => {
            push_semantic(
                diagnostics,
                DiagnosticCode::InvalidDate,
                "Period start_date is not a valid Gregorian calendar date.",
                format!("{pointer}/start_date"),
                Some(period_id),
                Some("start_date"),
                Some("YYYY-MM-DD".to_owned()),
                Some(start_text.to_owned()),
            );
            return None;
        }
    };

    let end = match CalendarDate::parse(end_text) {
        Some(date) => date,
        None => {
            push_semantic(
                diagnostics,
                DiagnosticCode::InvalidDate,
                "Period end_date is not a valid Gregorian calendar date.",
                format!("{pointer}/end_date"),
                Some(period_id),
                Some("end_date"),
                Some("YYYY-MM-DD".to_owned()),
                Some(end_text.to_owned()),
            );
            return None;
        }
    };

    if start > end {
        push_semantic(
            diagnostics,
            DiagnosticCode::PeriodOrderInvalid,
            "A reporting period start_date must not be after its end_date.",
            pointer,
            Some(period_id),
            Some("start_date/end_date"),
            Some("start_date <= end_date".to_owned()),
            Some(format!("{start_text} > {end_text}")),
        );
        return None;
    }

    let inclusive_days = end.ordinal() - start.ordinal() + 1;
    if inclusive_days != 365 && inclusive_days != 366 {
        push_semantic(
            diagnostics,
            DiagnosticCode::PeriodDurationInvalid,
            "V1 accepts calendar-based annual periods of 365 or 366 days; 52/53-week calendars are outside this contract.",
            pointer,
            Some(period_id),
            Some("period_duration"),
            Some("365 or 366 inclusive calendar days".to_owned()),
            Some(inclusive_days.to_string()),
        );
    }

    let expected_id = format!("FY{:04}", end.year);
    if period_id != expected_id {
        push_semantic(
            diagnostics,
            DiagnosticCode::PeriodIdYearMismatch,
            "The V1 period_id must use the calendar year in which the reporting period ends.",
            format!("{pointer}/period_id"),
            Some(period_id),
            Some("period_id"),
            Some(expected_id),
            Some(period_id.to_owned()),
        );
    }

    Some(PeriodWindow {
        id: period_id.to_owned(),
        start,
        end,
        pointer: pointer.to_owned(),
    })
}

fn validate_adjacent_periods(
    previous: &PeriodWindow,
    current: &PeriodWindow,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match previous.end.next_day() {
        Some(expected_start) if current.start == expected_start => {}
        Some(expected_start) if current.start <= previous.end => push_semantic(
            diagnostics,
            DiagnosticCode::PeriodOverlap,
            "Reporting periods must not overlap.",
            format!("{}/start_date", current.pointer),
            Some(&current.id),
            Some("start_date"),
            Some(expected_start.to_string()),
            Some(current.start.to_string()),
        ),
        Some(expected_start) => push_semantic(
            diagnostics,
            DiagnosticCode::PeriodGap,
            "Adjacent reporting periods must be contiguous with no missing calendar day.",
            format!("{}/start_date", current.pointer),
            Some(&current.id),
            Some("start_date"),
            Some(expected_start.to_string()),
            Some(current.start.to_string()),
        ),
        None => push_semantic(
            diagnostics,
            DiagnosticCode::PeriodHandoffInvalid,
            "A period cannot follow a reporting period ending on the maximum supported date.",
            format!("{}/start_date", current.pointer),
            Some(&current.id),
            Some("start_date"),
            None,
            Some(current.start.to_string()),
        ),
    }
}

fn validate_opening_balance_sheet(model: &ModelInput, diagnostics: &mut Vec<Diagnostic>) {
    let balances = &model.opening_balance_sheet.balances;
    validate_balance_domains(
        balances,
        "/opening_balance_sheet/balances",
        None,
        diagnostics,
    );
    validate_balance_sheet_identity(
        balances,
        "/opening_balance_sheet/balances",
        None,
        diagnostics,
    );
}

fn validate_historical_periods(model: &ModelInput, diagnostics: &mut Vec<Diagnostic>) {
    let mut previous = &model.opening_balance_sheet.balances;

    for (index, period) in model.historical_periods.iter().enumerate() {
        let pointer = format!("/historical_periods/{index}");
        let period_id = period.period_id.as_str();

        if period.source.source_reference.trim().is_empty() {
            push_semantic(
                diagnostics,
                DiagnosticCode::RequiredFieldMissing,
                "Every synthetic historical period must include a non-empty source_reference.",
                format!("{pointer}/source/source_reference"),
                Some(period_id),
                Some("source_reference"),
                Some("non-empty provenance reference".to_owned()),
                Some(period.source.source_reference.clone()),
            );
        }

        match period.source.classification {
            crate::domain::SourceClassification::Synthetic => {}
        }

        validate_income_statement_domains(
            &period.income_statement,
            &format!("{pointer}/income_statement"),
            Some(period_id),
            diagnostics,
        );
        validate_cash_flow_domains(
            &period.cash_flow_statement,
            &format!("{pointer}/cash_flow_statement"),
            Some(period_id),
            diagnostics,
        );
        validate_balance_domains(
            &period.balance_sheet,
            &format!("{pointer}/balance_sheet"),
            Some(period_id),
            diagnostics,
        );

        validate_balance_sheet_identity(
            &period.balance_sheet,
            &format!("{pointer}/balance_sheet"),
            Some(period_id),
            diagnostics,
        );

        validate_historical_income_statement(
            &period.income_statement,
            &format!("{pointer}/income_statement"),
            period_id,
            diagnostics,
        );

        validate_historical_cash_flow(
            &period.income_statement,
            &period.cash_flow_statement,
            &period.balance_sheet,
            previous,
            &format!("{pointer}/cash_flow_statement"),
            period_id,
            diagnostics,
        );

        validate_historical_rollforwards(
            &period.income_statement,
            &period.balance_sheet,
            &period.cash_flow_statement,
            previous,
            &pointer,
            period_id,
            diagnostics,
        );

        previous = &period.balance_sheet;
    }
}

fn validate_income_statement_domains(
    statement: &IncomeStatement,
    pointer: &str,
    period_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (field, value) in [
        ("revenue", statement.revenue),
        ("cogs", statement.cogs),
        ("operating_expenses", statement.operating_expenses),
        ("depreciation", statement.depreciation),
        ("interest_expense", statement.interest_expense),
        ("income_tax_expense", statement.income_tax_expense),
    ] {
        require_non_negative(
            value,
            diagnostics,
            &format!("{pointer}/{field}"),
            period_id,
            field,
        );
    }
}

fn validate_cash_flow_domains(
    statement: &CashFlowStatement,
    pointer: &str,
    period_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (field, value) in [
        ("depreciation_addback", statement.depreciation_addback),
        ("capex", statement.capex),
        ("debt_drawdowns", statement.debt_drawdowns),
        ("principal_repayments", statement.principal_repayments),
        ("dividends", statement.dividends),
        ("opening_cash", statement.opening_cash),
        ("closing_cash", statement.closing_cash),
    ] {
        require_non_negative(
            value,
            diagnostics,
            &format!("{pointer}/{field}"),
            period_id,
            field,
        );
    }
}

fn validate_balance_domains(
    balances: &BalanceSheet,
    pointer: &str,
    period_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (field, value) in [
        ("cash", balances.cash),
        ("accounts_receivable", balances.accounts_receivable),
        ("inventory", balances.inventory),
        ("gross_ppe", balances.gross_ppe),
        ("accumulated_depreciation", balances.accumulated_depreciation),
        ("accounts_payable", balances.accounts_payable),
        ("interest_bearing_debt", balances.interest_bearing_debt),
        ("share_capital", balances.share_capital),
    ] {
        require_non_negative(
            value,
            diagnostics,
            &format!("{pointer}/{field}"),
            period_id,
            field,
        );
    }

    if balances.accumulated_depreciation > balances.gross_ppe {
        push_semantic(
            diagnostics,
            DiagnosticCode::NumericDomainInvalid,
            "Accumulated depreciation must not exceed gross PP&E.",
            format!("{pointer}/accumulated_depreciation"),
            period_id,
            Some("accumulated_depreciation"),
            Some(balances.gross_ppe.to_string()),
            Some(balances.accumulated_depreciation.to_string()),
        );
    }
}

fn require_non_negative(
    value: Decimal,
    diagnostics: &mut Vec<Diagnostic>,
    pointer: &str,
    period_id: Option<&str>,
    field: &str,
) {
    if value < Decimal::ZERO {
        push_semantic(
            diagnostics,
            DiagnosticCode::NumericDomainInvalid,
            format!("{field} must be non-negative in the V1 input contract."),
            pointer,
            period_id,
            Some(field),
            Some(">= 0".to_owned()),
            Some(value.to_string()),
        );
    }
}

fn validate_balance_sheet_identity(
    balances: &BalanceSheet,
    pointer: &str,
    period_id: Option<&str>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let net_ppe = checked_sub(
        balances.gross_ppe,
        balances.accumulated_depreciation,
        diagnostics,
        pointer,
        period_id,
    );

    let total_assets = net_ppe.and_then(|net_ppe| {
        checked_sum(
            &[
                balances.cash,
                balances.accounts_receivable,
                balances.inventory,
                net_ppe,
            ],
            diagnostics,
            pointer,
            period_id,
        )
    });

    let liabilities_and_equity = checked_sum(
        &[
            balances.accounts_payable,
            balances.interest_bearing_debt,
            balances.share_capital,
            balances.retained_earnings,
        ],
        diagnostics,
        pointer,
        period_id,
    );

    if let (Some(total_assets), Some(liabilities_and_equity)) =
        (total_assets, liabilities_and_equity)
    {
        if total_assets != liabilities_and_equity {
            push_semantic(
                diagnostics,
                DiagnosticCode::HistoricalBalanceSheetUnbalanced,
                "Total assets must equal total liabilities plus total equity.",
                pointer,
                period_id,
                Some("total_assets"),
                Some(liabilities_and_equity.to_string()),
                Some(total_assets.to_string()),
            );
        }
    }
}

fn validate_historical_income_statement(
    statement: &IncomeStatement,
    pointer: &str,
    period_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let computed = checked_sub(
        statement.revenue,
        statement.cogs,
        diagnostics,
        pointer,
        Some(period_id),
    )
    .and_then(|value| checked_sub(value, statement.operating_expenses, diagnostics, pointer, Some(period_id)))
    .and_then(|value| checked_sub(value, statement.depreciation, diagnostics, pointer, Some(period_id)))
    .and_then(|value| checked_sub(value, statement.interest_expense, diagnostics, pointer, Some(period_id)))
    .and_then(|value| checked_sub(value, statement.income_tax_expense, diagnostics, pointer, Some(period_id)));

    if let Some(computed) = computed {
        compare_value(
            computed,
            statement.net_income,
            DiagnosticCode::HistoricalNetIncomeMismatch,
            "Reported historical net income does not reconcile to the V1 Income Statement formula.",
            format!("{pointer}/net_income"),
            Some(period_id),
            "net_income",
            diagnostics,
        );
    }
}

fn validate_historical_cash_flow(
    income_statement: &IncomeStatement,
    cash_flow: &CashFlowStatement,
    balance_sheet: &BalanceSheet,
    previous_balance_sheet: &BalanceSheet,
    pointer: &str,
    period_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    compare_value(
        income_statement.net_income,
        cash_flow.net_income,
        DiagnosticCode::HistoricalNetIncomeMismatch,
        "Cash Flow Statement net income must agree with the Income Statement.",
        format!("{pointer}/net_income"),
        Some(period_id),
        "net_income",
        diagnostics,
    );

    compare_value(
        income_statement.depreciation,
        cash_flow.depreciation_addback,
        DiagnosticCode::HistoricalCashFlowReconciliationFailed,
        "Cash Flow Statement depreciation addback must agree with Income Statement depreciation.",
        format!("{pointer}/depreciation_addback"),
        Some(period_id),
        "depreciation_addback",
        diagnostics,
    );

    if let (Some(current_nwc), Some(previous_nwc)) = (
        calculate_nwc(
            balance_sheet,
            diagnostics,
            &format!("/historical_periods/{period_id}/balance_sheet"),
            Some(period_id),
        ),
        calculate_nwc(
            previous_balance_sheet,
            diagnostics,
            "/historical_previous_balance_sheet",
            Some(period_id),
        ),
    ) {
        if let Some(change) = checked_sub(
            current_nwc,
            previous_nwc,
            diagnostics,
            pointer,
            Some(period_id),
        ) {
            compare_value(
                change,
                cash_flow.change_in_operating_nwc,
                DiagnosticCode::HistoricalNwcRollforwardFailed,
                "Reported change in operating NWC must equal the movement between consecutive Balance Sheets.",
                format!("{pointer}/change_in_operating_nwc"),
                Some(period_id),
                "change_in_operating_nwc",
                diagnostics,
            );
        }
    }

    compare_value(
        previous_balance_sheet.cash,
        cash_flow.opening_cash,
        DiagnosticCode::HistoricalCashTieoutFailed,
        "Cash Flow Statement opening cash must equal the preceding Balance Sheet cash.",
        format!("{pointer}/opening_cash"),
        Some(period_id),
        "opening_cash",
        diagnostics,
    );

    let cfo = checked_sum(
        &[cash_flow.net_income, cash_flow.depreciation_addback],
        diagnostics,
        pointer,
        Some(period_id),
    )
    .and_then(|value| {
        checked_sub(
            value,
            cash_flow.change_in_operating_nwc,
            diagnostics,
            pointer,
            Some(period_id),
        )
    });

    if let Some(cfo) = cfo {
        compare_value(
            cfo,
            cash_flow.cash_from_operations,
            DiagnosticCode::HistoricalCashFlowReconciliationFailed,
            "Cash from operations must equal net income plus depreciation addback less the change in operating NWC.",
            format!("{pointer}/cash_from_operations"),
            Some(period_id),
            "cash_from_operations",
            diagnostics,
        );
    }

    if let Some(cfi) = checked_sub(
        Decimal::ZERO,
        cash_flow.capex,
        diagnostics,
        pointer,
        Some(period_id),
    ) {
        compare_value(
            cfi,
            cash_flow.cash_from_investing,
            DiagnosticCode::HistoricalCashFlowReconciliationFailed,
            "Cash from investing must equal negative CapEx in the V1 baseline.",
            format!("{pointer}/cash_from_investing"),
            Some(period_id),
            "cash_from_investing",
            diagnostics,
        );
    }

    let cff = checked_sub(
        cash_flow.debt_drawdowns,
        cash_flow.principal_repayments,
        diagnostics,
        pointer,
        Some(period_id),
    )
    .and_then(|value| {
        checked_sub(
            value,
            cash_flow.dividends,
            diagnostics,
            pointer,
            Some(period_id),
        )
    });

    if let Some(cff) = cff {
        compare_value(
            cff,
            cash_flow.cash_from_financing,
            DiagnosticCode::HistoricalCashFlowReconciliationFailed,
            "Cash from financing must equal debt drawdowns less principal repayments and dividends.",
            format!("{pointer}/cash_from_financing"),
            Some(period_id),
            "cash_from_financing",
            diagnostics,
        );
    }

    let net_change = checked_sum(
        &[
            cash_flow.cash_from_operations,
            cash_flow.cash_from_investing,
            cash_flow.cash_from_financing,
        ],
        diagnostics,
        pointer,
        Some(period_id),
    );

    if let Some(net_change) = net_change {
        compare_value(
            net_change,
            cash_flow.net_change_in_cash,
            DiagnosticCode::HistoricalCashFlowReconciliationFailed,
            "Net change in cash must equal operating, investing, and financing cash flows.",
            format!("{pointer}/net_change_in_cash"),
            Some(period_id),
            "net_change_in_cash",
            diagnostics,
        );

        if let Some(closing_cash) = checked_add(
            cash_flow.opening_cash,
            net_change,
            diagnostics,
            pointer,
            Some(period_id),
        ) {
            compare_value(
                closing_cash,
                cash_flow.closing_cash,
                DiagnosticCode::HistoricalCashFlowReconciliationFailed,
                "Closing cash must equal opening cash plus net change in cash.",
                format!("{pointer}/closing_cash"),
                Some(period_id),
                "closing_cash",
                diagnostics,
            );
        }
    }

    compare_value(
        cash_flow.closing_cash,
        balance_sheet.cash,
        DiagnosticCode::HistoricalCashTieoutFailed,
        "Cash Flow Statement closing cash must agree with Balance Sheet cash.",
        format!("{pointer}/closing_cash"),
        Some(period_id),
        "closing_cash",
        diagnostics,
    );
}

fn validate_historical_rollforwards(
    income_statement: &IncomeStatement,
    balance_sheet: &BalanceSheet,
    cash_flow: &CashFlowStatement,
    previous: &BalanceSheet,
    period_pointer: &str,
    period_id: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let pointer = format!("{period_pointer}/balance_sheet");

    if let Some(expected_gross_ppe) = checked_add(
        previous.gross_ppe,
        cash_flow.capex,
        diagnostics,
        &pointer,
        Some(period_id),
    ) {
        compare_value(
            expected_gross_ppe,
            balance_sheet.gross_ppe,
            DiagnosticCode::HistoricalPpeRollforwardFailed,
            "Gross PP&E closing balance must equal opening gross PP&E plus CapEx under the V1 no-disposal convention.",
            format!("{pointer}/gross_ppe"),
            Some(period_id),
            "gross_ppe",
            diagnostics,
        );
    }

    if let Some(expected_accumulated_depreciation) = checked_add(
        previous.accumulated_depreciation,
        income_statement.depreciation,
        diagnostics,
        &pointer,
        Some(period_id),
    ) {
        compare_value(
            expected_accumulated_depreciation,
            balance_sheet.accumulated_depreciation,
            DiagnosticCode::HistoricalPpeRollforwardFailed,
            "Accumulated depreciation closing balance must equal opening accumulated depreciation plus depreciation.",
            format!("{pointer}/accumulated_depreciation"),
            Some(period_id),
            "accumulated_depreciation",
            diagnostics,
        );
    }

    let debt_available = checked_add(
        previous.interest_bearing_debt,
        cash_flow.debt_drawdowns,
        diagnostics,
        period_pointer,
        Some(period_id),
    );

    if let Some(available) = debt_available {
        if cash_flow.principal_repayments > available {
            push_semantic(
                diagnostics,
                DiagnosticCode::DebtRepaymentExceedsAvailable,
                "Principal repayments must not exceed opening debt plus current-period drawdowns.",
                format!("{period_pointer}/cash_flow_statement/principal_repayments"),
                Some(period_id),
                Some("principal_repayments"),
                Some(available.to_string()),
                Some(cash_flow.principal_repayments.to_string()),
            );
        }

        if let Some(expected_debt) = checked_sub(
            available,
            cash_flow.principal_repayments,
            diagnostics,
            period_pointer,
            Some(period_id),
        ) {
            compare_value(
                expected_debt,
                balance_sheet.interest_bearing_debt,
                DiagnosticCode::HistoricalDebtRollforwardFailed,
                "Debt closing balance must equal opening debt plus drawdowns less principal repayments.",
                format!("{pointer}/interest_bearing_debt"),
                Some(period_id),
                "interest_bearing_debt",
                diagnostics,
            );
        }
    }

    if let Some(expected_retained_earnings) = checked_add(
        previous.retained_earnings,
        income_statement.net_income,
        diagnostics,
        period_pointer,
        Some(period_id),
    )
    .and_then(|value| {
        checked_sub(
            value,
            cash_flow.dividends,
            diagnostics,
            period_pointer,
            Some(period_id),
        )
    }) {
        compare_value(
            expected_retained_earnings,
            balance_sheet.retained_earnings,
            DiagnosticCode::HistoricalRetainedEarningsFailed,
            "Retained earnings closing balance must equal opening retained earnings plus net income less dividends.",
            format!("{pointer}/retained_earnings"),
            Some(period_id),
            "retained_earnings",
            diagnostics,
        );
    }

    compare_value(
        previous.share_capital,
        balance_sheet.share_capital,
        DiagnosticCode::ShareCapitalMovementUnsupported,
        "Share capital must remain unchanged in the V1 model because equity issuance and buybacks are out of scope.",
        format!("{pointer}/share_capital"),
        Some(period_id),
        "share_capital",
        diagnostics,
    );
}

fn validate_forecast_assumptions(model: &ModelInput, diagnostics: &mut Vec<Diagnostic>) {
    let Some(last_historical) = model.historical_periods.last() else {
        return;
    };

    let mut debt = last_historical.balance_sheet.interest_bearing_debt;

    for (index, period) in model.forecast_periods.iter().enumerate() {
        let pointer = format!("/forecast_periods/{index}/assumptions");
        let period_id = period.period_id.as_str();
        let assumptions = &period.assumptions;

        require_minimum(
            assumptions.revenue_growth,
            -Decimal::ONE,
            diagnostics,
            &format!("{pointer}/revenue_growth"),
            period_id,
            "revenue_growth",
        );

        for (field, value) in [
            ("cogs_pct_revenue", assumptions.cogs_pct_revenue),
            (
                "operating_expenses_pct_revenue",
                assumptions.operating_expenses_pct_revenue,
            ),
            ("dso", assumptions.dso),
            ("dio", assumptions.dio),
            ("dpo", assumptions.dpo),
            ("capex_pct_revenue", assumptions.capex_pct_revenue),
            ("debt_drawdowns", assumptions.debt_drawdowns),
            ("principal_repayments", assumptions.principal_repayments),
            ("interest_rate", assumptions.interest_rate),
        ] {
            require_non_negative(
                value,
                diagnostics,
                &format!("{pointer}/{field}"),
                Some(period_id),
                field,
            );
        }

        for (field, value) in [
            ("depreciation_rate", assumptions.depreciation_rate),
            ("tax_rate", assumptions.tax_rate),
            ("dividend_payout_ratio", assumptions.dividend_payout_ratio),
        ] {
            require_between_zero_and_one(
                value,
                diagnostics,
                &format!("{pointer}/{field}"),
                period_id,
                field,
            );
        }

        let available = checked_add(
            debt,
            assumptions.debt_drawdowns,
            diagnostics,
            &pointer,
            Some(period_id),
        );

        if let Some(available) = available {
            if assumptions.principal_repayments > available {
                push_semantic(
                    diagnostics,
                    DiagnosticCode::DebtRepaymentExceedsAvailable,
                    "Forecast principal repayments must not exceed opening debt plus current-period drawdowns.",
                    format!("{pointer}/principal_repayments"),
                    Some(period_id),
                    Some("principal_repayments"),
                    Some(available.to_string()),
                    Some(assumptions.principal_repayments.to_string()),
                );
            }

            if let Some(closing_debt) = checked_sub(
                available,
                assumptions.principal_repayments,
                diagnostics,
                &pointer,
                Some(period_id),
            ) {
                debt = closing_debt;
            }
        }
    }
}

fn require_minimum(
    value: Decimal,
    minimum: Decimal,
    diagnostics: &mut Vec<Diagnostic>,
    pointer: &str,
    period_id: &str,
    field: &str,
) {
    if value < minimum {
        push_semantic(
            diagnostics,
            DiagnosticCode::NumericDomainInvalid,
            format!("{field} is below its permitted V1 minimum."),
            pointer,
            Some(period_id),
            Some(field),
            Some(format!(">= {minimum}")),
            Some(value.to_string()),
        );
    }
}

fn require_between_zero_and_one(
    value: Decimal,
    diagnostics: &mut Vec<Diagnostic>,
    pointer: &str,
    period_id: &str,
    field: &str,
) {
    if value < Decimal::ZERO || value > Decimal::ONE {
        push_semantic(
            diagnostics,
            DiagnosticCode::NumericDomainInvalid,
            format!("{field} must be between 0 and 1 inclusive."),
            pointer,
            Some(period_id),
            Some(field),
            Some("0 <= value <= 1".to_owned()),
            Some(value.to_string()),
        );
    }
}

fn calculate_nwc(
    balances: &BalanceSheet,
    diagnostics: &mut Vec<Diagnostic>,
    pointer: &str,
    period_id: Option<&str>,
) -> Option<Decimal> {
    checked_sum(
        &[balances.accounts_receivable, balances.inventory],
        diagnostics,
        pointer,
        period_id,
    )
    .and_then(|value| {
        checked_sub(
            value,
            balances.accounts_payable,
            diagnostics,
            pointer,
            period_id,
        )
    })
}

fn compare_value(
    expected: Decimal,
    actual: Decimal,
    code: DiagnosticCode,
    message: &str,
    pointer: String,
    period_id: Option<&str>,
    field: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if expected != actual {
        push_semantic(
            diagnostics,
            code,
            message,
            pointer,
            period_id,
            Some(field),
            Some(expected.to_string()),
            Some(actual.to_string()),
        );
    }
}

fn checked_sum(
    values: &[Decimal],
    diagnostics: &mut Vec<Diagnostic>,
    pointer: &str,
    period_id: Option<&str>,
) -> Option<Decimal> {
    values.iter().try_fold(Decimal::ZERO, |total, value| {
        checked_add(total, *value, diagnostics, pointer, period_id)
    })
}

fn checked_add(
    left: Decimal,
    right: Decimal,
    diagnostics: &mut Vec<Diagnostic>,
    pointer: &str,
    period_id: Option<&str>,
) -> Option<Decimal> {
    left.checked_add(right).or_else(|| {
        push_semantic(
            diagnostics,
            DiagnosticCode::DecimalOverflow,
            "Checked Decimal addition exceeded the supported numeric representation.",
            pointer,
            period_id,
            None,
            None,
            Some(format!("{left} + {right}")),
        );
        None
    })
}

fn checked_sub(
    left: Decimal,
    right: Decimal,
    diagnostics: &mut Vec<Diagnostic>,
    pointer: &str,
    period_id: Option<&str>,
) -> Option<Decimal> {
    left.checked_sub(right).or_else(|| {
        push_semantic(
            diagnostics,
            DiagnosticCode::DecimalOverflow,
            "Checked Decimal subtraction exceeded the supported numeric representation.",
            pointer,
            period_id,
            None,
            None,
            Some(format!("{left} - {right}")),
        );
        None
    })
}
