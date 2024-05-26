use chrono::DateTime;
use eyre::Context;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub last_updated_at: Option<String>,
}

impl State {
    pub fn try_create() -> Result<Self> {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("state.json");
        if !path.exists() {
            log::debug!("state file not exists at {}", path.display());
            return Ok(Self {
                last_updated_at: None,
            });
        }
        let content = std::fs::read_to_string(&path).context("Read")?;
        let state: Self = serde_json::from_str(&content)?;
        Ok(state)
    }
    pub fn save(&mut self) {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("state.json");
        log::debug!("save state {}", path.display());
        std::fs::write(path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i128,
    pub kind: String,
    pub title: String,
    pub url: Option<String>,
    pub repo_url: Option<String>,
    pub repo_name: Option<String>,
    pub updated_at: Option<String>,
}

impl Notification {
    pub fn is_new(&self, state: &mut State) -> bool {
        // If new notification different from previous it's new
        if let (Ok(state_updated_at), Ok(updated_at)) = (
            DateTime::parse_from_rfc3339(&state.last_updated_at.clone().unwrap_or_default()),
            DateTime::parse_from_rfc3339(&self.updated_at.clone().unwrap_or_default()),
        ) {
            // Compare the timestamps
            return updated_at > state_updated_at;
        }
        true
    }
}
