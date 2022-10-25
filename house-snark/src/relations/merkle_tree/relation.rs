use ark_crypto_primitives::{
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
    PathVar, CRH,
};
use ark_r1cs_std::{boolean::Boolean, eq::EqGadget, prelude::AllocVar, uint8::UInt8};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use crate::{
    relations::{
        byte_to_bits,
        merkle_tree::{
            gadgets::{
                LeafHashGadget, LeafHashParamsVar, TwoToOneHashGadget, TwoToOneHashParamsVar,
            },
            hash_functions::{LeafHash, TwoToOneHash},
            tree::{default_tree, MerkleConfig, Root, SimplePath},
        },
    },
    CircuitField, GetPublicInput,
};

/// The R1CS equivalent of the the Merkle tree root.
pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::OutputVar;

/// The R1CS equivalent of the the Merkle tree path.
pub type SimplePathVar = PathVar<MerkleConfig, LeafHashGadget, TwoToOneHashGadget, CircuitField>;

/// Relation for checking membership in a Merkle tree.
///
/// `MerkleTreeRelation` comes with the default instantiation, where it represents a membership
/// proof for the first leaf (at index 0) in a tree over 8 bytes (`[0u8,..,7u8]`). The underlying
/// tree (together with its hash function parameters) is generated from the function
/// `default_tree()`.
#[derive(Clone)]
pub struct MerkleTreeRelation {
    /// Private witness.
    pub authentication_path: SimplePath,

    /// Root of the tree (public input).
    pub root: Root,
    /// Leaf which membership is to be proven (public input).
    pub leaf: u8,

    /// Collision-resistant hash function for leafs (constant parameter).
    pub leaf_crh_params: <LeafHash as CRH>::Parameters,
    /// Collision-resistant hash function translating child hashes to parent hash
    /// (constant parameter).
    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl Default for MerkleTreeRelation {
    fn default() -> Self {
        let (tree, leaf_crh_params, two_to_one_crh_params, leaves) = default_tree();

        let leaf_idx = 0;

        MerkleTreeRelation {
            authentication_path: tree.generate_proof(leaf_idx).unwrap(),
            root: tree.root(),
            leaf: leaves[leaf_idx],
            leaf_crh_params,
            two_to_one_crh_params,
        }
    }
}

impl ConstraintSynthesizer<CircuitField> for MerkleTreeRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let path = SimplePathVar::new_witness(ark_relations::ns!(cs, "path_var"), || {
            Ok(self.authentication_path)
        })?;

        let root = RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.root))?;
        let leaf = UInt8::new_input(ark_relations::ns!(cs, "leaf_var"), || Ok(&self.leaf))?;

        let leaf_crh_params = LeafHashParamsVar::new_constant(cs.clone(), &self.leaf_crh_params)?;
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs, &self.two_to_one_crh_params)?;

        let leaf_bytes = vec![leaf; 1];

        let is_member = path.verify_membership(
            &leaf_crh_params,
            &two_to_one_crh_params,
            &root,
            &leaf_bytes.as_slice(),
        )?;
        is_member.enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}

impl GetPublicInput<CircuitField> for MerkleTreeRelation {
    fn public_input(&self) -> Vec<CircuitField> {
        [vec![self.root], byte_to_bits(self.leaf).to_vec()].concat()
    }
}
