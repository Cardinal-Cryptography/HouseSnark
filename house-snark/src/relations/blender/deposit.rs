use ark_ff::BigInteger256;
use ark_r1cs_std::alloc::AllocVar;
use ark_relations::{
    ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};

use super::{
    note::check_note,
    types::{
        BackendNote, BackendNullifier, BackendTokenAmount, BackendTokenId, BackendTrapdoor, FpVar,
        FrontendNote, FrontendNullifier, FrontendTokenAmount, FrontendTokenId, FrontendTrapdoor,
    },
};
use crate::relations::types::CircuitField;

/// 'Deposit' relation for the Blender application.
///
/// It expresses the fact that `note` is a prefix of the result of tangling together `token_id`,
/// `token_amount`, `trapdoor` and `nullifier`.
#[derive(Default, Clone, Copy)]
pub struct DepositRelation {
    // Public inputs.
    pub note: BackendNote,
    pub token_id: BackendTokenId,
    pub token_amount: BackendTokenAmount,

    // Private inputs.
    pub trapdoor: BackendTrapdoor,
    pub nullifier: BackendNullifier,
}

impl DepositRelation {
    pub fn new(
        note: FrontendNote,
        token_id: FrontendTokenId,
        token_amount: FrontendTokenAmount,
        trapdoor: FrontendTrapdoor,
        nullifier: FrontendNullifier,
    ) -> Self {
        Self {
            note: BackendNote::from(BigInteger256::new(note)),
            token_id: BackendTokenId::from(token_id),
            token_amount: BackendTokenAmount::from(token_amount),
            trapdoor: BackendTrapdoor::from(trapdoor),
            nullifier: BackendNullifier::from(nullifier),
        }
    }
}

impl ConstraintSynthesizer<CircuitField> for DepositRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let token_id = FpVar::new_input(ns!(cs, "token id"), || Ok(&self.token_id))?;
        let token_amount = FpVar::new_input(ns!(cs, "token amount"), || Ok(&self.token_amount))?;
        let note = FpVar::new_input(ns!(cs, "note"), || Ok(&self.note))?;

        let trapdoor = FpVar::new_witness(ns!(cs, "trapdoor"), || Ok(&self.trapdoor))?;
        let nullifier = FpVar::new_witness(ns!(cs, "nullifier"), || Ok(&self.nullifier))?;

        check_note(&token_id, &token_amount, &trapdoor, &nullifier, &note)
    }
}

#[cfg(test)]
mod tests {
    use ark_relations::r1cs::ConstraintSystem;

    use super::*;
    use crate::relations::blender::note::compute_note;

    #[test]
    fn deposit_constraints_correctness() {
        let token_id: FrontendTokenId = 1;
        let token_amount: FrontendTokenAmount = 10;
        let trapdoor: FrontendTrapdoor = 17;
        let nullifier: FrontendNullifier = 19;
        let note = compute_note(token_id, token_amount, trapdoor, nullifier);

        let circuit = DepositRelation::new(note, token_id, token_amount, trapdoor, nullifier);

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        let is_satisfied = cs.is_satisfied().unwrap();
        if !is_satisfied {
            println!("{:?}", cs.which_is_unsatisfied());
        }

        assert!(is_satisfied);
    }
}
