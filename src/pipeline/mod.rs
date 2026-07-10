//! Pipeline evaluation and execution.
//!
//! Executes the workflow DAG via featrs transformers and evaluates
//! performance metrics (RMSE, MAE, Accuracy, F1, training time).

pub mod evaluate;
pub mod execute;

pub use execute::ExecutionResult;
