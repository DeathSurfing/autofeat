//! Prompt templates for the AI planning agent.

use crate::dataset::Dataset;

/// Build the system prompt describing available tools and the dataset.
pub fn system_prompt(dataset: Option<&Dataset>) -> String {
    let mut prompt = String::from(
        "You are a feature engineering assistant. Your job is to help users build preprocessing pipelines.\n\n",
    );

    if let Some(ds) = dataset {
        prompt.push_str("## Loaded Dataset\n\n");
        prompt.push_str(&format!("- File: {}\n", ds.name));
        prompt.push_str(&format!("- Rows: {}\n", ds.df.height()));
        prompt.push_str(&format!("- Columns: {}\n\n", ds.columns.len()));

        prompt.push_str("### Schema\n\n");
        for col in &ds.columns {
            let nulls = if col.null_count > 0 {
                format!(" ({} nulls)", col.null_count)
            } else {
                String::new()
            };
            if let Some(ref stats) = col.stats {
                prompt.push_str(&format!(
                    "- {} [{}]  {}  mean={:.2}  std={:.2}  min={:.2}  max={:.2}{}\n",
                    col.name,
                    col.dtype,
                    stats.count,
                    stats.mean,
                    stats.std,
                    stats.min,
                    stats.max,
                    nulls
                ));
            } else {
                prompt.push_str(&format!(
                    "- {} [{}]{} non-null={}{}\n",
                    col.name, col.dtype, nulls, col.non_null_count, nulls
                ));
            }
        }
        prompt.push('\n');
    }

    prompt.push_str(
        "## Available Transformations\n\n\
         - MedianImputer: Fill missing numeric values with median\n\
         - MeanImputer: Fill missing numeric values with mean\n\
         - RobustScaler: Scale using median and IQR (outlier-robust)\n\
         - StandardScaler: Z-score normalization (mean=0, var=1)\n\
         - OneHotEncoder: Create binary dummy columns for categories\n\
         - FrequencyEncoder: Encode categories by frequency\n\
         - PolynomialFeatures: Generate polynomial and interaction features (degree 2)\n\n\
         ## Response Format\n\n\
         First, explain your reasoning. Then, suggest a pipeline using lines like:\n\n\
         ```\n\
         ADD MedianImputer\n\
         ADD RobustScaler\n\
         ```\n\n\
         You can also remove nodes with:\n\
         REMOVE <NodeKind>\n\n\
         The pipeline order matters. Consider: imputation → scaling → encoding → feature generation.\n"
    );

    prompt
}
