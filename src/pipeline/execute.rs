//! Execute the full workflow pipeline via featrs.

use std::time::Instant;

use polars::prelude::*;

use crate::workflow::graph::WorkflowGraph;
use crate::workflow::node::NodeKind;

/// Result of a single pipeline execution.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether the pipeline completed without errors.
    pub success: bool,
    /// Number of rows in the output DataFrame.
    pub output_rows: usize,
    /// Number of columns in the output DataFrame.
    pub output_cols: usize,
    /// Status message or error description.
    pub message: String,
    /// Execution time in milliseconds.
    pub duration_ms: u64,
}

/// Run the workflow pipeline on the given DataFrame.
pub fn run_pipeline(workflow: &WorkflowGraph, df: &DataFrame) -> ExecutionResult {
    let start = Instant::now();
    let mut current = cast_numeric_to_float64(df);

    for node in &workflow.nodes {
        if !node.enabled {
            continue;
        }
        match apply_transform(node.kind, &current) {
            Ok(result) => current = result,
            Err(e) => {
                let elapsed = start.elapsed().as_millis() as u64;
                return ExecutionResult {
                    success: false,
                    output_rows: current.height(),
                    output_cols: current.width(),
                    message: format!("{}: {}", node.kind.label(), e),
                    duration_ms: elapsed,
                };
            }
        }
    }

    let elapsed = start.elapsed().as_millis() as u64;
    ExecutionResult {
        success: true,
        output_rows: current.height(),
        output_cols: current.width(),
        message: "Pipeline completed successfully.".into(),
        duration_ms: elapsed,
    }
}

fn apply_transform(
    kind: NodeKind,
    df: &DataFrame,
) -> Result<DataFrame, Box<dyn std::error::Error>> {
    use featrs::prelude::*;

    match kind {
        NodeKind::MedianImputer => {
            let (num_df, num_names) = filter_float64_columns(df);
            if num_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut imputer = SimpleImputer::new(Strategy::Median);
            imputer.fit(num_df.clone())?;
            let filled = imputer.transform(num_df)?;
            Ok(merge_columns(df, &filled, &num_names))
        }
        NodeKind::MeanImputer => {
            let (num_df, num_names) = filter_float64_columns(df);
            if num_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut imputer = SimpleImputer::new(Strategy::Mean);
            imputer.fit(num_df.clone())?;
            let filled = imputer.transform(num_df)?;
            Ok(merge_columns(df, &filled, &num_names))
        }

        NodeKind::RobustScaler => {
            let (num_df, num_names) = filter_float64_columns(df);
            if num_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut scaler = RobustScaler::new();
            scaler.fit(num_df.clone())?;
            let scaled = scaler.transform(num_df)?;
            Ok(merge_columns(df, &scaled, &num_names))
        }
        NodeKind::StandardScaler => {
            let (num_df, num_names) = filter_float64_columns(df);
            if num_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut scaler = StandardScaler::new();
            scaler.fit(num_df.clone())?;
            let scaled = scaler.transform(num_df)?;
            Ok(merge_columns(df, &scaled, &num_names))
        }

        NodeKind::OneHotEncoder => {
            let (string_df, string_names) = filter_string_columns(df);
            if string_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut encoder = OneHotEncoder::new();
            encoder.fit(string_df.clone())?;
            let encoded = encoder.transform(string_df)?;
            let mut result = drop_columns(df, &string_names);
            for ec in encoded.columns() {
                let _ = result.hstack_mut(std::slice::from_ref(ec));
            }
            Ok(result)
        }

        NodeKind::FrequencyEncoder => {
            let (string_df, string_names) = filter_string_columns(df);
            if string_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut encoder = OrdinalEncoder::new();
            encoder.fit(string_df.clone())?;
            let encoded = encoder.transform(string_df)?;
            let mut result = drop_columns(df, &string_names);
            for ec in encoded.columns() {
                let _ = result.hstack_mut(std::slice::from_ref(ec));
            }
            Ok(result)
        }

        NodeKind::PolynomialFeatures => {
            let (num_df, num_names) = filter_float64_columns(df);
            if num_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut pf = PolynomialFeatures::new(2)?;
            pf.fit(num_df.clone())?;
            let expanded = pf.transform(num_df)?;
            // Keep non-numeric columns as-is, add polynomial output
            let mut result = drop_columns(df, &num_names);
            for ec in expanded.columns() {
                // expanded already includes the original columns plus new terms
                let name = ec.name().to_string();
                if result.column(&name).is_ok() {
                    // Column already in result (shouldn't happen after drop)
                    continue;
                }
                let _ = result.hstack_mut(std::slice::from_ref(ec));
            }
            Ok(result)
        }

        NodeKind::TargetEncoder => Err("TargetEncoder requires a target column — skipped".into()),

        NodeKind::DatetimeFeatures => Err("DatetimeFeatures not yet implemented — skipped".into()),
    }
}

// ── Column helpers ──

/// Filter columns that are Float64, returning the filtered DF and their names.
fn filter_float64_columns(df: &DataFrame) -> (DataFrame, Vec<String>) {
    let mut names = Vec::new();
    let cols: Vec<Column> = df
        .columns()
        .iter()
        .filter_map(|col| {
            let s = col.as_series()?;
            if matches!(s.dtype(), DataType::Float64) {
                names.push(s.name().to_string());
                Some(col.clone())
            } else {
                None
            }
        })
        .collect();
    if names.is_empty() {
        return (DataFrame::default(), Vec::new());
    }
    (DataFrame::new(df.height(), cols).unwrap_or_default(), names)
}

/// Filter columns that are String, returning the filtered DF and their names.
fn filter_string_columns(df: &DataFrame) -> (DataFrame, Vec<String>) {
    let mut names = Vec::new();
    let cols: Vec<Column> = df
        .columns()
        .iter()
        .filter_map(|col| {
            let s = col.as_series()?;
            if matches!(s.dtype(), DataType::String) {
                names.push(s.name().to_string());
                Some(col.clone())
            } else {
                None
            }
        })
        .collect();
    if names.is_empty() {
        return (DataFrame::default(), Vec::new());
    }
    (DataFrame::new(df.height(), cols).unwrap_or_default(), names)
}

/// Drop named columns from a DataFrame and return the result.
fn drop_columns(df: &DataFrame, names: &[String]) -> DataFrame {
    let mut result = df.clone();
    for name in names {
        let _ = result.drop_in_place(name);
    }
    result
}

/// Replace a subset of columns in `df` with the transformed versions from `transformed`.
fn merge_columns(original: &DataFrame, transformed: &DataFrame, names: &[String]) -> DataFrame {
    let mut result = drop_columns(original, names);
    for name in names {
        if let Ok(col) = transformed.column(name) {
            let _ = result.hstack_mut(std::slice::from_ref(col));
        }
    }
    result
}

/// Cast all numeric integer columns to Float64 so featrs transformers can process them.
fn cast_numeric_to_float64(df: &DataFrame) -> DataFrame {
    let height = df.height();
    let cols: Vec<Column> = df
        .columns()
        .iter()
        .map(|col| match col.as_series() {
            Some(s) => match s.dtype() {
                DataType::Int8
                | DataType::Int16
                | DataType::Int32
                | DataType::Int64
                | DataType::UInt8
                | DataType::UInt16
                | DataType::UInt32
                | DataType::UInt64
                | DataType::Float32 => s
                    .cast(&DataType::Float64)
                    .ok()
                    .map(|s| s.into_column())
                    .unwrap_or(col.clone()),
                _ => col.clone(),
            },
            None => col.clone(),
        })
        .collect();
    DataFrame::new(height, cols).unwrap_or_else(|_| df.clone())
}
