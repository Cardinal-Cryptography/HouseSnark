use aleph_client::SignedConnection;
use anyhow::Result;
use house_snark::{DepositRelation, NonUniversalProvingSystem, RawKeys};

use crate::contract::{Blender, Relation};

pub(super) fn do_register(contract: Blender, connection: SignedConnection) -> Result<()> {
    let circuit = DepositRelation::default();
    let system = NonUniversalProvingSystem::Groth16;
    let RawKeys { vk, .. } = system.generate_keys(circuit);
    contract.register_vk(&connection, Relation::Deposit, vk)?;

    // TODO : register withdrawal vk

    Ok(())
}
