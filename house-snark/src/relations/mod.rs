mod linear;
mod merkle_tree;
mod xor;

use ark_ff::{One, PrimeField, Zero};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef};
use ark_serialize::CanonicalSerialize;
use clap::Subcommand;
pub use linear::LinearEqRelation;
pub use merkle_tree::{MerkleTreeRelation, MerkleTreeRelationArgs};
pub use xor::XorRelation;

use crate::CircuitField;

/// All implemented relations.
///
/// They should have corresponding definition in submodule.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Subcommand)]
pub enum Relation {
    Xor(XorRelation),
    LinearEquation(LinearEqRelation),
    MerkleTree(MerkleTreeRelationArgs),
}

impl Relation {
    /// Relation identifier.
    pub fn id(&self) -> String {
        match &self {
            Relation::Xor(_) => String::from("xor"),
            Relation::LinearEquation(_) => String::from("linear_equation"),
            Relation::MerkleTree(_) => String::from("merkle_tree"),
        }
    }
}

impl ConstraintSynthesizer<CircuitField> for Relation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> ark_relations::r1cs::Result<()> {
        match self {
            Relation::Xor(relation @ XorRelation { .. }) => relation.generate_constraints(cs),
            Relation::LinearEquation(relation @ LinearEqRelation { .. }) => {
                relation.generate_constraints(cs)
            }
            Relation::MerkleTree(args @ MerkleTreeRelationArgs { .. }) => {
                <MerkleTreeRelationArgs as Into<MerkleTreeRelation>>::into(args)
                    .generate_constraints(cs)
            }
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
            Relation::Xor(relation @ XorRelation { .. }) => relation.public_input(),
            Relation::LinearEquation(relation @ LinearEqRelation { .. }) => relation.public_input(),
            Relation::MerkleTree(args @ MerkleTreeRelationArgs { .. }) => {
                <MerkleTreeRelationArgs as Into<MerkleTreeRelation>>::into(args.to_owned())
                    .public_input()
            }
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

/// Takes a string an converts it to a 32 byte array
/// missing bytes are padded with 0's
fn string_to_padded_bytes(s: String) -> [u8; 32] {
    let mut bytes: Vec<u8> = s.as_bytes().into();
    bytes.resize(32, 0);
    bytes.try_into().expect("this should never fail")
}
