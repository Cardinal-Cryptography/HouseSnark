use aleph_client::SignedConnection;
use anyhow::Result;

use crate::{app_state::AppState, config::DepositCmd, contract::Blender};

pub(super) fn do_deposit(
    contract: Blender,
    connection: SignedConnection,
    cmd: DepositCmd,
    app_state: &mut AppState,
) -> Result<()> {
    let dummy_proof = vec![1, 2, 3];
    let dummy_note = Default::default();

    let leaf_idx = contract.deposit(
        &connection,
        cmd.token_id,
        cmd.amount,
        dummy_note,
        &dummy_proof,
    )?;

    app_state.add_deposit(cmd.token_id, cmd.amount, leaf_idx);

    Ok(())
}
