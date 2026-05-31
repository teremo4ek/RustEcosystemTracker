use std::path::Path;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct FeedState {
    pub last_updated: Option<String>,
}

impl FeedState {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(path)?;
        let state: FeedState = serde_json::from_str(&data).unwrap_or_default();
        Ok(state)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn has_changed(&self, new_updated: &DateTime<FixedOffset>) -> bool {
        match &self.last_updated {
            None => true,
            Some(stored_str) => match DateTime::parse_from_rfc3339(stored_str) {
                Ok(stored_dt) => &stored_dt < new_updated,
                Err(_) => true,
            },
        }
    }
}
