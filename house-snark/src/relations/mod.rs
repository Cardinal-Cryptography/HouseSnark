mod linear;
mod xor;

use ark_ff::{One, PrimeField, Zero};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef};
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
    /// Relation identifier.
    pub fn id(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

impl<CircuitField: PrimeField> ConstraintSynthesizer<CircuitField> for Relation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> ark_relations::r1cs::Result<()> {
        match self {
            Relation::Xor => XorRelation::default().generate_constraints(cs),
            Relation::LinearEquation => LinearEqRelation::default().generate_constraints(cs),
        }
    }
}

pub trait GetPublicInput {
    fn public_input<CircuitField: PrimeField>(&self) -> Vec<CircuitField> {
        vec![]
    }
}

impl GetPublicInput for Relation {
    fn public_input<CircuitField: PrimeField>(&self) -> Vec<CircuitField> {
        match self {
            Relation::Xor => XorRelation::default().public_input(),
            Relation::LinearEquation => LinearEqRelation::default().public_input(),
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
