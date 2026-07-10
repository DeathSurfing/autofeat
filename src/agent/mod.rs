//! AI planning agent powered by rig + OpenRouter.
//!
//! The agent analyzes the current workflow, proposes improvements,
//! reviews the full pipeline, and explains individual transformations.
//! It never modifies the workflow without user approval.

pub mod explain;
pub mod planner;
pub mod prompts;
pub mod review;
