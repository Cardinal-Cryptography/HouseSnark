use std::{fs, path::Path};

use aleph_client::AccountId;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct AppState {
    pub caller_seed: String,
    pub node_address: String,
    pub contract_address: AccountId,
}

const DEFAULT_SEED: &str = "//Alice";
const DEFAULT_NODE_ADDRESS: &str = "ws://127.0.0.1:9944";

impl Default for AppState {
    fn default() -> Self {
        Self {
            caller_seed: DEFAULT_SEED.to_string(),
            node_address: DEFAULT_NODE_ADDRESS.to_string(),
            contract_address: AccountId::new([0u8; 32]),
        }
    }
}

pub fn read_from(path: &Path) -> Result<AppState> {
    let file_content = fs::read_to_string(path)?;
    serde_json::from_str::<AppState>(&file_content)
        .map_err(|e| anyhow!("Failed to deserialize application state: {:?}", e))
}

pub fn write_to(state: &AppState, path: &Path) -> Result<()> {
    fs::write(path, serde_json::to_string_pretty(state)?)
        .map_err(|e| anyhow!("Failed to save application state: {:?}", e))
}
