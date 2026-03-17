use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::bear::Bear;
use crate::events::EventRecord;
use crate::llm::LlmProvider;
use crate::time::GameTime;

const SAVE_FILE: &str = "bear_save.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub bear: Bear,
    pub time: GameTime,
    pub event_log: Vec<EventRecord>,
    pub dialogue_log: Vec<DialogueEntry>,
    #[serde(default)]
    pub food_inventory: u32,
    #[serde(default)]
    pub llm_provider: Option<LlmProvider>,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub hibernation_ready: bool, // true once fat hits threshold during fall
    #[serde(default)]
    pub bear_missing: bool,      // true when hunger hit 0 — bear wandered off
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEntry {
    pub player: String,
    pub bear: String,
    pub year: u32,
    pub season: String,
}

impl SaveState {
    pub fn new(bear: Bear) -> Self {
        Self {
            bear,
            time: GameTime::new(),
            event_log: Vec::new(),
            dialogue_log: Vec::new(),
            food_inventory: 3,
            llm_provider: None,
            api_key: None,
            hibernation_ready: false,
            bear_missing: false,
        }
    }

    pub fn save_path() -> PathBuf {
        PathBuf::from(SAVE_FILE)
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(Self::save_path(), json)?;
        Ok(())
    }

    pub fn load() -> Result<Option<Self>> {
        let path = Self::save_path();
        if !path.exists() {
            return Ok(None);
        }
        let json = fs::read_to_string(path)?;
        let state = serde_json::from_str(&json)?;
        Ok(Some(state))
    }

}
