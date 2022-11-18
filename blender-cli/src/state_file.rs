use std::{
    fs,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};

use crate::app_state::AppState;

/// Try to get `AppState` from `path`. If `path` describes non-existing file, the default `AppState`
/// will be created, saved to `path` and returned.
pub fn get_app_state(path: &PathBuf) -> Result<AppState> {
    match path.exists() {
        true => {
            println!("File with state was found. Reading the state from {path:?}.");
            read_from(path)
        }
        false => {
            println!("File with state not found. Creating the default state in {path:?}.");
            create_and_save_default_state(path)
        }
    }
}

/// Save `app_state` to `path`.
pub fn save_app_state(app_state: &AppState, path: &PathBuf) -> Result<()> {
    fs::write(
        path,
        serde_json::to_string_pretty(app_state).expect("Serialization should succeed"),
    )
    .map_err(|e| anyhow!("Failed to save application state: {e:?}"))
}

/// Read `AppState` from `path`.
fn read_from(path: &Path) -> Result<AppState> {
    let file_content =
        fs::read_to_string(path).map_err(|e| anyhow!("Failed to read file content: {e:?}"))?;
    serde_json::from_str::<AppState>(&file_content)
        .map_err(|e| anyhow!("Failed to deserialize application state: {e:?}"))
}

/// Create the default `AppState`, save it to `path` and return it.
fn create_and_save_default_state(path: &PathBuf) -> Result<AppState> {
    File::create(path).map_err(|e| anyhow!("Failed to create {path:?}: {e:?}"))?;

    let state = AppState::default();
    save_app_state(&state, path).map_err(|e| anyhow!("Failed to save state to {path:?}: {e:?}"))?;

    Ok(state)
}
