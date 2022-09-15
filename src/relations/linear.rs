use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::Groth16;
use ark_r1cs_std::{
    prelude::{AllocVar, EqGadget},
    uint32::UInt32,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef};
use ark_snark::SNARK;
use ark_std::rand::{prelude::StdRng, SeedableRng};

use crate::relations::{PureArtifacts, SnarkRelation};

pub type ConstraintF = Fr;

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

impl ConstraintSynthesizer<ConstraintF> for LinearEqRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> ark_relations::r1cs::Result<()> {
        // Watch out for overflows!!!

        let x = UInt32::new_witness(ark_relations::ns!(cs, "x"), || Ok(&self.x))?;

        let a = UInt32::new_constant(ark_relations::ns!(cs, "a"), &self.a)?;
        let y = UInt32::new_constant(ark_relations::ns!(cs, "y"), &self.y)?;

        let xx_a = UInt32::addmany(&[x.clone(), x, a])?;

        xx_a.enforce_equal(&y)
    }
}

impl SnarkRelation<Bls12_381, ConstraintF> for LinearEqRelation {
    fn id() -> &'static str {
        "linear-equation"
    }

    fn generate_artifacts(&self) -> PureArtifacts<Bls12_381, ConstraintF> {
        let mut rng = StdRng::from_seed([0u8; 32]);

        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(*self, &mut rng)
            .unwrap_or_else(|e| panic!("Problems with setup: {:?}", e));

        let proof = Groth16::prove(&pk, *self, &mut rng)
            .unwrap_or_else(|e| panic!("Cannot prove: {:?}", e));

        PureArtifacts {
            verifying_key: vk,
            proof,
            public_input: vec![],
        }
    }
}
