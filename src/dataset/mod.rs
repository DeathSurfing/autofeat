//! Dataset loading, schema discovery, and statistical profiling.

use std::fs::File;
use std::path::Path;

use polars::prelude::*;

pub mod schema;
pub mod stats;

/// A loaded dataset with its schema and column statistics.
pub struct Dataset {
    /// Original file name.
    pub name: String,
    /// Polars DataFrame.
    pub df: DataFrame,
    /// Per-column metadata.
    pub columns: Vec<ColumnInfo>,
}

/// Metadata about a single column.
pub struct ColumnInfo {
    /// Column name.
    pub name: String,
    /// Dtype as a display string.
    pub dtype: String,
    /// Number of null values.
    pub null_count: usize,
    /// Number of non-null values.
    pub non_null_count: usize,
    /// Percentage of nulls (0–100).
    pub null_pct: f64,
    /// Statistical summary (only for numeric columns).
    pub stats: Option<stats::ColumnStats>,
}

impl Dataset {
    /// Load a CSV file as a dataset.
    pub fn from_csv(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let df = CsvReader::new(file).finish()?;
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "unknown".into());
        Ok(Self::from_df(df, name))
    }

    /// Wrap an existing DataFrame as a dataset.
    pub fn from_df(df: DataFrame, name: String) -> Self {
        let columns = Self::compute_columns(&df);
        Self { name, df, columns }
    }

    fn compute_columns(df: &DataFrame) -> Vec<ColumnInfo> {
        let schema = df.schema();
        let mut infos = Vec::new();
        for (name, dtype) in schema.iter() {
            let column = df.column(name).ok();
            let total = df.height();
            let null_count = column
                .and_then(|c| c.as_series())
                .map(|s| s.null_count())
                .unwrap_or(0);
            let non_null_count = total - null_count;
            let null_pct = if total > 0 {
                null_count as f64 / total as f64 * 100.0
            } else {
                0.0
            };
            let stats = if dtype.is_numeric() {
                column.and_then(stats::ColumnStats::from_column)
            } else {
                None
            };
            infos.push(ColumnInfo {
                name: name.to_string(),
                dtype: format!("{:?}", dtype),
                null_count,
                non_null_count,
                null_pct,
                stats,
            });
        }
        infos
    }
}
