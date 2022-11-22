use std::u32;

use anyhow::Result;

use super::types::{FrontendAccount, FrontendMerklePathSingle, FrontendMerkleRoot, FrontendNote};
use crate::relations::blender::note::note_from_bytes;

pub fn parse_frontend_note(frontend_note: &str) -> Result<FrontendNote> {
    Ok(note_from_bytes(frontend_note.as_bytes()))
}

// TODO: Change note_from_bytes.
pub fn parse_frontend_merkle_root(frontend_merkle_root: &str) -> Result<FrontendMerkleRoot> {
    Ok(note_from_bytes(frontend_merkle_root.as_bytes()))
}

pub fn parse_frontend_account(frontend_account: &str) -> Result<FrontendAccount> {
    Ok(account_from_bytes(frontend_account.as_bytes()))
}

fn account_from_bytes(bytes: &[u8]) -> FrontendAccount {
    [
        u32::from_le_bytes(bytes[0..8].try_into().unwrap()),
        u32::from_le_bytes(bytes[8..16].try_into().unwrap()),
        u32::from_le_bytes(bytes[16..24].try_into().unwrap()),
        u32::from_le_bytes(bytes[24..32].try_into().unwrap()),
    ]
}

// TODO: Change note_from_bytes.
pub fn parse_frontend_merkle_path_single(
    frontend_merkle_path_single: &str,
) -> Result<FrontendMerklePathSingle> {
    Ok(note_from_bytes(frontend_merkle_path_single.as_bytes()))
}
