//! DAG representation of the preprocessing workflow.

use crate::workflow::node::{Node, NodeKind};

/// A pipeline of preprocessing nodes (sequential order).
pub struct WorkflowGraph {
    /// Ordered list of nodes in the workflow.
    pub nodes: Vec<Node>,
    /// Next ID to assign.
    next_id: usize,
}

impl Default for WorkflowGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowGraph {
    /// Create an empty workflow.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a new node of the given kind at the end of the pipeline.
    pub fn add_node(&mut self, kind: NodeKind) {
        let node = Node::new(kind, self.next_id);
        self.next_id += 1;
        self.nodes.push(node);
    }

    /// Remove the node at `index`.
    pub fn remove_node(&mut self, index: usize) {
        if index < self.nodes.len() {
            self.nodes.remove(index);
        }
    }

    /// Move the node at `index` up one position.
    pub fn move_up(&mut self, index: usize) {
        if index > 0 && index < self.nodes.len() {
            self.nodes.swap(index, index - 1);
        }
    }

    /// Move the node at `index` down one position.
    pub fn move_down(&mut self, index: usize) {
        if index + 1 < self.nodes.len() {
            self.nodes.swap(index, index + 1);
        }
    }

    /// Toggle the enabled state of the node at `index`.
    pub fn toggle(&mut self, index: usize) {
        if let Some(node) = self.nodes.get_mut(index) {
            node.enabled = !node.enabled;
        }
    }

    /// Duplicate the node at `index`, inserting the copy after it.
    pub fn duplicate(&mut self, index: usize) {
        if let Some(orig) = self.nodes.get(index) {
            let mut copy = orig.clone();
            copy.id = self.next_id;
            self.next_id += 1;
            self.nodes.insert(index + 1, copy);
        }
    }

    /// Number of nodes in the pipeline.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether the pipeline is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
