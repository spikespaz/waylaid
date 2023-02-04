use std::time::Duration;

use serde::Deserialize;

use crate::duration::serde_with::DurationString;

#[derive(Debug, Clone, Deserialize)]
pub struct WaylaidConfig {
    pub timeouts: Vec<TimeoutCommand>,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct TimeoutCommand {
    #[serde_as(as = "DurationString")]
    pub timeout: Duration,
    pub command: String,
    pub resume_command: Option<String>,
}
