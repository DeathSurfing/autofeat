//! `autofeat` — Interactive AI-powered feature engineering CLI.
//!
//! A terminal-native tool where the AI proposes preprocessing workflows,
//! the user reviews and edits every step, and transformations execute
//! through Rust tools powered by `featrs`.
//!
//! # Architecture
//!
//! | Module | Description |
//! |---|---|
//! | [`cli`] | CLI argument parsing (clap) |
//! | [`app`] | Event loop, screen routing, shared state |
//! | [`tui`] | Ratatui terminal UI — Agent, Dataset, Workflow, Tools, Settings, Help screens |
//! | [`workflow`] | Directed acyclic graph of preprocessing nodes |
//! | [`agent`] | AI planning agent (rig + OpenRouter), prompt templates, pipeline review, explainability |
//! | [`tools`] | Tool registry and transformer executors wrapping featrs |
//! | [`pipeline`] | Pipeline evaluation and execution |
//! | [`dataset`] | Dataset loading, schema discovery, statistics |
//! | [`history`] | Revision stack for undo/redo |
//! | [`config`] | Serializable user settings and theme definitions |

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod agent;
pub mod app;
pub mod cli;
pub mod config;
pub mod dataset;
pub mod history;
pub mod pipeline;
pub mod tools;
pub mod tui;
pub mod workflow;

/// Convenient glob import of common types.
pub mod prelude {
    pub use crate::app::App;
    pub use crate::cli::Cli;
    pub use crate::config::settings::Settings;
    pub use crate::workflow::graph::WorkflowGraph;
    pub use crate::workflow::node::Node;
    pub use crate::workflow::node::NodeKind;
}
