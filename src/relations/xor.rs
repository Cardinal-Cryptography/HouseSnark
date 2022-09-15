use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::Groth16;
use ark_r1cs_std::prelude::{AllocVar, EqGadget, UInt8};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef};
use ark_snark::SNARK;
use ark_std::{
    rand::{prelude::StdRng, SeedableRng},
    One, Zero,
};

use crate::relations::{PureArtifacts, SnarkRelation};

pub type ConstraintF = Fr;

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

impl ConstraintSynthesizer<ConstraintF> for XorRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> ark_relations::r1cs::Result<()> {
        let public_xoree = UInt8::new_input(ark_relations::ns!(cs, "public_summand"), || {
            Ok(&self.public_xoree)
        })?;
        let private_xoree = UInt8::new_witness(ark_relations::ns!(cs, "private_summand"), || {
            Ok(&self.private_xoree)
        })?;
        let result = UInt8::new_constant(ark_relations::ns!(cs, "result"), &self.result)?;

        let xor = UInt8::xor(&public_xoree, &private_xoree)?;
        xor.enforce_equal(&result)
    }
}

impl SnarkRelation<Bls12_381, ConstraintF> for XorRelation {
    fn id(&self) -> &'static str {
        "xor"
    }

    fn generate_artifacts(&self) -> PureArtifacts<Bls12_381, ConstraintF> {
        let mut rng = StdRng::from_seed([0u8; 32]);

        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(*self, &mut rng)
            .unwrap_or_else(|e| panic!("Problems with setup: {:?}", e));

        let mut public_input = [ConstraintF::zero(); 8];
        for (idx, bit) in public_input.iter_mut().enumerate() {
            if (self.public_xoree >> idx) & 1 == 1 {
                *bit = ConstraintF::one();
            }
        }

        let proof = Groth16::prove(&pk, *self, &mut rng)
            .unwrap_or_else(|e| panic!("Cannot prove: {:?}", e));

        PureArtifacts {
            verifying_key: vk,
            proof,
            public_input: public_input.to_vec(),
        }
    }
}
