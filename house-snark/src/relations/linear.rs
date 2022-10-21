use ark_ff::PrimeField;
use ark_r1cs_std::{
    prelude::{AllocVar, EqGadget},
    uint32::UInt32,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use crate::GetPublicInput;

/// Relation with:
///  - 1 private witness (x)
///  - 2 constant        (a, y)
/// such that: 2*x + a = y.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LinearEqRelation {
    pub x: u32,
    pub a: u32,
    pub y: u32,
}

impl LinearEqRelation {
    pub fn new(x: u32, a: u32, y: u32) -> Self {
        Self { x, a, y }
    }
}

impl Default for LinearEqRelation {
    // 2*7 + 5 = 19
    fn default() -> Self {
        LinearEqRelation::new(7, 5, 19)
    }
}

impl<Field: PrimeField> ConstraintSynthesizer<Field> for LinearEqRelation {
    fn generate_constraints(self, cs: ConstraintSystemRef<Field>) -> Result<(), SynthesisError> {
        // Watch out for overflows!!!

        let x = UInt32::new_witness(ark_relations::ns!(cs, "x"), || Ok(&self.x))?;

        let a = UInt32::new_constant(ark_relations::ns!(cs, "a"), &self.a)?;
        let y = UInt32::new_constant(ark_relations::ns!(cs, "y"), &self.y)?;

        let xx_a = UInt32::addmany(&[x.clone(), x, a])?;

        xx_a.enforce_equal(&y)
    }
}

impl GetPublicInput for LinearEqRelation {}
