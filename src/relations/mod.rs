mod linear;
mod xor;

use ark_ff::{One, Zero};
use clap::ValueEnum;
pub use linear::LinearEqRelation;
pub use xor::XorRelation;

use crate::system::{Environment, Fr, Proof, ProvingKey, VerifyingKey};

/// All implemented relations.
///
/// They should have corresponding definition in submodule.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum Relation {
    Xor,
    LinearEquation,
}

impl Relation {
    pub fn as_snark_relation<Env: Environment>(self) -> Box<dyn SnarkRelation<Env>> {
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
pub type PureKeys<Env> = Keys<VerifyingKey<Env>, ProvingKey<Env>>;
pub type PureProvingArtifacts<Env> = ProvingArtifacts<Proof<Env>, Vec<Fr<Env>>>;

/// Common interface for the relations.
pub trait SnarkRelation<Env: Environment> {
    /// String identifier of relation.
    fn id(&self) -> &'static str;

    /// Produce keys (in a pure form).
    fn generate_keys(&self) -> PureKeys<Env>;

    /// Produce proof and a public input (in a pure form).
    fn generate_proof(&self, proving_key: ProvingKey<Env>) -> PureProvingArtifacts<Env>;
}

impl<Env: Environment> SnarkRelation<Env> for Box<dyn SnarkRelation<Env>> {
    fn id(&self) -> &'static str {
        self.as_ref().id()
    }

    fn generate_keys(&self) -> PureKeys<Env> {
        self.as_ref().generate_keys()
    }

    fn generate_proof(&self, proving_key: ProvingKey<Env>) -> PureProvingArtifacts<Env> {
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
