use ark_crypto_primitives::{
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
    merkle_tree::Config,
    CRHGadget, MerkleTree, Path, PathVar, CRH,
};
use ark_ff::{PrimeField, ToBytes};
use ark_r1cs_std::{
    prelude::{AllocVar, Boolean, EqGadget, UInt8},
    uint128::UInt128,
    ToBytesGadget,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::{
    hash::Hash,
    rand::{prelude::StdRng, SeedableRng},
};

use super::{
    gadgets::LeafHashGadget,
    hash_functions::{LeafHash, TwoToOneHash},
};

pub type Root = <TwoToOneHash as TwoToOneCRH>::Output;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PoseidonMerkleTreeConfig;
impl Config for PoseidonMerkleTreeConfig {
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

/// A Merkle tree containing some bytes.
pub type PoseidonMerkleTree = MerkleTree<PoseidonMerkleTreeConfig>;

/// Creates a new tree of Notes
pub fn new(leaves: Vec<Vec<u8>>, seed: [u8; 32]) -> PoseidonMerkleTree {
    let mut rng = StdRng::from_seed(seed);
    let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
    let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();
    PoseidonMerkleTree::new(&leaf_crh_params, &two_to_one_crh_params, &leaves).unwrap()
}

pub type PoseidonMerkleTreePath = Path<PoseidonMerkleTreeConfig>;
