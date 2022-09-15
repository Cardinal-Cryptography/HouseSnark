mod gadgets;
mod hash_functions;
mod relation;
mod tree;

use ark_ed_on_bls12_381::Fr;
pub use relation::MerkleTreeRelation;

pub type ConstraintF = Fr;
