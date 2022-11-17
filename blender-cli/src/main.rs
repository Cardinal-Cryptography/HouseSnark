use std::{fs::File, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;

use crate::{
    app_state::AppState,
    config::{CliConfig, Command, DepositCmd, SetContractAddressCmd, SetNodeCmd, SetSeedCmd},
    contract_actions::deposit,
};

type TokenId = u16;
type TokenAmount = u64;

mod app_state;
mod config;
mod contract_actions;

fn create_and_save_default_state(path: &PathBuf) -> Result<AppState> {
    File::create(path).map_err(|e| anyhow!("Failed to create {path:?}: {e:?}"))?;

    let state = AppState::default();
    app_state::write_to(&state, path)
        .map_err(|e| anyhow!("Failed to save state to {path:?}: {e:?}"))?;

    Ok(state)
}

fn get_app_state(path: &PathBuf) -> Result<AppState> {
    match path.exists() {
        true => {
            println!("File was found. Reading the state from {path:?}.");
            app_state::read_from(path)
        }
        false => {
            println!("File not found. Creating the default state in {path:?}.");
            create_and_save_default_state(path)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_config: CliConfig = CliConfig::parse();

    let mut app_state = get_app_state(&cli_config.state_file)?;

    match cli_config.command {
        Command::SetSeed(SetSeedCmd { seed }) => {
            app_state.caller_seed = seed;
            app_state::write_to(&app_state, &cli_config.state_file)?;
        }
        Command::SetNode(SetNodeCmd { node }) => {
            app_state.node_address = node;
            app_state::write_to(&app_state, &cli_config.state_file)?;
        }
        Command::SetContractAddress(SetContractAddressCmd { address }) => {
            app_state.contract_address = address;
            app_state::write_to(&app_state, &cli_config.state_file)?;
        }

        Command::Deposit(DepositCmd { token_id, amount }) => deposit(&app_state, token_id, amount),
    }

    Ok(())
}
