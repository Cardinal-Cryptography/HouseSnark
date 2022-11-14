use ark_crypto_primitives::{
    crh::{pedersen::Parameters, TwoToOneCRH},
    merkle_tree::Config,
    MerkleTree, Path, CRH,
};
use ark_ed_on_bls12_381::EdwardsProjective;
use ark_std::rand::{prelude::StdRng, SeedableRng};

use crate::relations::merkle_tree::hash_functions::{LeafHash, TwoToOneHash};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

/// A Merkle tree containing some bytes.
pub type SimpleMerkleTree = MerkleTree<MerkleConfig>;
/// The root of the byte Merkle tree.
pub type Root = <TwoToOneHash as TwoToOneCRH>::Output;
/// A membership proof for a given byte.
pub type SimplePath = Path<MerkleConfig>;

/// Creates a merkle tree over an array of 8 bytes (with values from [0..7]).
///
/// Returns a tuple of:
///  - the tree
///  - the parameters of leaf hashing function
///  - the parameters of node combining hashing function
///  - the array of leaf values
pub fn default_tree() -> (
    MerkleTree<MerkleConfig>,
    Parameters<EdwardsProjective>,
    Parameters<EdwardsProjective>,
    [u8; 8],
) {
    let mut rng = StdRng::from_seed([0u8; 32]);

    let leaf_crh_params = <LeafHash as CRH>::setup(&mut rng).unwrap();
    let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

    let leaves = [0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8];
    let tree = SimpleMerkleTree::new(&leaf_crh_params, &two_to_one_crh_params, &leaves).unwrap();

    (tree, leaf_crh_params, two_to_one_crh_params, leaves)
}
