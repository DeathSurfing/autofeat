//! Tool registry and transformer executors wrapping featrs.
//!
//! Each tool maps a [`NodeKind`](crate::workflow::node::NodeKind) to a featrs
//! transformer. The registry provides lookup, and individual modules
//! encapsulate the fit/transform logic.

pub mod encode;
pub mod features;
pub mod impute;
pub mod registry;
pub mod scale;
