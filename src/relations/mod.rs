// mod merkle_tree;
// pub use merkle_tree::MerkleTreeRelation;

mod linear;
mod xor;

use ark_ec::PairingEngine;
use ark_groth16::{Proof, VerifyingKey};
use clap::ValueEnum;
pub use linear::LinearEqRelation;
pub use xor::XorRelation;

/// All implemented relations.
///
/// They should have corresponding definition in submodule.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum Relation {
    Xor,
    MerkleTree,
    LinearEquation,
}

/// Output of some SNARK relation.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Artifacts<VK, P, PI> {
    pub verifying_key: VK,
    pub proof: P,
    pub public_input: PI,
}

/// Artifacts that are produced directly by relation, without any conversions.
pub type PureArtifacts<Pairing, FieldElement> =
    Artifacts<VerifyingKey<Pairing>, Proof<Pairing>, Vec<FieldElement>>;

/// Common interface for the relations.
pub trait SnarkRelation<Pairing: PairingEngine, FieldElement>: Default {
    /// String identifier of relation.
    ///
    /// By default, empty string.
    fn id() -> &'static str {
        ""
    }

    /// Produce pure artifacts.
    fn generate_artifacts(&self) -> PureArtifacts<Pairing, FieldElement>;
}
