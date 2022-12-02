//! This module contains two relations that are the core of the Shielder application: `deposit` and
//! `withdraw`. It also exposes some functions and types that might be useful for input generation.
//!
//! Currently, instead of using some real hash function, we chose to incorporate a simple tangling
//! algorithm. Essentially, it is a procedure that just mangles a byte sequence.

mod deposit;
#[allow(dead_code)]
mod note;
mod parsing;
mod tangle;
mod types;
mod withdraw;

pub use deposit::{DepositRelation, DepositRelationArgs};
pub use note::compute_note;
pub use types::{
    FrontendMerklePath as MerklePath, FrontendMerkleRoot as MerkleRoot, FrontendNote as Note,
    FrontendNullifier as Nullifier, FrontendTokenAmount as TokenAmount, FrontendTokenId as TokenId,
    FrontendTrapdoor as Trapdoor,
};
pub use withdraw::{WithdrawRelation, WithdrawRelationArgs};

pub use crate::relations::types::CircuitField;
