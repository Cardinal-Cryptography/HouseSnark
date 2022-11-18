use aleph_client::SignedConnection;
use anyhow::Result;
use inquire::{CustomType, Select};

use crate::{app_state::AppState, config::WithdrawCmd, contract::Blender, DepositId, TokenAmount};

pub(super) fn do_withdraw(
    contract: Blender,
    connection: SignedConnection,
    cmd: WithdrawCmd,
    app_state: &mut AppState,
) -> Result<()> {
    let (deposit_id, amount) = get_deposit_and_amount(&cmd, app_state)?;

    Ok(())
}

fn get_deposit_and_amount(
    cmd: &WithdrawCmd,
    app_state: &AppState,
) -> Result<(DepositId, TokenAmount)> {
    if !cmd.interactive {
        return Ok((cmd.deposit_id.unwrap(), cmd.amount.unwrap()));
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

    Ok((deposit.deposit_id, amount))
}
