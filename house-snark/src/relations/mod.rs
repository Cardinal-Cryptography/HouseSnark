mod linear;
mod merkle_tree;
mod xor;

use ark_ff::{One, PrimeField, Zero};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef};
use ark_serialize::CanonicalSerialize;
use clap::{Args, FromArgMatches, Subcommand, ValueEnum};
pub use linear::LinearEqRelation;
pub use merkle_tree::MerkleTreeRelation;
pub use xor::XorRelation;

use crate::CircuitField;

/// All implemented relations.
///
/// They should have corresponding definition in submodule.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Subcommand)]
pub enum Relation {
    Xor(XorRelation),
    LinearEquation,
    MerkleTree,
}

impl Relation {
    /// Relation identifier.
    pub fn id(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl ConstraintSynthesizer<CircuitField> for Relation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> ark_relations::r1cs::Result<()> {
        match self {
            Relation::Xor(XorRelation {
                public_xoree,
                private_xoree,
                result,
            }) => XorRelation::new(public_xoree, private_xoree, result).generate_constraints(cs),
            Relation::LinearEquation => LinearEqRelation::default().generate_constraints(cs),
            Relation::MerkleTree => MerkleTreeRelation::default().generate_constraints(cs),
        }
    }
}

pub trait GetPublicInput<CircuitField: PrimeField + CanonicalSerialize> {
    fn public_input(&self) -> Vec<CircuitField> {
        vec![]
    }
}

impl GetPublicInput<CircuitField> for Relation {
    fn public_input(&self) -> Vec<CircuitField> {
        match self {
            Relation::Xor(XorRelation { public_xoree, .. }) => byte_to_bits(*public_xoree).to_vec(),
            Relation::LinearEquation => LinearEqRelation::default().public_input(),
            Relation::MerkleTree => MerkleTreeRelation::default().public_input(),
        }
    }
}

/// Convert `u8` into an 8-tuple of bits over `F` (little endian).
fn byte_to_bits<F: Zero + One + Copy>(byte: u8) -> [F; 8] {
    let mut bits = [F::zero(); 8];
    for (idx, bit) in bits.iter_mut().enumerate() {
        if (byte >> idx) & 1 == 1 {
            *bit = F::one();
        }
    }
    bits
}
