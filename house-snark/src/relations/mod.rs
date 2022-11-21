#[cfg(any(feature = "deposit", feature = "withdraw"))]
pub mod blender;
#[cfg(feature = "linear")]
mod linear;
#[cfg(feature = "merkle_tree")]
mod merkle_tree;
mod types;
#[cfg(feature = "xor")]
mod xor;

use ark_ff::{One, PrimeField, Zero};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef};
use ark_serialize::CanonicalSerialize;
#[cfg(feature = "deposit")]
pub use blender::DepositRelation;
#[cfg(feature = "withdraw")]
pub use blender::WithdrawRelation;
use clap::Subcommand;
#[cfg(feature = "linear")]
pub use linear::LinearEqRelation;
#[cfg(feature = "merkle_tree")]
pub use merkle_tree::{MerkleTreeRelation, MerkleTreeRelationArgs};
#[cfg(feature = "xor")]
pub use xor::XorRelation;

use crate::relations::types::CircuitField;

/// All implemented relations.
///
/// They should have corresponding definition in submodule.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Subcommand)]
pub enum Relation {
    #[cfg(feature = "xor")]
    Xor(XorRelation),
    #[cfg(feature = "linear")]
    LinearEquation(LinearEqRelation),
    #[cfg(feature = "merkle_tree")]
    MerkleTree(MerkleTreeRelationArgs),
    #[cfg(feature = "deposit")]
    Deposit(DepositRelation),
    #[cfg(feature = "withdraw")]
    Withdraw(WithdrawRelation),
}

impl Relation {
    /// Relation identifier.
    #[allow(dead_code)]
    pub fn id(&self) -> String {
        match &self {
            #[cfg(feature = "xor")]
            Relation::Xor(_) => String::from("xor"),
            #[cfg(feature = "linear")]
            Relation::LinearEquation(_) => String::from("linear_equation"),
            #[cfg(feature = "merkle_tree")]
            Relation::MerkleTree(_) => String::from("merkle_tree"),
            #[cfg(feature = "deposit")]
            Relation::Deposit(_) => String::from("deposit"),
            #[cfg(feature = "withdraw")]
            Relation::Withdraw(_) => String::from("withdraw"),
        }
    }
}

impl ConstraintSynthesizer<CircuitField> for Relation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> ark_relations::r1cs::Result<()> {
        match self {
            #[cfg(feature = "xor")]
            Relation::Xor(relation @ XorRelation { .. }) => relation.generate_constraints(cs),
            #[cfg(feature = "linear")]
            Relation::LinearEquation(relation @ LinearEqRelation { .. }) => {
                relation.generate_constraints(cs)
            }
            #[cfg(feature = "merkle_tree")]
            Relation::MerkleTree(args @ MerkleTreeRelationArgs { .. }) => {
                <MerkleTreeRelationArgs as Into<MerkleTreeRelation>>::into(args)
                    .generate_constraints(cs)
            }
            #[cfg(feature = "deposit")]
            Relation::Deposit(relation @ DepositRelation { .. }) => {
                relation.generate_constraints(cs)
            }
            #[cfg(feature = "withdraw")]
            Relation::Withdraw(relation @ WithdrawRelation { .. }) => {
                relation.generate_constraints(cs)
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
            #[cfg(feature = "xor")]
            Relation::Xor(relation @ XorRelation { .. }) => relation.public_input(),
            #[cfg(feature = "linear")]
            Relation::LinearEquation(relation @ LinearEqRelation { .. }) => relation.public_input(),
            #[cfg(feature = "merkle_tree")]
            Relation::MerkleTree(args @ MerkleTreeRelationArgs { .. }) => {
                <MerkleTreeRelationArgs as Into<MerkleTreeRelation>>::into(args.to_owned())
                    .public_input()
            }
            #[cfg(feature = "deposit")]
            Relation::Deposit(relation @ DepositRelation { .. }) => relation.public_input(),
            #[cfg(feature = "withdraw")]
            Relation::Withdraw(relation @ WithdrawRelation { .. }) => relation.public_input(),
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
#[cfg(feature = "merkle_tree")]
fn string_to_padded_bytes(s: String) -> [u8; 32] {
    let mut bytes: Vec<u8> = s.as_bytes().into();
    bytes.resize(32, 0);
    bytes.try_into().expect("this should never fail")
}
