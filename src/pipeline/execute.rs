//! Execute the full workflow pipeline via featrs.

use std::time::Instant;

use polars::prelude::DataFrame;

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
    let mut current = df.clone();

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
            let mut encoder = OneHotEncoder::new();
            encoder.fit(df.clone())?;
            encoder.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::TargetEncoder => {
            // TargetEncoder needs a target column — skip for now
            Err("TargetEncoder requires a target column — not implemented yet".into())
        }
        NodeKind::FrequencyEncoder => {
            // Use OrdinalEncoder as a stand-in
            let mut encoder = OrdinalEncoder::new();
            encoder.fit(df.clone())?;
            encoder.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::PolynomialFeatures => {
            let mut pf = PolynomialFeatures::new(2)?;
            pf.fit(df.clone())?;
            pf.transform(df.clone()).map_err(Into::into)
        }
        NodeKind::DatetimeFeatures => Err("DatetimeFeatures not yet implemented in featrs".into()),
    }
}
