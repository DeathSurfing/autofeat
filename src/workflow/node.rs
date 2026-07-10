//! Node types representing individual preprocessing steps.

/// The kind of transformation a node represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    /// Fill missing values with the median.
    MedianImputer,
    /// Fill missing values with the mean.
    MeanImputer,
    /// Scale using median and IQR (outlier-robust).
    RobustScaler,
    /// Z-score normalization (mean 0, variance 1).
    StandardScaler,
    /// Create binary dummy columns for categories.
    OneHotEncoder,
    /// Encode categories by target mean.
    TargetEncoder,
    /// Encode categories by their frequency.
    FrequencyEncoder,
    /// Generate polynomial and interaction features.
    PolynomialFeatures,
    /// Extract datetime components (hour, month, etc.).
    DatetimeFeatures,
}

impl NodeKind {
    /// All available node kinds in display order.
    pub const ALL: &[NodeKind] = &[
        NodeKind::MedianImputer,
        NodeKind::MeanImputer,
        NodeKind::RobustScaler,
        NodeKind::StandardScaler,
        NodeKind::OneHotEncoder,
        NodeKind::TargetEncoder,
        NodeKind::FrequencyEncoder,
        NodeKind::PolynomialFeatures,
        NodeKind::DatetimeFeatures,
    ];

    /// Display label for the node kind.
    pub fn label(&self) -> &str {
        match self {
            NodeKind::MedianImputer => "Median Imputer",
            NodeKind::MeanImputer => "Mean Imputer",
            NodeKind::RobustScaler => "Robust Scaler",
            NodeKind::StandardScaler => "Standard Scaler",
            NodeKind::OneHotEncoder => "One Hot Encoder",
            NodeKind::TargetEncoder => "Target Encoder",
            NodeKind::FrequencyEncoder => "Frequency Encoder",
            NodeKind::PolynomialFeatures => "Polynomial Features",
            NodeKind::DatetimeFeatures => "Datetime Features",
        }
    }
}

/// A single node in the workflow.
#[derive(Debug, Clone)]
pub struct Node {
    /// Unique identifier.
    pub id: usize,
    /// The kind of transformation this node represents.
    pub kind: NodeKind,
    /// Whether the node is active in the pipeline.
    pub enabled: bool,
}

impl Node {
    /// Create a new node with the given kind and id.
    pub fn new(kind: NodeKind, id: usize) -> Self {
        Self {
            id,
            kind,
            enabled: true,
        }
    }
}
