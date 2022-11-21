mod relations;

#[cfg(any(feature = "deposit", feature = "withdraw"))]
pub use relations::blender::compute_note;
#[cfg(feature = "deposit")]
pub use relations::blender::DepositRelation;
#[cfg(feature = "withdraw")]
pub use relations::blender::WithdrawRelation;
#[cfg(feature = "linear")]
pub use relations::LinearEqRelation;
#[cfg(feature = "merkle_tree")]
pub use relations::MerkleTreeRelation;
#[cfg(feature = "xor")]
pub use relations::XorRelation;
