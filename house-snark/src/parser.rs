use anyhow::Result as AnyResult;
use relations::{
    note_from_bytes, FrontendAccount, FrontendMerklePathNode, FrontendMerkleRoot, FrontendNote,
};

pub fn parse_frontend_note(frontend_note: &str) -> AnyResult<FrontendNote> {
    Ok(note_from_bytes(frontend_note.as_bytes()))
}

pub fn parse_frontend_merkle_root(frontend_merkle_root: &str) -> AnyResult<FrontendMerkleRoot> {
    Ok(note_from_bytes(frontend_merkle_root.as_bytes()))
}

pub fn parse_frontend_account(frontend_account: &str) -> AnyResult<FrontendAccount> {
    Ok(frontend_account.as_bytes().try_into().unwrap())
}

pub fn parse_frontend_merkle_path_single(
    frontend_merkle_path_single: &str,
) -> AnyResult<FrontendMerklePathNode> {
    Ok(note_from_bytes(frontend_merkle_path_single.as_bytes()))
}
