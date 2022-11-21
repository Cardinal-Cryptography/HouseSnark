use aleph_client::{account_from_keypair, keypair_from_string, SignedConnection};
use anyhow::{anyhow, Result};
use inquire::{CustomType, Select};

use crate::{
    app_state::{AppState, Deposit},
    config::WithdrawCmd,
    contract::Blender,
    TokenAmount,
};

pub(super) fn do_withdraw(
    contract: Blender,
    mut connection: SignedConnection,
    cmd: WithdrawCmd,
    app_state: &mut AppState,
) -> Result<()> {
    let (deposit, withdraw_amount) = get_deposit_and_withdraw_amount(&cmd, app_state)?;

    let WithdrawCmd {
        recipient,
        caller_seed,
        fee,
        ..
    } = cmd;

    if let Some(seed) = caller_seed {
        connection = SignedConnection::new(&app_state.node_address, keypair_from_string(&seed));
    }
    let recipient = match recipient {
        None => account_from_keypair(&keypair_from_string(&app_state.caller_seed)),
        Some(recipient) => recipient,
    };

    let merkle_root = contract.get_merkle_root(&connection);

    // TODO:
    // - create real proof
    // - create new note

    let dummy_proof = vec![1, 2, 3];
    let dummy_note = Default::default();

    let leaf_idx = contract.withdraw(
        &connection,
        deposit.token_id,
        withdraw_amount,
        recipient,
        fee,
        merkle_root,
        deposit.nullifier,
        dummy_note,
        &dummy_proof,
    )?;

    app_state.delete_deposit_by_id(deposit.deposit_id);
    // save new deposit to the state
    let tokens_left = deposit.token_amount - withdraw_amount;
    if tokens_left > 0 {
        app_state.add_deposit(deposit.token_id, tokens_left, leaf_idx);
    }

    Ok(())
}

fn get_deposit_and_withdraw_amount(
    cmd: &WithdrawCmd,
    app_state: &AppState,
) -> Result<(Deposit, TokenAmount)> {
    if !cmd.interactive {
        if let Some(deposit) = app_state.get_deposit_by_id(cmd.deposit_id.unwrap()) {
            return Ok((deposit, cmd.amount.unwrap()));
        }
        return Err(anyhow!("Incorrect deposit id"));
    }

    let deposit = Select::new("Select one of your deposits:", app_state.deposits())
        .with_page_size(5)
        .prompt()?;

    let amount = CustomType::<TokenAmount>::new("Specify how many tokens should be withdrawn:")
        .with_default(deposit.token_amount)
        .with_parser(&|a| match str::parse::<TokenAmount>(a) {
            Ok(amount) if amount <= deposit.token_amount => Ok(amount),
            _ => Err(()),
        })
        .with_error_message(
            "You should provide a valid amount, no more than the whole deposit value",
        )
        .prompt()?;

    Ok((deposit, amount))
}
