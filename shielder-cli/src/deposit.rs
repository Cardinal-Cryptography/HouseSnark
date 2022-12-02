use std::fs;

use aleph_client::SignedConnection;
use anyhow::Result;
use house_snark::{
    compute_note, DepositRelation, NonUniversalProvingSystem, RawKeys, SomeProvingSystem,
};
use rand::Rng;

use crate::{app_state::AppState, config::DepositCmd, contract::Shielder, Nullifier, Trapdoor};

pub(super) fn do_deposit(
    contract: Shielder,
    connection: SignedConnection,
    cmd: DepositCmd,
    app_state: &mut AppState,
) -> Result<()> {
    let DepositCmd {
        token_id,
        amount: token_amount,
        proving_key_file,
        ..
    } = cmd;

    let mut rng = rand::thread_rng();

    let trapdoor: Trapdoor = rng.gen::<u64>();
    let nullifier: Nullifier = rng.gen::<u64>();
    let note = compute_note(token_id, token_amount, trapdoor, nullifier);

    let circuit = DepositRelation::new(note, token_id, token_amount, trapdoor, nullifier);

    let pk = match fs::read(proving_key_file) {
        Ok(bytes) => bytes,
        Err(_e) => {
            let system = NonUniversalProvingSystem::Groth16;
            let RawKeys { pk, vk } = system.generate_keys(circuit.clone());

            fs::write("deposit.pk.bytes", pk.clone()).unwrap();
            // NOTE: not needed here but for registering in the snarcos pallet
            fs::write("deposit.vk.bytes", vk).unwrap();

            pk
        }
    };

    let system = SomeProvingSystem::NonUniversal(NonUniversalProvingSystem::Groth16);
    let proof = system.prove(circuit, pk);
    let leaf_idx = contract.deposit(&connection, cmd.token_id, cmd.amount, note, &proof)?;

    app_state.add_deposit(cmd.token_id, cmd.amount, trapdoor, nullifier, leaf_idx);

    Ok(())
}
