mod linear;
mod xor;

use ark_ec::PairingEngine;
use ark_ff::{One, Zero};
use ark_groth16::{Proof, ProvingKey, VerifyingKey};
use clap::ValueEnum;
pub use linear::LinearEqRelation;
pub use xor::XorRelation;

/// All implemented relations.
///
/// They should have corresponding definition in submodule.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum Relation {
    Xor,
    LinearEquation,
}

impl Relation {
    pub fn as_snark_relation<Pairing: PairingEngine>(self) -> Box<dyn SnarkRelation<Pairing>> {
        match self {
            Relation::Xor => Box::new(XorRelation::default()),
            Relation::LinearEquation => Box::new(LinearEqRelation::default()),
        }
    }
}

/// Pair of keys resulted from a setup process.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Keys<VK, PK> {
    pub verifying_key: VK,
    pub proving_key: PK,
}

/// Proof accompanied by a public input.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ProvingArtifacts<P, PI> {
    pub proof: P,
    pub public_input: PI,
}

/// Artifacts that are produced directly by relation, without any conversions.
pub type PureKeys<Pairing> = Keys<VerifyingKey<Pairing>, ProvingKey<Pairing>>;
pub type PureProvingArtifacts<Pairing> =
    ProvingArtifacts<Proof<Pairing>, Vec<<Pairing as PairingEngine>::Fr>>;

/// Common interface for the relations.
pub trait SnarkRelation<Pairing: PairingEngine> {
    /// String identifier of relation.
    fn id(&self) -> &'static str;

    /// Produce keys (in a pure form).
    fn generate_keys(&self) -> PureKeys<Pairing>;

    /// Produce proof and a public input (in a pure form).
    fn generate_proof(&self, proving_key: ProvingKey<Pairing>) -> PureProvingArtifacts<Pairing>;
}

impl<P: PairingEngine> SnarkRelation<P> for Box<dyn SnarkRelation<P>> {
    fn id(&self) -> &'static str {
        self.as_ref().id()
    }

    fn generate_keys(&self) -> PureKeys<P> {
        self.as_ref().generate_keys()
    }

    fn generate_proof(&self, proving_key: ProvingKey<P>) -> PureProvingArtifacts<P> {
        self.as_ref().generate_proof(proving_key)
    }
}

fn byte_to_bits<F: Zero + One + Copy>(byte: u8) -> [F; 8] {
    let mut bits = [F::zero(); 8];
    for (idx, bit) in bits.iter_mut().enumerate() {
        if (byte >> idx) & 1 == 1 {
            *bit = F::one();
        }
    }
    bits
}
