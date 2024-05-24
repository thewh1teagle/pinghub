use std::env;
use eyre::Context;
use serde::{Serialize,Deserialize};
use eyre::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub last_id: Option<i128>
}

impl State {

    pub fn try_create() -> Result<Self> {
        let path = env::current_exe().unwrap().parent().unwrap().join("state.json");
        if !path.exists() {
            log::debug!("state file not exists at {}", path.display());
            return Ok(Self { last_id: None });
        }
        let content = std::fs::read_to_string(&path).context("Read")?;
        let state: Self = serde_json::from_str(&content)?;
        Ok(state)
    }
    pub fn save(&mut self) {
        let path = env::current_exe().unwrap().parent().unwrap().join("state.json");
        log::debug!("save state {}", path.display());
        std::fs::write(path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i128,
    pub title: String,
    pub url: String,
    pub repo_name: String
}

impl Notification {
    pub fn is_new(&self, state: &mut State) -> bool {
        if let Some(last_id) = state.last_id {
            let is_new = self.id > last_id;
            if !is_new {
                return false;
            }
        }
        // Update state
        state.last_id = Some(self.id);
        state.save();
        true
    }
}