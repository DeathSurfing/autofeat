//! AI planning agent powered by rig + OpenRouter.
//!
//! The agent analyzes the current workflow, proposes improvements,
//! reviews the full pipeline, and explains individual transformations.
//! It never modifies the workflow without user approval.

use crate::dataset::Dataset;
use crate::workflow::graph::WorkflowGraph;
use crate::workflow::node::NodeKind;

pub mod explain;
pub mod planner;
pub mod prompts;
pub mod review;

/// A single message in the agent conversation.
pub struct Message {
    /// Role label ("You", "Assistant", "System").
    pub role: &'static str,
    /// Message body text.
    pub content: String,
    /// Optional dataset summary attached to this message.
    pub dataset_summary: Option<String>,
}

/// State for the Agent conversation screen.
pub struct AgentState {
    /// Conversation history.
    pub messages: Vec<Message>,
    /// Current input buffer.
    pub input: String,
    /// Whether the input popover is open.
    pub inputting: bool,
    /// Scroll offset for the conversation view.
    pub scroll: usize,
    /// Whether the AI is currently generating a response.
    pub waiting: bool,
}

impl Default for AgentState {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentState {
    /// Create a new agent state with a welcome message.
    pub fn new() -> Self {
        Self {
            messages: vec![Message {
                role: "Assistant",
                content: "Hello! I'm your feature engineering assistant. \
                         I can help you build preprocessing pipelines, \
                         suggest transformations, and review your workflow. \
                         What would you like to do?"
                    .into(),
                dataset_summary: None,
            }],
            input: String::new(),
            inputting: false,
            scroll: 0,
            waiting: false,
        }
    }

    /// Add a user message and generate an assistant response via the LLM.
    /// Optionally modifies the workflow based on the AI's suggestions.
    pub async fn send_message(
        &mut self,
        text: String,
        api_key: &str,
        model: &str,
        dataset: Option<&Dataset>,
        workflow: &mut WorkflowGraph,
    ) {
        if text.is_empty() {
            return;
        }
        self.messages.push(Message {
            role: "You",
            content: text,
            dataset_summary: None,
        });
        self.input.clear();
        self.inputting = false;
        self.waiting = true;

        // Build the system prompt
        let system = prompts::system_prompt(dataset);

        // Call the LLM
        let reply = match planner::call_llm(api_key, model, &system, &self.messages).await {
            Ok(content) => content,
            Err(e) => {
                self.messages.push(Message {
                    role: "Assistant",
                    content: e,
                    dataset_summary: None,
                });
                self.waiting = false;
                self.scroll = usize::MAX;
                return;
            }
        };

        // Parse actions from the reply and apply them to the workflow
        let actions = parse_actions(&reply);
        for action in &actions {
            match *action {
                Action::Add(kind) => workflow.add_node(kind),
                Action::Remove(kind) => {
                    // Remove last occurrence of the given kind
                    if let Some(pos) = workflow.nodes.iter().rposition(|n| n.kind == kind) {
                        workflow.remove_node(pos);
                    }
                }
            }
        }

        let message = if actions.is_empty() {
            reply
        } else {
            let action_count = actions.len();
            format!(
                "{}\n\n---\n*I've added {} {} to the pipeline. Press W to view your workflow.*",
                reply,
                action_count,
                if action_count == 1 { "step" } else { "steps" },
            )
        };

        self.messages.push(Message {
            role: "Assistant",
            content: message,
            dataset_summary: None,
        });
        self.waiting = false;
        self.scroll = usize::MAX;
    }
}

pub(crate) enum Action {
    Add(NodeKind),
    Remove(NodeKind),
}

pub(crate) fn parse_actions(reply: &str) -> Vec<Action> {
    let mut actions = Vec::new();
    for line in reply.lines() {
        let line = line.trim();
        if let Some(kind_name) = line.strip_prefix("ADD ")
            && let Some(kind) = parse_node_kind(kind_name)
        {
            actions.push(Action::Add(kind));
        } else if let Some(kind_name) = line.strip_prefix("REMOVE ")
            && let Some(kind) = parse_node_kind(kind_name)
        {
            actions.push(Action::Remove(kind));
        }
    }
    actions
}

pub(crate) fn parse_node_kind(s: &str) -> Option<NodeKind> {
    match s.trim() {
        "MedianImputer" => Some(NodeKind::MedianImputer),
        "MeanImputer" => Some(NodeKind::MeanImputer),
        "RobustScaler" => Some(NodeKind::RobustScaler),
        "StandardScaler" => Some(NodeKind::StandardScaler),
        "OneHotEncoder" => Some(NodeKind::OneHotEncoder),
        "FrequencyEncoder" => Some(NodeKind::FrequencyEncoder),
        "PolynomialFeatures" => Some(NodeKind::PolynomialFeatures),
        _ => None,
    }
}
