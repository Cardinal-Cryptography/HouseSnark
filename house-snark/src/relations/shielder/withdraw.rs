use std::ops::{Add, Div};

use ark_ff::BigInteger;
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::FieldVar, R1CSVar, ToBytesGadget};
use ark_relations::{
    ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use clap::Args;

use super::{
    note::check_note,
    parsing::{
        parse_frontend_account, parse_frontend_merkle_path_single, parse_frontend_merkle_root,
        parse_frontend_note,
    },
    tangle::tangle_in_field,
    types::{
        BackendAccount, BackendLeafIndex, BackendMerklePath, BackendMerkleRoot, BackendNote,
        BackendNullifier, BackendTokenAmount, BackendTokenId, BackendTrapdoor, ByteVar, FpVar,
        FrontendAccount, FrontendLeafIndex, FrontendMerklePath, FrontendMerkleRoot, FrontendNote,
        FrontendNullifier, FrontendTokenAmount, FrontendTokenId, FrontendTrapdoor,
    },
    CircuitField,
};
use crate::relations::GetPublicInput;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct WithdrawRelationArgs {
    // Public inputs.
    #[clap(long)]
    pub old_nullifier: FrontendNullifier,
    #[clap(long, value_parser = parse_frontend_merkle_root)]
    pub merkle_root: FrontendMerkleRoot,
    #[clap(long, value_parser = parse_frontend_note)]
    pub new_note: FrontendNote,
    #[clap(long)]
    pub token_id: FrontendTokenId,
    #[clap(long)]
    pub token_amount_out: FrontendTokenAmount,
    #[clap(long)]
    pub fee: FrontendTokenAmount,
    #[clap(long, value_parser = parse_frontend_account)]
    pub recipient: FrontendAccount,

    // Private inputs.
    #[clap(long)]
    pub old_trapdoor: FrontendTrapdoor,
    #[clap(long)]
    pub new_trapdoor: FrontendTrapdoor,
    #[clap(long)]
    pub new_nullifier: FrontendNullifier,
    #[clap(long, value_delimiter = ',', value_parser = parse_frontend_merkle_path_single)]
    pub merkle_path: FrontendMerklePath,
    #[clap(long)]
    pub leaf_index: FrontendLeafIndex,
    #[clap(long, value_parser = parse_frontend_note)]
    pub old_note: FrontendNote,
    #[clap(long)]
    pub whole_token_amount: FrontendTokenAmount,
    #[clap(long)]
    pub new_token_amount: FrontendTokenAmount,
}

/// 'Withdraw' relation for the Shielder application.
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
///
/// When providing a public input to proof verification, you should keep the order of variable
/// declarations in circuit, i.e.: `fee`, `recipient`, `token_id`, `old_nullifier`, `new_note`,
/// `token_amount_out`, `merkle_root`.
#[derive(Clone)]
pub struct WithdrawRelation {
    // Public inputs.
    fee: BackendTokenAmount,
    recipient: BackendAccount,
    token_id: BackendTokenId,
    old_nullifier: BackendNullifier,
    new_note: BackendNote,
    token_amount_out: BackendTokenAmount,
    merkle_root: BackendMerkleRoot,

    // Private inputs.
    old_trapdoor: BackendTrapdoor,
    new_trapdoor: BackendTrapdoor,
    new_nullifier: BackendNullifier,
    merkle_path: BackendMerklePath,
    leaf_index: BackendLeafIndex,
    old_note: BackendNote,
    whole_token_amount: BackendTokenAmount,
    new_token_amount: BackendTokenAmount,
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
            old_nullifier: old_nullifier.into(),
            merkle_root: merkle_root.into(),
            new_note: new_note.into(),
            token_id: token_id.into(),
            token_amount_out: token_amount_out.into(),
            old_trapdoor: old_trapdoor.into(),
            new_trapdoor: new_trapdoor.into(),
            new_nullifier: new_nullifier.into(),
            merkle_path: merkle_path.into(),
            leaf_index: leaf_index.into(),
            old_note: old_note.into(),
            whole_token_amount: whole_token_amount.into(),
            new_token_amount: new_token_amount.into(),
            fee: fee.into(),
            recipient: recipient.into(),
        }
    }
}

impl From<WithdrawRelationArgs> for WithdrawRelation {
    fn from(args: WithdrawRelationArgs) -> Self {
        let WithdrawRelationArgs {
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
        } = args;
        WithdrawRelation::new(
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
        )
    }
}

impl ConstraintSynthesizer<CircuitField> for WithdrawRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        //-----------------------------------------------
        // Baking `fee` and `recipient` into the circuit.
        //-----------------------------------------------
        let _fee = FpVar::new_input(ns!(cs, "fee"), || Ok(&self.fee.0))?;
        let _recipient = FpVar::new_input(ns!(cs, "recipient"), || Ok(&self.recipient.0))?;

        //------------------------------
        // Check the old note arguments.
        //------------------------------
        let old_note = FpVar::new_witness(ns!(cs, "old note"), || Ok(&self.old_note.0))?;
        let token_id = FpVar::new_input(ns!(cs, "token id"), || Ok(&self.token_id.0))?;
        let whole_token_amount = FpVar::new_witness(ns!(cs, "whole token amount"), || {
            Ok(&self.whole_token_amount.0)
        })?;
        let old_trapdoor =
            FpVar::new_witness(ns!(cs, "old trapdoor"), || Ok(&self.old_trapdoor.0))?;
        let old_nullifier =
            FpVar::new_input(ns!(cs, "old nullifier"), || Ok(&self.old_nullifier.0))?;

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
        let new_note = FpVar::new_input(ns!(cs, "new note"), || Ok(&self.new_note.0))?;
        let new_token_amount =
            FpVar::new_witness(ns!(cs, "new token amount"), || Ok(&self.new_token_amount.0))?;
        let new_trapdoor =
            FpVar::new_witness(ns!(cs, "new trapdoor"), || Ok(&self.new_trapdoor.0))?;
        let new_nullifier =
            FpVar::new_witness(ns!(cs, "new nullifier"), || Ok(&self.new_nullifier.0))?;

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
            FpVar::new_input(ns!(cs, "token amount out"), || Ok(&self.token_amount_out.0))?;
        // some range checks for overflows?
        let token_sum = token_amount_out.add(new_token_amount);
        token_sum.enforce_equal(&whole_token_amount)?;

        //------------------------
        // Check the merkle proof.
        //------------------------
        let merkle_root = FpVar::new_input(ns!(cs, "merkle root"), || Ok(&self.merkle_root.0))?;
        let mut leaf_index = FpVar::new_witness(ns!(cs, "leaf index"), || Ok(&self.leaf_index.0))?;

        let mut current_hash_bytes = old_note.to_bytes()?;
        for hash in self.merkle_path.0 {
            let sibling = FpVar::new_witness(ns!(cs, "merkle path node"), || Ok(hash))?;
            let bytes: Vec<ByteVar> = if leaf_index.value().unwrap_or_default().0.is_even() {
                [current_hash_bytes.clone(), sibling.to_bytes()?].concat()
            } else {
                [sibling.to_bytes()?, current_hash_bytes.clone()].concat()
            };

            current_hash_bytes = tangle_in_field::<2>(bytes)?;

            leaf_index = FpVar::constant(
                leaf_index
                    .value()
                    .unwrap_or_default()
                    .div(CircuitField::from(2)),
            );
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

impl GetPublicInput<CircuitField> for WithdrawRelation {
    fn public_input(&self) -> Vec<CircuitField> {
        [
            self.fee.0,
            self.recipient.0,
            self.token_id.0,
            self.old_nullifier.0,
            self.new_note.0,
            self.token_amount_out.0,
            self.merkle_root.0,
        ]
        .to_vec()
    }
}

#[cfg(test)]
mod tests {
    use ark_bls12_381::Bls12_381;
    use ark_ff::BigInteger256;
    use ark_groth16::Groth16;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_snark::SNARK;

    use super::*;
    use crate::relations::shielder::note::{compute_note, compute_parent_hash};

    fn get_circuit_and_input() -> (WithdrawRelation, [CircuitField; 7]) {
        let token_id = FrontendTokenId(1);

        let old_trapdoor = FrontendTrapdoor(17);
        let old_nullifier = FrontendNullifier(19);
        let whole_token_amount = FrontendTokenAmount(10);

        let new_trapdoor = FrontendTrapdoor(27);
        let new_nullifier = FrontendNullifier(87);
        let new_token_amount = FrontendTokenAmount(3);

        let token_amount_out = FrontendTokenAmount(7);

        let old_note = compute_note(token_id, whole_token_amount, old_trapdoor, old_nullifier);
        let new_note = compute_note(token_id, new_token_amount, new_trapdoor, new_nullifier);

        // Our leaf has a left bro. Their parent has a right bro. Our grandpa is the root.
        let leaf_index = FrontendLeafIndex(5);

        let sibling_note = compute_note(
            FrontendTokenId(0),
            FrontendTokenAmount(1),
            FrontendTrapdoor(2),
            FrontendNullifier(3),
        );
        let parent_note = compute_parent_hash(sibling_note, old_note);
        let uncle_note = compute_note(
            FrontendTokenId(4),
            FrontendTokenAmount(5),
            FrontendTrapdoor(6),
            FrontendNullifier(7),
        );
        let merkle_root_as_note = compute_parent_hash(parent_note, uncle_note);

        let merkle_path = FrontendMerklePath(vec![sibling_note.0, uncle_note.0]);

        let fee = FrontendTokenAmount(1);
        let recipient = FrontendAccount([
            212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133,
            88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
        ]);

        let circuit = WithdrawRelation::new(
            old_nullifier,
            FrontendMerkleRoot(merkle_root_as_note.0),
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

        let input = circuit.public_input();

        (circuit, input.try_into().unwrap())
    }

    #[test]
    fn withdraw_constraints_correctness() {
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
    fn withdraw_proving_procedure() {
        let (circuit, input) = get_circuit_and_input();

        let mut rng = ark_std::test_rng();
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

        let proof = Groth16::prove(&pk, circuit, &mut rng).unwrap();
        let valid_proof = Groth16::verify(&vk, &input, &proof).unwrap();
        assert!(valid_proof);
    }

    #[test]
    fn neither_fee_nor_recipient_are_simplified_out() {
        let (circuit, true_input) = get_circuit_and_input();

        let mut rng = ark_std::test_rng();
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
        let proof = Groth16::prove(&pk, circuit, &mut rng).unwrap();

        let mut input_with_corrupted_fee = true_input.clone();
        input_with_corrupted_fee[0] = BackendTokenAmount::from(FrontendTokenAmount(2)).0;
        assert_ne!(true_input[0], input_with_corrupted_fee[0]);

        let valid_proof = Groth16::verify(&vk, &input_with_corrupted_fee, &proof).unwrap();
        assert!(!valid_proof);

        let mut input_with_corrupted_recipient = true_input.clone();
        let fake_recipient = [41; 32];
        // todo: implement casting between backend and frontend types
        input_with_corrupted_recipient[1] = CircuitField::new(BigInteger256::new([
            u64::from_le_bytes(fake_recipient[0..8].try_into().expect("0-8")),
            u64::from_le_bytes(fake_recipient[8..16].try_into().expect("8-16")),
            u64::from_le_bytes(fake_recipient[16..24].try_into().expect("16-24")),
            u64::from_le_bytes(fake_recipient[24..32].try_into().expect("24-32")),
        ]));
        assert_ne!(true_input[1], input_with_corrupted_recipient[1]);

        let valid_proof = Groth16::verify(&vk, &input_with_corrupted_recipient, &proof).unwrap();
        assert!(!valid_proof);
    }
}
