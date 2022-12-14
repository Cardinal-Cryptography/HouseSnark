use ark_ff::PrimeField;
use ark_r1cs_std::prelude::{AllocVar, EqGadget, UInt8};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_serialize::CanonicalSerialize;
use clap::Args;

use crate::relations::{byte_to_bits, GetPublicInput};

/// XOR relation: a ⊕ b = c
///
/// Relation with:
///  - 1 public input    (a | `public_xoree`)
///  - 1 private witness (b | `private_xoree`)
///  - 1 constant        (c | `result`)
/// such that: a ^ b = c.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct XorRelation {
    // ToDo: Especially for Groth16, it is better to provide public input as a field element.
    // Otherwise, we have to provide it to circuit bit by bit.
    #[clap(long, short = 'a', default_value = "2")]
    pub public_xoree: u8,
    #[clap(long, short = 'b', default_value = "3")]
    pub private_xoree: u8,
    #[clap(long, short = 'c', default_value = "1")]
    pub result: u8,
}

impl<Field: PrimeField> ConstraintSynthesizer<Field> for XorRelation {
    fn generate_constraints(self, cs: ConstraintSystemRef<Field>) -> Result<(), SynthesisError> {
        let public_xoree = UInt8::new_input(ark_relations::ns!(cs, "public_xoree"), || {
            Ok(&self.public_xoree)
        })?;
        let private_xoree = UInt8::new_witness(ark_relations::ns!(cs, "private_xoree"), || {
            Ok(&self.private_xoree)
        })?;
        let result = UInt8::new_constant(ark_relations::ns!(cs, "result"), &self.result)?;

        let xor = UInt8::xor(&public_xoree, &private_xoree)?;
        xor.enforce_equal(&result)
    }
}

impl<CircuitField: PrimeField + CanonicalSerialize> GetPublicInput<CircuitField> for XorRelation {
    fn public_input(&self) -> Vec<CircuitField> {
        byte_to_bits(self.public_xoree).to_vec()
    }
}
