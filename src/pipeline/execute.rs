//! Execute the full workflow pipeline via featrs.

use std::time::Instant;

use polars::prelude::*;

use crate::workflow::graph::WorkflowGraph;
use crate::workflow::node::NodeKind;

/// Result of a single pipeline execution.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Whether the execution succeeded.
    pub success: bool,
    /// Output row count.
    pub output_rows: usize,
    /// Output column count.
    pub output_cols: usize,
    /// Status or error message.
    pub message: String,
    /// Duration in milliseconds.
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
            let mut imputer = SimpleImputer::new(Strategy::Median);
            imputer.fit(df.clone())?;
            imputer.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::MeanImputer => {
            let mut imputer = SimpleImputer::new(Strategy::Mean);
            imputer.fit(df.clone())?;
            imputer.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::RobustScaler => {
            let mut scaler = RobustScaler::new();
            scaler.fit(df.clone())?;
            scaler.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::StandardScaler => {
            let mut scaler = StandardScaler::new();
            scaler.fit(df.clone())?;
            scaler.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::OneHotEncoder => {
            let (string_df, string_names) = filter_string_columns(df);
            if string_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut encoder = OneHotEncoder::new();
            encoder.fit(string_df.clone())?;
            let encoded = encoder.transform(string_df)?;
            let mut result = df.clone();
            for name in &string_names {
                let _ = result.drop_in_place(name);
            }
            for ec in encoded.columns() {
                let _ = result.hstack_mut(std::slice::from_ref(ec));
            }
            Ok(result)
        }
        NodeKind::TargetEncoder => {
            Err("TargetEncoder requires a target column — not implemented yet".into())
        }
        NodeKind::FrequencyEncoder => {
            let (string_df, string_names) = filter_string_columns(df);
            if string_df.width() == 0 {
                return Ok(df.clone());
            }
            let mut encoder = OrdinalEncoder::new();
            encoder.fit(string_df.clone())?;
            let encoded = encoder.transform(string_df)?;
            let mut result = df.clone();
            for name in &string_names {
                let _ = result.drop_in_place(name);
            }
            for ec in encoded.columns() {
                let _ = result.hstack_mut(std::slice::from_ref(ec));
            }
            Ok(result)
        }
        NodeKind::PolynomialFeatures => {
            let mut pf = PolynomialFeatures::new(2)?;
            pf.fit(df.clone())?;
            pf.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::DatetimeFeatures => Err("DatetimeFeatures not yet implemented in featrs".into()),
    }
}

/// Filter a DataFrame to only String columns, returning the filtered DF and column names.
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
    let filtered = DataFrame::new(df.height(), cols).unwrap_or_default();
    (filtered, names)
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
