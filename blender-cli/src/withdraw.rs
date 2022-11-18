use aleph_client::{account_from_keypair, keypair_from_string, SignedConnection};
use anyhow::{anyhow, Result};
use inquire::{CustomType, Select};

use crate::{
    app_state::{AppState, Deposit},
    config::WithdrawCmd,
    contract::Blender,
    DepositId, TokenAmount,
};

pub(super) fn do_withdraw(
    contract: Blender,
    mut connection: SignedConnection,
    cmd: WithdrawCmd,
    app_state: &mut AppState,
) -> Result<()> {
    let (deposit, amount) = get_deposit_and_amount(&cmd, app_state)?;

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

    Ok(())
}

fn get_deposit_and_amount(
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
