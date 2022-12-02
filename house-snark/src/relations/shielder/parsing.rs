use std::str::FromStr;

use anyhow::Result;

use super::types::{
    FrontendAccount, FrontendLeafIndex, FrontendMerklePathNode, FrontendMerkleRoot, FrontendNote,
    FrontendNullifier, FrontendTokenAmount, FrontendTokenId, FrontendTrapdoor,
};
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

macro_rules! parse_frontend_type {
    ($frontend_type:tt, $inner_type:ty) => {
        impl FromStr for $frontend_type {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(
                    <$inner_type>::from_str(s).map_err(|e| format!("Failed to parse: {e:?}"))?,
                ))
            }
        }
    };
}

parse_frontend_type!(FrontendNullifier, u64);
parse_frontend_type!(FrontendTrapdoor, u64);
parse_frontend_type!(FrontendTokenId, u16);
parse_frontend_type!(FrontendTokenAmount, u64);
parse_frontend_type!(FrontendLeafIndex, u64);
