use aleph_client::SignedConnection;
use anyhow::Result;
use house_snark::{compute_note, DepositRelation, NonUniversalProvingSystem, SomeProvingSystem};

use crate::{app_state::AppState, config::DepositCmd, contract::Blender, Nullifier, Trapdoor};

pub(super) fn do_deposit(
    contract: Blender,
    connection: SignedConnection,
    cmd: DepositCmd,
    app_state: &mut AppState,
) -> Result<()> {
    let DepositCmd {
        token_id,
        amount: token_amount,
        ..
    } = cmd;

    // TODO
    // - read from CLI args
    let dummy_trapdoor = Trapdoor::default();
    let dummy_nullifier = Nullifier::default();
    let dummy_pk = vec![0u8, 0, 0, 0];

    let note = compute_note(token_id, token_amount, dummy_trapdoor, dummy_nullifier);
    let circuit = DepositRelation::new(
        note,
        token_id,
        token_amount,
        dummy_trapdoor,
        dummy_nullifier,
    );

    let system: SomeProvingSystem =
        house_snark::SomeProvingSystem::NonUniversal(NonUniversalProvingSystem::Groth16);
    let proof = system.prove(circuit, dummy_pk);
    let leaf_idx = contract.deposit(&connection, cmd.token_id, cmd.amount, note, &proof)?;
    app_state.add_deposit(cmd.token_id, cmd.amount, leaf_idx);

    Ok(())
}
