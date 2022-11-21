use aleph_client::SignedConnection;
use anyhow::Result;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
use house_snark::{
    compute_note, DepositRelation, NonUniversalProvingSystem, RawKeys, SomeProvingSystem,
};

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

    let note = compute_note(token_id, token_amount, dummy_trapdoor, dummy_nullifier);
    let circuit = DepositRelation::new(
        note,
        token_id,
        token_amount,
        dummy_trapdoor,
        dummy_nullifier,
    );

    let cs = ConstraintSystem::new_ref();
    circuit.generate_constraints(cs.clone())?;

    let system = NonUniversalProvingSystem::Groth16;
    let RawKeys { pk, vk } = system.generate_keys(circuit);

    println!("pk {:?}", pk);
    println!("vk {:?}", vk);

    // let system: SomeProvingSystem =
    //     SomeProvingSystem::NonUniversal(NonUniversalProvingSystem::Groth16);

    // // system.

    // let keys = NonUniversalProvingSystem::generate_keys(&system.0, circuit);

    // let proof = system.prove(circuit, dummy_pk);
    // let leaf_idx = contract.deposit(&connection, cmd.token_id, cmd.amount, note, &proof)?;
    // app_state.add_deposit(cmd.token_id, cmd.amount, leaf_idx);

    // let is_satisfied = cs.is_satisfied().unwrap();
    // if !is_satisfied {
    //     println!("{:?}", cs.which_is_unsatisfied());
    // }

    // assert!(is_satisfied);

    Ok(())
}
