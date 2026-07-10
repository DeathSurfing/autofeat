//! Ratatui style definitions and theme configuration.

use crate::config::settings::ThemeVariant;

/// Theme definition containing Ratatui styles for the entire UI.
pub struct Theme {
    /// Selected theme variant.
    pub variant: ThemeVariant,
}
