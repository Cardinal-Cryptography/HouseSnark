use ark_ff::PrimeField;
use ark_r1cs_std::prelude::{AllocVar, EqGadget, UInt8};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::rand::{prelude::StdRng, SeedableRng};

use crate::{
    relations::{byte_to_bits, PureKeys, PureProvingArtifacts, SnarkRelation},
    system::Environment,
    ProvingKey,
};

/// Relation with:
///  - 1 public input    (a | `public_xoree`)
///  - 1 private witness (b | `private_xoree`)
///  - 1 constant        (c | `result`)
/// such that: a ^ b = c.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct XorRelation {
    // ToDo: Especially for Groth16, it is better to provide public input as a field element.
    // Otherwise, we have to provide it to circuit bit by bit.
    pub public_xoree: u8,
    pub private_xoree: u8,
    pub result: u8,
}

impl XorRelation {
    pub fn new(public_xoree: u8, private_xoree: u8, result: u8) -> Self {
        Self {
            public_xoree,
            private_xoree,
            result,
        }
    }
}

impl Default for XorRelation {
    fn default() -> Self {
        XorRelation::new(2, 3, 1)
    }
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

impl<Env: Environment> SnarkRelation<Env> for XorRelation {
    fn id(&self) -> &'static str {
        "xor"
    }

    fn generate_keys(&self) -> PureKeys<Env> {
        let mut rng = StdRng::from_seed([0u8; 32]);

        let (proving_key, verifying_key) =
            Env::setup(*self, &mut rng).unwrap_or_else(|e| panic!("Problems with setup: {:?}", e));

        PureKeys::<Env> {
            proving_key,
            verifying_key,
        }
    }

    fn generate_proof(&self, proving_key: ProvingKey<Env>) -> PureProvingArtifacts<Env> {
        let mut rng = StdRng::from_seed([0u8; 32]);

        let public_input = byte_to_bits(self.public_xoree);

        let proof = Env::prove(&proving_key, *self, &mut rng)
            .unwrap_or_else(|e| panic!("Cannot prove: {:?}", e));

        PureProvingArtifacts::<Env> {
            proof,
            public_input: public_input.to_vec(),
        }
    }
}
