//! Statistical profiling — distributions, skew, outliers, summary statistics.

use polars::prelude::*;

/// Summary statistics for a numeric column.
#[derive(Debug, Clone)]
pub struct ColumnStats {
    /// Number of non-null values.
    pub count: usize,
    /// Arithmetic mean.
    pub mean: f64,
    /// Sample standard deviation.
    pub std: f64,
    /// Minimum value.
    pub min: f64,
    /// Maximum value.
    pub max: f64,
}

impl ColumnStats {
    /// Compute stats from a numeric column.
    pub fn from_column(column: &Column) -> Option<Self> {
        let s = column.as_series()?;
        let count = s.len() - s.null_count();
        let mean = s.mean()?;
        let std = s.std(1)?;
        let min = s.min::<f64>().ok()??;
        let max = s.max::<f64>().ok()??;
        Some(Self {
            count,
            mean,
            std,
            min,
            max,
        })
    }
}
