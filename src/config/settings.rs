//! Serializable user settings (General, LLM, Pipeline, Evaluation, Diagnostics).

use serde::{Deserialize, Serialize};

/// LLM provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    /// LLM provider name (e.g. "openrouter", "openai").
    pub provider: String,
    /// Model identifier (e.g. "gpt-4o", "claude-3-opus").
    pub model: String,
    /// Sampling temperature (0.0–2.0).
    pub temperature: f64,
    /// Maximum tokens per response.
    pub max_tokens: u32,
    /// API key for the LLM provider.
    pub api_key: String,
}

impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            provider: "openrouter".into(),
            model: "gpt-4o".into(),
            temperature: 0.7,
            max_tokens: 4096,
            api_key: String::new(),
        }
    }
}

/// Pipeline feature toggles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineSettings {
    /// Enable automatic feature generation.
    pub feature_generation: bool,
    /// Enable scaling transformations.
    pub scaling: bool,
    /// Enable encoding transformations.
    pub encoding: bool,
    /// Enable polynomial feature expansion.
    pub polynomial_features: bool,
    /// Enable datetime feature extraction.
    pub datetime_features: bool,
}

impl Default for PipelineSettings {
    fn default() -> Self {
        Self {
            feature_generation: true,
            scaling: true,
            encoding: true,
            polynomial_features: false,
            datetime_features: false,
        }
    }
}

/// Evaluation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationSettings {
    /// Performance metric (RMSE, MAE, Accuracy, F1).
    pub metric: String,
    /// Number of cross-validation folds.
    pub cross_validation: u32,
}

impl Default for EvaluationSettings {
    fn default() -> Self {
        Self {
            metric: "RMSE".into(),
            cross_validation: 5,
        }
    }
}

/// Top-level user settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Enable verbose logging.
    pub verbose_logging: bool,
    /// Enable automatic saving of pipeline state.
    pub auto_save: bool,

    /// LLM provider configuration.
    pub llm: LlmSettings,
    /// Pipeline feature toggles.
    pub pipeline: PipelineSettings,
    /// Evaluation configuration.
    pub evaluation: EvaluationSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            verbose_logging: false,
            auto_save: true,
            llm: LlmSettings::default(),
            pipeline: PipelineSettings::default(),
            evaluation: EvaluationSettings::default(),
        }
    }
}
