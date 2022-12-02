mod environment;
mod relations;
pub mod serialization;

pub use environment::{CircuitField, NonUniversalProvingSystem, RawKeys, SomeProvingSystem};
#[cfg(feature = "deposit")]
pub use relations::shielder::DepositRelation;
#[cfg(feature = "withdraw")]
pub use relations::shielder::WithdrawRelation;
#[cfg(any(feature = "deposit", feature = "withdraw"))]
pub use relations::shielder::{
    bytes_from_note, compute_note, note_from_bytes, MerklePath, MerkleRoot, Note, Nullifier,
    TokenAmount, TokenId, Trapdoor,
};
pub use relations::GetPublicInput;
#[cfg(feature = "linear")]
pub use relations::LinearEqRelation;
#[cfg(feature = "merkle_tree")]
pub use relations::MerkleTreeRelation;
#[cfg(feature = "xor")]
pub use relations::XorRelation;
