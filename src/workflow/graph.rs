//! DAG representation of the preprocessing workflow.

use crate::workflow::node::Node;

/// A directed acyclic graph of preprocessing nodes.
pub struct WorkflowGraph {
    /// Ordered list of nodes in the workflow.
    pub nodes: Vec<Node>,
}
