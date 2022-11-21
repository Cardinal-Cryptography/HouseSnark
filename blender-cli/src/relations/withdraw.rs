use std::ops::{Add, Div};

use ark_ff::{BigInteger, BigInteger256};
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::FieldVar, R1CSVar, ToBytesGadget};
use ark_relations::{
    ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};

use crate::relations::{
    note::check_note,
    tangle::tangle_in_field,
    types::{
        BackendAccount, BackendLeafIndex, BackendMerklePath, BackendMerkleRoot, BackendNote,
        BackendNullifier, BackendTokenAmount, BackendTokenId, BackendTrapdoor, ByteVar,
        CircuitField, FpVar, FrontendAccount, FrontendLeafIndex, FrontendMerklePath,
        FrontendMerkleRoot, FrontendNote, FrontendNullifier, FrontendTokenAmount, FrontendTokenId,
        FrontendTrapdoor,
    },
};

/// 'Withdraw' relation for the Blender application.
///
/// It expresses the facts that:
///  - `new_note` is a prefix of the result of tangling together `token_id`, `whole_token_amount`,
///    `old_trapdoor` and `old_nullifier`,
///  - `old_note` is a prefix of the result of tangling together `token_id`, `new_token_amount`,
///    `new_trapdoor` and `new_nullifier`,
///  - `new_token_amount + token_amount_out = whole_token_amount`
///  - `merkle_path` is a valid Merkle proof for `old_note` being present at `leaf_index` in some
///    Merkle tree with `merkle_root` hash in the root
/// It also includes two artificial inputs `fee` and `recipient` just to strengthen the application
/// security by treating them as public inputs (and thus integral part of the SNARK).
pub struct WithdrawRelation {
    // Public inputs.
    pub old_nullifier: BackendNullifier,
    pub merkle_root: BackendMerkleRoot,
    pub new_note: BackendNote,
    pub token_id: BackendTokenId,
    pub token_amount_out: BackendTokenAmount,
    pub fee: BackendTokenAmount,
    pub recipient: BackendAccount,

    // Private inputs.
    pub old_trapdoor: BackendTrapdoor,
    pub new_trapdoor: BackendTrapdoor,
    pub new_nullifier: BackendNullifier,
    pub merkle_path: BackendMerklePath,
    pub leaf_index: BackendLeafIndex,
    pub old_note: BackendNote,
    pub whole_token_amount: BackendTokenAmount,
    pub new_token_amount: BackendTokenAmount,
}

impl WithdrawRelation {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        old_nullifier: FrontendNullifier,
        merkle_root: FrontendMerkleRoot,
        new_note: FrontendNote,
        token_id: FrontendTokenId,
        token_amount_out: FrontendTokenAmount,
        old_trapdoor: FrontendTrapdoor,
        new_trapdoor: FrontendTrapdoor,
        new_nullifier: FrontendNullifier,
        merkle_path: FrontendMerklePath,
        leaf_index: FrontendLeafIndex,
        old_note: FrontendNote,
        whole_token_amount: FrontendTokenAmount,
        new_token_amount: FrontendTokenAmount,
        fee: FrontendTokenAmount,
        recipient: FrontendAccount,
    ) -> Self {
        Self {
            old_nullifier: BackendNullifier::from(old_nullifier),
            merkle_root: BackendMerkleRoot::from(BigInteger256::new(merkle_root)),
            new_note: BackendNote::from(BigInteger256::new(new_note)),
            token_id: BackendTokenId::from(token_id),
            token_amount_out: BackendTokenAmount::from(token_amount_out),
            old_trapdoor: BackendTrapdoor::from(old_trapdoor),
            new_trapdoor: BackendTrapdoor::from(new_trapdoor),
            new_nullifier: BackendNullifier::from(new_nullifier),
            merkle_path: merkle_path
                .iter()
                .map(|node| BackendNote::from(BigInteger256::new(*node)))
                .collect(),
            leaf_index: BackendLeafIndex::from(leaf_index),
            old_note: BackendNote::from(BigInteger256::new(old_note)),
            whole_token_amount: BackendTokenAmount::from(whole_token_amount),
            new_token_amount: BackendTokenAmount::from(new_token_amount),
            fee: BackendTokenAmount::from(fee),
            recipient: BackendAccount::from(BigInteger256::new(recipient.map(|x| x as u64))),
        }
    }
}

impl ConstraintSynthesizer<CircuitField> for WithdrawRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        //----------------------------------------------------------------------
        // Dummy constraint so that the `fee` and `recipient` are actually used.
        //----------------------------------------------------------------------
        let fee = FpVar::new_input(ns!(cs, "fee"), || Ok(&self.fee))?;
        let recipient = FpVar::new_input(ns!(cs, "recipient"), || Ok(&self.recipient))?;
        (fee.clone().add(&recipient)).enforce_equal(&recipient.add(&fee))?;

        //------------------------------
        // Check the old note arguments.
        //------------------------------
        let old_note = FpVar::new_witness(ns!(cs, "old note"), || Ok(&self.old_note))?;
        let token_id = FpVar::new_input(ns!(cs, "token id"), || Ok(&self.token_id))?;
        let whole_token_amount = FpVar::new_witness(ns!(cs, "whole token amount"), || {
            Ok(&self.whole_token_amount)
        })?;
        let old_trapdoor = FpVar::new_witness(ns!(cs, "old trapdoor"), || Ok(&self.old_trapdoor))?;
        let old_nullifier = FpVar::new_input(ns!(cs, "old nullifier"), || Ok(&self.old_nullifier))?;

        check_note(
            &token_id,
            &whole_token_amount,
            &old_trapdoor,
            &old_nullifier,
            &old_note,
        )?;

        //------------------------------
        // Check the new note arguments.
        //------------------------------
        let new_note = FpVar::new_input(ns!(cs, "new note"), || Ok(&self.new_note))?;
        let new_token_amount =
            FpVar::new_witness(ns!(cs, "new token amount"), || Ok(&self.new_token_amount))?;
        let new_trapdoor = FpVar::new_witness(ns!(cs, "new trapdoor"), || Ok(&self.new_trapdoor))?;
        let new_nullifier =
            FpVar::new_witness(ns!(cs, "new nullifier"), || Ok(&self.new_nullifier))?;

        check_note(
            &token_id,
            &new_token_amount,
            &new_trapdoor,
            &new_nullifier,
            &new_note,
        )?;

        //----------------------------------
        // Check the token values soundness.
        //----------------------------------
        let token_amount_out =
            FpVar::new_input(ns!(cs, "token amount out"), || Ok(&self.token_amount_out))?;
        // some range checks for overflows?
        let token_sum = token_amount_out.add(new_token_amount);
        token_sum.enforce_equal(&whole_token_amount)?;

        //------------------------
        // Check the merkle proof.
        //------------------------
        let merkle_root = FpVar::new_input(ns!(cs, "merkle root"), || Ok(&self.merkle_root))?;
        let mut leaf_index = FpVar::new_witness(ns!(cs, "leaf index"), || Ok(&self.leaf_index))?;

        let mut current_hash_bytes = old_note.to_bytes()?;
        for hash in self.merkle_path {
            let sibling = FpVar::new_witness(ns!(cs, "merkle path node"), || Ok(hash))?;
            let bytes: Vec<ByteVar> = if leaf_index.value()?.0.is_even() {
                [current_hash_bytes.clone(), sibling.to_bytes()?].concat()
            } else {
                [sibling.to_bytes()?, current_hash_bytes.clone()].concat()
            };

            current_hash_bytes = tangle_in_field::<2>(bytes)?;

            leaf_index = FpVar::constant(leaf_index.value()?.div(CircuitField::from(2)));
        }

        for (a, b) in merkle_root
            .to_bytes()?
            .iter()
            .zip(current_hash_bytes.iter())
        {
            a.enforce_equal(b)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ark_ff::BigInteger;
    use ark_relations::r1cs::ConstraintSystem;

    use super::*;
    use crate::relations::{
        note::{compute_note, note_from_bytes},
        tangle::tangle,
    };

    fn compute_parent_hash(left: FrontendNote, right: FrontendNote) -> FrontendNote {
        let bytes = [
            BigInteger256::new(left).to_bytes_le(),
            BigInteger256::new(right).to_bytes_le(),
        ]
        .concat();
        note_from_bytes(tangle::<2>(bytes).as_slice())
    }

    #[test]
    fn withdraw_constraints_correctness() {
        let token_id: FrontendTokenId = 1;

        let old_trapdoor: FrontendTrapdoor = 17;
        let old_nullifier: FrontendNullifier = 19;
        let whole_token_amount: FrontendTokenAmount = 10;

        let new_trapdoor: FrontendTrapdoor = 27;
        let new_nullifier: FrontendNullifier = 87;
        let new_token_amount: FrontendTokenAmount = 3;

        let token_amount_out: FrontendTokenAmount = 7;

        let old_note = compute_note(token_id, whole_token_amount, old_trapdoor, old_nullifier);
        let new_note = compute_note(token_id, new_token_amount, new_trapdoor, new_nullifier);

        // Our leaf has a left bro. Their parent has a right bro. Our grandpa is the root.
        let leaf_index = 5;

        let sibling_note = compute_note(0, 1, 2, 3);
        let parent_note = compute_parent_hash(sibling_note, old_note);
        let uncle_note = compute_note(4, 5, 6, 7);
        let merkle_root = compute_parent_hash(parent_note, uncle_note);

        let merkle_path = vec![sibling_note, uncle_note];

        let fee: FrontendTokenAmount = 1;
        let recipient: FrontendAccount = [0u32; 4];

        let circuit = WithdrawRelation::new(
            old_nullifier,
            merkle_root,
            new_note,
            token_id,
            token_amount_out,
            old_trapdoor,
            new_trapdoor,
            new_nullifier,
            merkle_path,
            leaf_index,
            old_note,
            whole_token_amount,
            new_token_amount,
            fee,
            recipient,
        );

        let cs = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs.clone()).unwrap();

        let is_satisfied = cs.is_satisfied().unwrap();
        if !is_satisfied {
            println!("{:?}", cs.which_is_unsatisfied());
        }

        assert!(is_satisfied);
    }
}
