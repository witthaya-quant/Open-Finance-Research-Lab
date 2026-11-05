use crate::diagnostics::{Diagnostic, DiagnosticCategory, DiagnosticCode};
use crate::domain::{parse_decimal_text, DecimalTextError};
use serde_json::Value;

#[derive(Clone, Copy)]
enum Shape {
    String,
    U16,
    Decimal,
    Enum(&'static [&'static str]),
    Object(&'static [Field]),
    Array(&'static Shape),
}

struct Field {
    name: &'static str,
    shape: &'static Shape,
}

static STRING: Shape = Shape::String;
static U16: Shape = Shape::U16;
static DECIMAL: Shape = Shape::Decimal;

static UNIT_VALUES: &[&str] = &["units", "thousands", "millions"];
static SOURCE_VALUES: &[&str] = &["synthetic"];
static UNIT: Shape = Shape::Enum(UNIT_VALUES);
static SOURCE: Shape = Shape::Enum(SOURCE_VALUES);

static MODEL_FIELDS: &[Field] = &[
    Field {
        name: "model_id",
        shape: &STRING,
    },
    Field {
        name: "model_name",
        shape: &STRING,
    },
    Field {
        name: "reporting_currency",
        shape: &STRING,
    },
    Field {
        name: "unit_scale",
        shape: &UNIT,
    },
    Field {
        name: "day_count_basis",
        shape: &U16,
    },
];
static MODEL: Shape = Shape::Object(MODEL_FIELDS);

static BS_FIELDS: &[Field] = &[
    Field {
        name: "cash",
        shape: &DECIMAL,
    },
    Field {
        name: "accounts_receivable",
        shape: &DECIMAL,
    },
    Field {
        name: "inventory",
        shape: &DECIMAL,
    },
    Field {
        name: "gross_ppe",
        shape: &DECIMAL,
    },
    Field {
        name: "accumulated_depreciation",
        shape: &DECIMAL,
    },
    Field {
        name: "accounts_payable",
        shape: &DECIMAL,
    },
    Field {
        name: "interest_bearing_debt",
        shape: &DECIMAL,
    },
    Field {
        name: "share_capital",
        shape: &DECIMAL,
    },
    Field {
        name: "retained_earnings",
        shape: &DECIMAL,
    },
];
static BS: Shape = Shape::Object(BS_FIELDS);

static OPENING_FIELDS: &[Field] = &[
    Field {
        name: "as_of_date",
        shape: &STRING,
    },
    Field {
        name: "balances",
        shape: &BS,
    },
];
static OPENING: Shape = Shape::Object(OPENING_FIELDS);

static SOURCE_FIELDS: &[Field] = &[
    Field {
        name: "classification",
        shape: &SOURCE,
    },
    Field {
        name: "source_reference",
        shape: &STRING,
    },
];
static HIST_SOURCE: Shape = Shape::Object(SOURCE_FIELDS);

static IS_FIELDS: &[Field] = &[
    Field {
        name: "revenue",
        shape: &DECIMAL,
    },
    Field {
        name: "cogs",
        shape: &DECIMAL,
    },
    Field {
        name: "operating_expenses",
        shape: &DECIMAL,
    },
    Field {
        name: "depreciation",
        shape: &DECIMAL,
    },
    Field {
        name: "interest_expense",
        shape: &DECIMAL,
    },
    Field {
        name: "income_tax_expense",
        shape: &DECIMAL,
    },
    Field {
        name: "net_income",
        shape: &DECIMAL,
    },
];
static IS: Shape = Shape::Object(IS_FIELDS);

static CF_FIELDS: &[Field] = &[
    Field {
        name: "net_income",
        shape: &DECIMAL,
    },
    Field {
        name: "depreciation_addback",
        shape: &DECIMAL,
    },
    Field {
        name: "change_in_operating_nwc",
        shape: &DECIMAL,
    },
    Field {
        name: "cash_from_operations",
        shape: &DECIMAL,
    },
    Field {
        name: "capex",
        shape: &DECIMAL,
    },
    Field {
        name: "cash_from_investing",
        shape: &DECIMAL,
    },
    Field {
        name: "debt_drawdowns",
        shape: &DECIMAL,
    },
    Field {
        name: "principal_repayments",
        shape: &DECIMAL,
    },
    Field {
        name: "dividends",
        shape: &DECIMAL,
    },
    Field {
        name: "cash_from_financing",
        shape: &DECIMAL,
    },
    Field {
        name: "net_change_in_cash",
        shape: &DECIMAL,
    },
    Field {
        name: "opening_cash",
        shape: &DECIMAL,
    },
    Field {
        name: "closing_cash",
        shape: &DECIMAL,
    },
];
static CF: Shape = Shape::Object(CF_FIELDS);

static HIST_FIELDS: &[Field] = &[
    Field {
        name: "period_id",
        shape: &STRING,
    },
    Field {
        name: "start_date",
        shape: &STRING,
    },
    Field {
        name: "end_date",
        shape: &STRING,
    },
    Field {
        name: "source",
        shape: &HIST_SOURCE,
    },
    Field {
        name: "income_statement",
        shape: &IS,
    },
    Field {
        name: "balance_sheet",
        shape: &BS,
    },
    Field {
        name: "cash_flow_statement",
        shape: &CF,
    },
];
static HIST: Shape = Shape::Object(HIST_FIELDS);
static HIST_ARRAY: Shape = Shape::Array(&HIST);

static ASSUMPTION_FIELDS: &[Field] = &[
    Field {
        name: "revenue_growth",
        shape: &DECIMAL,
    },
    Field {
        name: "cogs_pct_revenue",
        shape: &DECIMAL,
    },
    Field {
        name: "operating_expenses_pct_revenue",
        shape: &DECIMAL,
    },
    Field {
        name: "dso",
        shape: &DECIMAL,
    },
    Field {
        name: "dio",
        shape: &DECIMAL,
    },
    Field {
        name: "dpo",
        shape: &DECIMAL,
    },
    Field {
        name: "capex_pct_revenue",
        shape: &DECIMAL,
    },
    Field {
        name: "depreciation_rate",
        shape: &DECIMAL,
    },
    Field {
        name: "debt_drawdowns",
        shape: &DECIMAL,
    },
    Field {
        name: "principal_repayments",
        shape: &DECIMAL,
    },
    Field {
        name: "interest_rate",
        shape: &DECIMAL,
    },
    Field {
        name: "tax_rate",
        shape: &DECIMAL,
    },
    Field {
        name: "dividend_payout_ratio",
        shape: &DECIMAL,
    },
];
static ASSUMPTIONS: Shape = Shape::Object(ASSUMPTION_FIELDS);

static FORECAST_FIELDS: &[Field] = &[
    Field {
        name: "period_id",
        shape: &STRING,
    },
    Field {
        name: "start_date",
        shape: &STRING,
    },
    Field {
        name: "end_date",
        shape: &STRING,
    },
    Field {
        name: "assumptions",
        shape: &ASSUMPTIONS,
    },
];
static FORECAST: Shape = Shape::Object(FORECAST_FIELDS);
static FORECAST_ARRAY: Shape = Shape::Array(&FORECAST);

static ROOT_FIELDS: &[Field] = &[
    Field {
        name: "schema_version",
        shape: &STRING,
    },
    Field {
        name: "model",
        shape: &MODEL,
    },
    Field {
        name: "opening_balance_sheet",
        shape: &OPENING,
    },
    Field {
        name: "historical_periods",
        shape: &HIST_ARRAY,
    },
    Field {
        name: "forecast_periods",
        shape: &FORECAST_ARRAY,
    },
];
static ROOT: Shape = Shape::Object(ROOT_FIELDS);

struct Failure {
    code: DiagnosticCode,
    category: DiagnosticCategory,
    message: &'static str,
    expected: Option<String>,
    actual: Option<String>,
    path: Vec<String>,
}

pub(crate) fn validate_model_input_shape(value: &Value) -> Result<(), Diagnostic> {
    let mut path = Vec::new();
    validate_value(value, &ROOT, &mut path).map_err(|failure| failure.into_diagnostic(value))
}

impl Failure {
    fn into_diagnostic(self, root: &Value) -> Diagnostic {
        Diagnostic::error(
            self.code,
            self.category,
            self.message,
            json_pointer(&self.path),
            period_id_for_path(root, &self.path),
            field_for_path(&self.path),
            self.expected,
            self.actual,
        )
    }
}

fn validate_value(value: &Value, shape: &Shape, path: &mut Vec<String>) -> Result<(), Failure> {
    match shape {
        Shape::String => {
            if value.is_string() {
                Ok(())
            } else {
                Err(fail(
                    DiagnosticCode::JsonTypeMismatch,
                    DiagnosticCategory::InputSyntax,
                    "A field has the wrong JSON type.",
                    Some("string"),
                    Some(json_type(value)),
                    path,
                ))
            }
        }
        Shape::U16 => {
            if value
                .as_u64()
                .is_some_and(|number| number <= u64::from(u16::MAX))
            {
                Ok(())
            } else {
                Err(fail(
                    DiagnosticCode::JsonTypeMismatch,
                    DiagnosticCategory::InputSyntax,
                    "A field has the wrong JSON type or integer range.",
                    Some("unsigned 16-bit integer"),
                    Some(json_type(value)),
                    path,
                ))
            }
        }
        Shape::Decimal => {
            let Some(text) = value.as_str() else {
                return Err(fail(
                    DiagnosticCode::DecimalStringRequired,
                    DiagnosticCategory::InputSyntax,
                    "A Decimal field must be supplied as a JSON string token.",
                    Some("canonical Decimal string"),
                    Some(json_type(value)),
                    path,
                ));
            };

            match parse_decimal_text(text) {
                Ok(_) => Ok(()),
                Err(error) => {
                    let (code, message, expected) = match error {
                        DecimalTextError::NonCanonical => (
                            DiagnosticCode::DecimalTextNonCanonical,
                            "A Decimal string does not use the canonical base-10 grammar.",
                            "canonical base-10 Decimal notation",
                        ),
                        DecimalTextError::ScaleExceeded => (
                            DiagnosticCode::DecimalScaleExceeded,
                            "A Decimal string exceeds the maximum supported fractional scale.",
                            "at most 28 fractional digits",
                        ),
                        DecimalTextError::OutOfRange => (
                            DiagnosticCode::DecimalOutOfRange,
                            "A Decimal string is outside the supported representation or loses information when parsed.",
                            "exact rust_decimal representation",
                        ),
                    };

                    Err(fail(
                        code,
                        DiagnosticCategory::InputSyntax,
                        message,
                        Some(expected),
                        Some(text),
                        path,
                    ))
                }
            }
        }
        Shape::Enum(variants) => {
            let Some(actual) = value.as_str() else {
                return Err(fail(
                    DiagnosticCode::JsonTypeMismatch,
                    DiagnosticCategory::InputSyntax,
                    "An enumeration field has the wrong JSON type.",
                    Some("string"),
                    Some(json_type(value)),
                    path,
                ));
            };

            if variants.contains(&actual) {
                Ok(())
            } else {
                Err(Failure {
                    code: DiagnosticCode::InvalidEnumValue,
                    category: DiagnosticCategory::Schema,
                    message: "The input contains a value outside the declared enumeration.",
                    expected: Some(variants.join(", ")),
                    actual: Some(actual.to_owned()),
                    path: path.to_vec(),
                })
            }
        }
        Shape::Object(fields) => {
            let Some(object) = value.as_object() else {
                return Err(fail(
                    DiagnosticCode::JsonTypeMismatch,
                    DiagnosticCategory::InputSyntax,
                    "An object field has the wrong JSON type.",
                    Some("object"),
                    Some(json_type(value)),
                    path,
                ));
            };

            let mut unexpected = object
                .keys()
                .filter(|key| !fields.iter().any(|field| field.name == key.as_str()))
                .collect::<Vec<_>>();
            unexpected.sort_unstable();

            if let Some(key) = unexpected.first() {
                path.push((*key).to_string());
                let result = fail(
                    DiagnosticCode::UnknownField,
                    DiagnosticCategory::Schema,
                    "The input contains a field that is not declared in this schema version.",
                    Some("declared V1 model property"),
                    Some(key),
                    path,
                );
                path.pop();
                return Err(result);
            }

            for field in *fields {
                if !object.contains_key(field.name) {
                    path.push(field.name.to_owned());
                    let result = fail(
                        DiagnosticCode::RequiredFieldMissing,
                        DiagnosticCategory::Schema,
                        "The input is missing a required field.",
                        Some("required property"),
                        None,
                        path,
                    );
                    path.pop();
                    return Err(result);
                }
            }

            for field in *fields {
                if let Some(child) = object.get(field.name) {
                    path.push(field.name.to_owned());
                    let result = validate_value(child, field.shape, path);
                    path.pop();
                    result?;
                }
            }

            Ok(())
        }
        Shape::Array(item_shape) => {
            let Some(items) = value.as_array() else {
                return Err(fail(
                    DiagnosticCode::JsonTypeMismatch,
                    DiagnosticCategory::InputSyntax,
                    "An array field has the wrong JSON type.",
                    Some("array"),
                    Some(json_type(value)),
                    path,
                ));
            };

            for (index, item) in items.iter().enumerate() {
                path.push(index.to_string());
                let result = validate_value(item, item_shape, path);
                path.pop();
                result?;
            }

            Ok(())
        }
    }
}

fn fail(
    code: DiagnosticCode,
    category: DiagnosticCategory,
    message: &'static str,
    expected: Option<&str>,
    actual: Option<&str>,
    path: &[String],
) -> Failure {
    Failure {
        code,
        category,
        message,
        expected: expected.map(str::to_owned),
        actual: actual.map(str::to_owned),
        path: path.to_vec(),
    }
}

fn json_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn json_pointer(path: &[String]) -> Option<String> {
    let mut pointer = String::new();
    for segment in path {
        pointer.push('/');
        pointer.push_str(&segment.replace('~', "~0").replace('/', "~1"));
    }
    Some(pointer)
}

fn field_for_path(path: &[String]) -> Option<String> {
    let is_period_array_index = path.len() >= 2
        && matches!(
            path[path.len() - 2].as_str(),
            "historical_periods" | "forecast_periods"
        )
        && path[path.len() - 1].parse::<usize>().is_ok();

    let index = if is_period_array_index {
        path.len().checked_sub(2)
    } else {
        path.len().checked_sub(1)
    }?;

    path.get(index).cloned()
}

fn period_id_for_path(root: &Value, path: &[String]) -> Option<String> {
    for pair in path.windows(2) {
        if pair[0].as_str() != "historical_periods"
            && pair[0].as_str() != "forecast_periods"
        {
            continue;
        }

        let index = pair[1].parse::<usize>().ok()?;
        return root
            .get(pair[0].as_str())?
            .as_array()?
            .get(index)?
            .get("period_id")?
            .as_str()
            .map(str::to_owned);
    }
    None
}
