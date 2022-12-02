use anyhow::Result;

use super::types::{FrontendAccount, FrontendMerklePathNode, FrontendMerkleRoot, FrontendNote};
use crate::relations::shielder::note::note_from_bytes;

pub fn parse_frontend_note(frontend_note: &str) -> Result<FrontendNote> {
    Ok(note_from_bytes(frontend_note.as_bytes()))
}

pub fn parse_frontend_merkle_root(frontend_merkle_root: &str) -> Result<FrontendMerkleRoot> {
    Ok(FrontendMerkleRoot(
        note_from_bytes(frontend_merkle_root.as_bytes()).0,
    ))
}

pub fn parse_frontend_account(frontend_account: &str) -> Result<FrontendAccount> {
    Ok(FrontendAccount(
        frontend_account.as_bytes().try_into().unwrap(),
    ))
}

pub fn parse_frontend_merkle_path_single(
    frontend_merkle_path_single: &str,
) -> Result<FrontendMerklePathNode> {
    Ok(FrontendMerklePathNode(
        note_from_bytes(frontend_merkle_path_single.as_bytes()).0,
    ))
}
