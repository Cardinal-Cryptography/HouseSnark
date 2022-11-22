use aleph_client::{account_from_keypair, keypair_from_string, SignedConnection};
use anyhow::{anyhow, Result};
use house_snark::{compute_note, WithdrawRelation};
use inquire::{CustomType, Select};
use rand::Rng;

use crate::{
    app_state::{AppState, Deposit},
    config::WithdrawCmd,
    contract::Blender,
    Nullifier, TokenAmount, Trapdoor,
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

    let Deposit {
        token_id,
        token_amount: whole_token_amount,
        trapdoor: old_trapdoor,
        nullifier: old_nullifier,
        leaf_idx,
        ..
    } = deposit;

    let old_note = compute_note(token_id, whole_token_amount, old_trapdoor, old_nullifier);

    if let Some(seed) = caller_seed {
        connection = SignedConnection::new(&app_state.node_address, keypair_from_string(&seed));
    }
    let recipient = match recipient {
        None => account_from_keypair(&keypair_from_string(&app_state.caller_seed)),
        Some(recipient) => recipient,
    };

    let merkle_root = contract.get_merkle_root(&connection);

    let mut rng = rand::thread_rng();
    let new_trapdoor: Trapdoor = rng.gen::<u64>();
    let new_nullifier: Nullifier = rng.gen::<u64>();
    let new_token_amount = whole_token_amount - withdraw_amount;
    let new_note = compute_note(token_id, new_token_amount, new_trapdoor, new_nullifier);

    let circuit = WithdrawRelation::new(
        old_nullifier,
        merkle_root,
        new_note,
        token_id,
        withdraw_amount,
        old_trapdoor,
        new_trapdoor,
        new_nullifier,
        merkle_path,
        leaf_idx.into(),
        old_note,
        whole_token_amount,
        new_token_amount,
        fee.unwrap_or_default(),
        recipient,
    );

    let leaf_idx = contract.withdraw(
        &connection,
        token_id,
        withdraw_amount,
        recipient,
        fee,
        merkle_root,
        old_nullifier,
        new_note,
        &dummy_proof,
    )?;

    app_state.delete_deposit_by_id(deposit.deposit_id);

    // save new deposit to the state
    if new_token_amount > 0 {
        app_state.add_deposit(
            token_id,
            new_token_amount,
            new_trapdoor,
            new_nullifier,
            leaf_idx,
        );
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
