//! AI planning agent powered by rig + OpenRouter.
//!
//! The agent analyzes the current workflow, proposes improvements,
//! reviews the full pipeline, and explains individual transformations.
//! It never modifies the workflow without user approval.

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
        }
    }

    /// Add a user message and generate an assistant response.
    pub fn send_message(&mut self, text: String) {
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

        // TODO: call actual LLM via rig
        self.messages.push(Message {
            role: "Assistant",
            content: "I received your message. Full AI integration is coming soon! \
                     For now, you can set up your API key in Settings."
                .into(),
            dataset_summary: None,
        });
        self.scroll = usize::MAX;
    }
}
