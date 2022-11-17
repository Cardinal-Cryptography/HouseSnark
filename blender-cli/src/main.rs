use std::{fs::File, path::PathBuf};

use aleph_client::{keypair_from_string, SignedConnection};
use anyhow::{anyhow, Result};
use clap::Parser;

use crate::{
    app_state::AppState,
    config::{CliConfig, Command, DepositCmd, SetContractAddressCmd, SetNodeCmd, SetSeedCmd},
    contract::Blender,
};

type TokenId = u16;
type TokenAmount = u64;
type Note = [u8; 32];

mod app_state;
mod config;
mod contract;

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

        Command::Deposit(DepositCmd {
            token_id,
            amount,
            metadata_file,
        }) => {
            let signer = keypair_from_string(&app_state.caller_seed);
            let connection = SignedConnection::new(&app_state.node_address, signer);

            let contract = Blender::new(&app_state.contract_address, &metadata_file)?;
            contract.deposit(
                &connection,
                token_id,
                amount,
                Default::default(),
                &vec![1, 2, 3],
            )?;
        }
    }

    Ok(())
}
