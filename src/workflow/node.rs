//! Node types representing individual preprocessing steps.

/// The kind of transformation a node represents.
pub enum NodeKind {
    /// MedianImputer
    MedianImputer,
    /// MeanImputer
    MeanImputer,
    /// RobustScaler
    RobustScaler,
    /// StandardScaler
    StandardScaler,
    /// OneHotEncoder
    OneHotEncoder,
    /// TargetEncoder
    TargetEncoder,
    /// FrequencyEncoder
    FrequencyEncoder,
    /// PolynomialFeatures
    PolynomialFeatures,
    /// DatetimeFeatures
    DatetimeFeatures,
}

/// A single node in the workflow DAG.
pub struct Node {
    /// The kind of transformation this node represents.
    pub kind: NodeKind,
    /// Whether the node is active in the pipeline.
    pub enabled: bool,
}
