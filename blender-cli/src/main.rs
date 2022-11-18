use aleph_client::{keypair_from_string, SignedConnection};
use anyhow::Result;
use clap::Parser;
use inquire::Password;
use zeroize::Zeroize;

use crate::{
    app_state::AppState,
    config::{
        CliConfig, Command, Command::*, DepositCmd, SetContractAddressCmd, SetNodeCmd, SetSeedCmd,
        ShowAssetsCmd,
    },
    contract::Blender,
    state_file::{get_app_state, save_app_state},
};

type TokenId = u16;
type TokenAmount = u64;
type Note = [u8; 32];

mod app_state;
mod config;
mod contract;
mod state_file;

fn perform_state_update_action(
    mut app_state: AppState,
    command: Command,
) -> Result<Option<AppState>> {
    match command {
        SetSeed(SetSeedCmd { seed }) => {
            app_state.caller_seed = seed;
        }
        SetNode(SetNodeCmd { node }) => {
            app_state.node_address = node;
        }
        SetContractAddress(SetContractAddressCmd { address }) => {
            app_state.contract_address = address;
        }
        _ => unreachable!(),
    };
    Ok(Some(app_state))
}

fn perform_state_read_action(app_state: AppState, command: Command) -> Result<Option<AppState>> {
    match command {
        ShowAssets(ShowAssetsCmd { token_id }) => {
            let assets = match token_id {
                None => app_state.get_all_assets(),
                Some(token_id) => app_state.get_single_asset(token_id),
            };
            println!("{:?}", assets)
        }
        PrintState => println!("{}", serde_json::to_string_pretty(&app_state).unwrap()),
        _ => {}
    };
    Ok(None)
}

fn perform_contract_action(mut app_state: AppState, command: Command) -> Result<Option<AppState>> {
    let signer = keypair_from_string(&app_state.caller_seed);
    let connection = SignedConnection::new(&app_state.node_address, signer);

    let metadata_file = command.get_metadata_file().unwrap();
    let contract = Blender::new(&app_state.contract_address, &metadata_file)?;

    match command {
        Deposit(DepositCmd {
            token_id, amount, ..
        }) => {
            let dummy_proof = vec![1, 2, 3];
            let dummy_note = Default::default();

            let leaf_idx =
                contract.deposit(&connection, token_id, amount, dummy_note, &dummy_proof)?;

            app_state.deposits.push(app_state::Deposit {
                token_id,
                token_amount: amount,
                leaf_idx,
            });
        }
        _ => unreachable!(),
    };
    Ok(Some(app_state))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_config: CliConfig = CliConfig::parse();

    let mut password = match cli_config.password {
        Some(p) => p,
        _ => Password::new("Password:").without_confirmation().prompt()?,
    };

    let app_state = get_app_state(&cli_config.state_file, &password)?;

    let updated_state = if cli_config.command.is_state_update_action() {
        perform_state_update_action(app_state, cli_config.command)?
    } else if cli_config.command.is_state_read_action() {
        perform_state_read_action(app_state, cli_config.command)?
    } else if cli_config.command.is_contract_action() {
        perform_contract_action(app_state, cli_config.command)?
    } else {
        unreachable!()
    };

    if let Some(state) = updated_state {
        save_app_state(&state, &cli_config.state_file, &password)?
    }

    password.zeroize();

    Ok(())
}
