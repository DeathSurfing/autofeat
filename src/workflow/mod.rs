//! Directed acyclic graph of preprocessing nodes.
//!
//! The workflow represents the user's feature engineering pipeline as a DAG
//! where every node is a transformation. Nodes can be added, removed, moved,
//! disabled, duplicated, and edited while validating ordering constraints.

pub mod graph;
pub mod node;
pub mod validation;
