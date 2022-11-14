use ark_crypto_primitives::{
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
};
use ark_r1cs_std::{prelude::{AllocVar, EqGadget}, uint128::UInt128, ToBytesGadget};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::rand::{prelude::StdRng, SeedableRng};

use crate::{
    relations::{
        deposit::{
            gadgets::{
                TwoToOneHashGadget, TwoToOneHashParamsVar,
            },
            hash_functions::TwoToOneHash,
        },
    },
    CircuitField, GetPublicInput,
};

/// The R1CS equivalent of the the Merkle tree root.
pub type NoteVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::OutputVar;

/// Relation for checking membership in a Merkle tree.
///
/// `MerkleTreeRelation` comes with the default instantiation, where it represents a membership
/// proof for the first leaf (at index 0) in a tree over 8 bytes (`[0u8,..,7u8]`). The underlying
/// tree (together with its hash function parameters) is generated from the function
/// `default_tree()`.
#[derive(Clone)]
pub struct DepositRelation<Trapdoor, Nullifier, TokenId> {
    pub t: Trapdoor,
    pub n: Nullifier,

    pub note: NoteVar,
    pub token_id: TokenId,
    pub value: u128,

    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl<Trapdoor, Nullifier, TokenId> Default for DepositRelation<Trapdoor, Nullifier, TokenId> 
where
    Trapdoor: Default + ToBytesGadget<CircuitField>,
    Nullifier: Default + ToBytesGadget<CircuitField>,
    TokenId: Default +ToBytesGadget<CircuitField>, {
    fn default() -> Self {
        let mut rng = StdRng::from_seed([0u8; 32]);
        let t = Trapdoor::default();
        let n = Nullifier::default();
        let token_id = TokenId::default();
        let value = u128::default();
        let two_to_one_crh_params = <TwoToOneHash as TwoToOneCRH>::setup(&mut rng).unwrap();

        let mut parsed_n = n.to_bytes().expect("").as_slice();
        let mut parsed_t = t.to_bytes().expect("").as_slice();

        let mut parsed_token_id = token_id.to_bytes().expect("").as_slice();
        let mut parsed_value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&value)).expect("");

        let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, parsed_n, parsed_t).expect("");
        let right_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, parsed_token_id, parsed_value.to_bytes().expect("").as_slice()).expect("");
        let note = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, left_hash.to_bytes().expect("").as_slice(), right_hash.to_bytes().expect("").as_slice()).expect("");

        DepositRelation {
            t,
            n,
            note,
            token_id,
            value,
            two_to_one_crh_params,
        }
    }
}

impl<Trapdoor, Nullifier, TokenId> ConstraintSynthesizer<CircuitField> for DepositRelation<Trapdoor, Nullifier, TokenId> 
where
    Trapdoor: ToBytesGadget<CircuitField>,
    Nullifier: ToBytesGadget<CircuitField>,
    TokenId: ToBytesGadget<CircuitField>, {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs, &self.two_to_one_crh_params)?;

        let mut n = self.n.to_bytes()?.as_slice();
        let mut t = self.t.to_bytes()?.as_slice();

        let mut token_id = self.token_id.to_bytes()?.as_slice();
        let mut value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&self.value))?;

        let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, n, t)?;
        let right_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, token_id, value.to_bytes()?.as_slice())?;
        let final_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, left_hash.to_bytes()?.as_slice(), right_hash.to_bytes()?.as_slice())?;

        final_hash.enforce_equal(&self.note)?;

        Ok(())
    }
}

impl<Trapdoor, Nullifier, TokenId> GetPublicInput<CircuitField> for DepositRelation<Trapdoor, Nullifier, TokenId> {
}
