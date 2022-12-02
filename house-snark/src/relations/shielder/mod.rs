//! This module contains two relations that are the core of the Shielder application: `deposit` and
//! `withdraw`. It also exposes some functions and types that might be useful for input generation.
//!
//! Currently, instead of using some real hash function, we chose to incorporate a simple tangling
//! algorithm. Essentially, it is a procedure that just mangles a byte sequence.

mod deposit;
#[allow(dead_code)] // Actually it is needed, even though we are reexporting items from there.
mod note;
#[cfg(feature = "cli")]
mod parser;
mod tangle;
mod types;
mod withdraw;

pub use deposit::{DepositRelation, DepositRelationArgs};
pub use note::{bytes_from_note, compute_note, compute_parent_hash, note_from_bytes};
pub use types::{
    FrontendMerklePath as MerklePath, FrontendMerkleRoot as MerkleRoot, FrontendNote as Note,
    FrontendNullifier as Nullifier, FrontendTokenAmount as TokenAmount, FrontendTokenId as TokenId,
    FrontendTrapdoor as Trapdoor,
};
pub use withdraw::{WithdrawRelation, WithdrawRelationArgs};

pub use crate::relations::types::CircuitField;
