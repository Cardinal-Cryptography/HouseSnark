use aleph_client::{keypair_from_string, SignedConnection};
use anyhow::Result;
use clap::Parser;
use house_snark::{MerkleRoot, Note, Nullifier, TokenAmount, TokenId, Trapdoor};
use inquire::Password;
use zeroize::Zeroize;
use ContractInteractionCommand::{Deposit, RegisterVK, Withdraw};
use StateReadCommand::{PrintState, ShowAssets};
use StateWriteCommand::{SetContractAddress, SetNode};

use crate::{
    app_state::AppState,
    config::{
        CliConfig,
        Command::{ContractInteraction, StateRead, StateWrite},
        ContractInteractionCommand, SetContractAddressCmd, SetNodeCmd, ShowAssetsCmd,
        StateReadCommand, StateWriteCommand,
    },
    contract::Blender,
    deposit::do_deposit,
    register::do_register,
    state_file::{get_app_state, save_app_state},
    withdraw::do_withdraw,
};

type DepositId = u16;

mod app_state;
mod config;
mod contract;
mod deposit;
mod register;
mod state_file;
mod withdraw;

fn perform_state_write_action(app_state: &mut AppState, command: StateWriteCommand) -> Result<()> {
    match command {
        SetNode(SetNodeCmd { node }) => {
            app_state.node_address = node;
        }
        SetContractAddress(SetContractAddressCmd { address }) => {
            app_state.contract_address = address;
        }
    };
    Ok(())
}

fn perform_state_read_action(app_state: &mut AppState, command: StateReadCommand) -> Result<()> {
    match command {
        ShowAssets(ShowAssetsCmd { token_id }) => {
            let assets = match token_id {
                None => app_state.get_all_assets(),
                Some(token_id) => app_state.get_single_asset(token_id),
            };
            println!("{:?}", assets)
        }
        PrintState => {
            println!("{}", serde_json::to_string_pretty(&app_state).unwrap())
        }
    };
    Ok(())
}

fn perform_contract_action(
    app_state: &mut AppState,
    command: ContractInteractionCommand,
) -> Result<()> {
    let signer = keypair_from_string(&app_state.caller_seed);
    let connection = SignedConnection::new(&app_state.node_address, signer);

    let metadata_file = command.get_metadata_file();
    let contract = Blender::new(&app_state.contract_address, &metadata_file)?;

    match command {
        Deposit(cmd) => do_deposit(contract, connection, cmd, app_state)?,
        Withdraw(cmd) => do_withdraw(contract, connection, cmd, app_state)?,
        RegisterVK(_) => do_register(contract, connection)?,
    };
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_config: CliConfig = CliConfig::parse();

    let seed = match cli_config.seed {
        Some(seed) => seed,
        _ => Password::new("Password (account seed):")
            .without_confirmation()
            .prompt()?,
    };

    let mut app_state = get_app_state(&cli_config.state_file, &seed)?;
    app_state.caller_seed = seed;

    match cli_config.command {
        StateWrite(cmd) => perform_state_write_action(&mut app_state, cmd)?,
        StateRead(cmd) => perform_state_read_action(&mut app_state, cmd)?,
        ContractInteraction(cmd) => perform_contract_action(&mut app_state, cmd)?,
    }

    save_app_state(&app_state, &cli_config.state_file, &app_state.caller_seed)?;

    app_state.caller_seed.zeroize();
    // `cli_config.seed` and `seed` are already moved

    Ok(())
}
