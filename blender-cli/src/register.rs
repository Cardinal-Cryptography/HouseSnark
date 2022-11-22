use aleph_client::SignedConnection;
use anyhow::Result;

use crate::contract::{Blender, Relation};

#[allow(dead_code)]
pub fn do_register(
    contract: Blender,
    connection: SignedConnection,
    relation: Relation,
    vk: Vec<u8>,
) -> Result<()> {
    contract.register_vk(&connection, relation, vk)?;
    Ok(())
}
