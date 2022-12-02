use ark_r1cs_std::alloc::AllocVar;
use ark_relations::{
    ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use clap::Args;

use super::{
    note::check_note,
    parser::parse_frontend_note,
    types::{
        BackendNote, BackendNullifier, BackendTokenAmount, BackendTokenId, BackendTrapdoor, FpVar,
        FrontendNote, FrontendNullifier, FrontendTokenAmount, FrontendTokenId, FrontendTrapdoor,
    },
};
use crate::relations::{types::CircuitField, GetPublicInput};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct DepositRelationArgs {
    // Public inputs.
    #[clap(long, value_parser = parse_frontend_note)]
    pub note: FrontendNote,
    #[clap(long)]
    pub token_id: FrontendTokenId,
    #[clap(long)]
    pub token_amount: FrontendTokenAmount,

    // Private inputs.
    #[clap(long)]
    pub trapdoor: FrontendTrapdoor,
    #[clap(long)]
    pub nullifier: FrontendNullifier,
}

/// 'Deposit' relation for the Shielder application.
///
/// It expresses the fact that `note` is a prefix of the result of tangling together `token_id`,
/// `token_amount`, `trapdoor` and `nullifier`.
///
/// When providing a public input to proof verification, you should keep the order of variable
/// declarations in circuit, i.e.: `note`, `token_id`, `token_amount`.
#[derive(Clone)]
pub struct DepositRelation {
    // Public inputs.
    note: BackendNote,
    token_id: BackendTokenId,
    token_amount: BackendTokenAmount,

    // Private inputs.
    trapdoor: BackendTrapdoor,
    nullifier: BackendNullifier,
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
            note: note.into(),
            token_id: token_id.into(),
            token_amount: token_amount.into(),
            trapdoor: trapdoor.into(),
            nullifier: nullifier.into(),
        }
    }
}

impl From<DepositRelationArgs> for DepositRelation {
    fn from(args: DepositRelationArgs) -> Self {
        let DepositRelationArgs {
            note,
            token_id,
            token_amount,
            trapdoor,
            nullifier,
        } = args;
        DepositRelation::new(note, token_id, token_amount, trapdoor, nullifier)
    }
}

impl ConstraintSynthesizer<CircuitField> for DepositRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let note = FpVar::new_input(ns!(cs, "note"), || Ok(&self.note.0))?;
        let token_id = FpVar::new_input(ns!(cs, "token id"), || Ok(&self.token_id.0))?;
        let token_amount = FpVar::new_input(ns!(cs, "token amount"), || Ok(&self.token_amount.0))?;

        let trapdoor = FpVar::new_witness(ns!(cs, "trapdoor"), || Ok(&self.trapdoor.0))?;
        let nullifier = FpVar::new_witness(ns!(cs, "nullifier"), || Ok(&self.nullifier.0))?;

        check_note(&token_id, &token_amount, &trapdoor, &nullifier, &note)
    }
}

impl GetPublicInput<CircuitField> for DepositRelation {
    fn public_input(&self) -> Vec<CircuitField> {
        vec![self.note.0, self.token_id.0, self.token_amount.0]
    }
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::Bls12_381;
    use ark_groth16::Groth16;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_snark::SNARK;

    use super::*;
    use crate::relations::shielder::note::compute_note;

    fn get_circuit_and_input() -> (DepositRelation, [CircuitField; 3]) {
        let token_id = FrontendTokenId(1);
        let token_amount = FrontendTokenAmount(10);
        let trapdoor = FrontendTrapdoor(17);
        let nullifier = FrontendNullifier(19);
        let note = compute_note(token_id, token_amount, trapdoor, nullifier);

        let circuit = DepositRelation::new(note, token_id, token_amount, trapdoor, nullifier);
        let input = [
            BackendNote::from(note).0,
            BackendTokenId::from(token_id).0,
            BackendTokenAmount::from(token_amount).0,
        ];

        (circuit, input)
    }

    #[test]
    fn deposit_constraints_correctness() {
        let (circuit, _input) = get_circuit_and_input();

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        let is_satisfied = cs.is_satisfied().unwrap();
        if !is_satisfied {
            println!("{:?}", cs.which_is_unsatisfied());
        }

        assert!(is_satisfied);
    }

    #[test]
    fn deposit_proving_procedure() {
        let (circuit, input) = get_circuit_and_input();

        let mut rng = ark_std::test_rng();
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

        let proof = Groth16::prove(&pk, circuit, &mut rng).unwrap();
        let valid_proof = Groth16::verify(&vk, &input, &proof).unwrap();
        assert!(valid_proof);
    }
}
