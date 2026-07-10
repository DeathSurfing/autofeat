//! Ratatui terminal UI — screen rendering and shared widgets.
//!
//! | Screen | Key | Description |
//! |---|---|---|
//! | [`Agent`](screens::agent) | `A` | Live reasoning, tool execution, conversation |
//! | [`Dataset`](screens::dataset) | `D` | Schema, statistics, distributions, null counts |
//! | [`Workflow`](screens::workflow) | `W` | Interactive DAG editor |
//! | [`Tools`](screens::tools) | `T` | Execution history, runtime, outputs |
//! | [`Settings`](screens::settings) | `S` | General / LLM / Pipeline / Eval configuration |
//! | [`Help`](screens::help) | `H` | Keyboard shortcuts reference |

pub mod screens;
pub mod widgets;
