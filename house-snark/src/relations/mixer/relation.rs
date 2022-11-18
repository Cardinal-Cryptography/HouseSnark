use ark_bls12_381::FrParameters;
use ark_crypto_primitives::{
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
    merkle_tree::Config,
    Path, PathVar,
};
use ark_ff::{Fp256, FromBytes};
use ark_r1cs_std::{
    fields::fp::FpVar,
    prelude::{AllocVar, Boolean, EqGadget},
    uint128::UInt128,
    ToBytesGadget,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use super::{gadgets::LeafHashGadget, hash_functions::LeafHash};
use crate::{
    relations::mixer::{
        gadgets::{TwoToOneHashGadget, TwoToOneHashParamsVar},
        hash_functions::TwoToOneHash,
    },
    CircuitField, GetPublicInput,
};

pub type Note = [u8; 32];

#[derive(Clone)]
pub struct DepositRelation {
    pub t: u128,
    pub n: u128,

    pub note: Note,
    pub token_id: u128,
    pub value: u128,

    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl ConstraintSynthesizer<CircuitField> for DepositRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        let n = UInt128::new_input(ark_relations::ns!(cs, "n_var"), || Ok(&self.n))?;
        let t = UInt128::new_input(ark_relations::ns!(cs, "t_var"), || Ok(&self.t))?;

        let token_id =
            UInt128::new_input(
                ark_relations::ns!(cs, "token_id_var"),
                || Ok(&self.token_id),
            )?;
        let value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&self.value))?;

        let left_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            n.to_bytes()?.as_slice(),
            t.to_bytes()?.as_slice(),
        )?;
        let right_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            token_id.to_bytes()?.as_slice(),
            value.to_bytes()?.as_slice(),
        )?;
        let final_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            left_hash.to_bytes()?.as_slice(),
            right_hash.to_bytes()?.as_slice(),
        )?;

        let note =
            FpVar::<Fp256<FrParameters>>::new_input(ark_relations::ns!(cs, "note_var"), || {
                <Fp256<FrParameters> as FromBytes>::read(self.note.as_slice())
                    .map_err(|_| SynthesisError::UnexpectedIdentity)
            })?;

        final_hash.enforce_equal(&note)?;

        Ok(())
    }
}

impl GetPublicInput<CircuitField> for DepositRelation {}

/// The R1CS equivalent of the the Merkle tree root.
pub type RootVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::OutputVar;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct MerkleConfig;
impl Config for MerkleConfig {
    type LeafHash = LeafHash;
    type TwoToOneHash = TwoToOneHash;
}

/// The root of the byte Merkle tree.
pub type Root = <TwoToOneHash as TwoToOneCRH>::Output;

#[derive(Clone)]
pub struct MerklePathWrapper {
    path: Path<MerkleConfig>,
}

#[derive(Clone)]
pub struct WithdrawRelation {
    pub old_t: u128,
    pub new_t: u128,
    pub new_value: u128,
    pub merkle_proof: MerklePathWrapper,
    pub old_n: u128,
    pub old_note: Note,

    pub token_id: u128,
    pub value: u128,
    pub value_out: u128,
    pub merkle_root: Root,
    pub new_n: u128,
    pub new_note: Note,
    pub _recipient: u128,
    pub _fee: u128,

    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
    pub leaf_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl WithdrawRelation {
    fn verify_old_inputs(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        let old_n = UInt128::new_input(ark_relations::ns!(cs, "old_n_var"), || Ok(&self.old_n))?;
        let old_t = UInt128::new_input(ark_relations::ns!(cs, "old_t_var"), || Ok(&self.old_t))?;

        let token_id =
            UInt128::new_input(
                ark_relations::ns!(cs, "token_id_var"),
                || Ok(&self.token_id),
            )?;
        let value_out = UInt128::new_input(ark_relations::ns!(cs, "value_out_var"), || {
            Ok(&self.value_out)
        })?;

        let left_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            old_n.to_bytes()?.as_slice(),
            old_t.to_bytes()?.as_slice(),
        )?;
        let right_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            token_id.to_bytes()?.as_slice(),
            value_out.to_bytes()?.as_slice(),
        )?;
        let final_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            left_hash.to_bytes()?.as_slice(),
            right_hash.to_bytes()?.as_slice(),
        )?;

        let old_note = FpVar::<Fp256<FrParameters>>::new_input(
            ark_relations::ns!(cs, "old_note_var"),
            || {
                <Fp256<FrParameters> as FromBytes>::read(self.old_note.as_slice())
                    .map_err(|_| SynthesisError::UnexpectedIdentity)
            },
        )?;

        final_hash.enforce_equal(&old_note)?;

        Ok(())
    }

    fn verify_new_inputs(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        let new_n = UInt128::new_input(ark_relations::ns!(cs, "new_n_var"), || Ok(&self.new_n))?;
        let new_t = UInt128::new_input(ark_relations::ns!(cs, "new_t_var"), || Ok(&self.new_t))?;

        let token_id =
            UInt128::new_input(
                ark_relations::ns!(cs, "token_id_var"),
                || Ok(&self.token_id),
            )?;
        let new_value = UInt128::new_input(ark_relations::ns!(cs, "new_value_var"), || {
            Ok(&self.new_value)
        })?;
        let value_out = UInt128::new_input(ark_relations::ns!(cs, "value_out_var"), || {
            Ok(&self.value_out)
        })?;

        let value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&self.value))?;
        let sum = UInt128::addmany(&[new_value.clone(), value_out])?;
        sum.enforce_equal(&value)?;

        let left_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            new_n.to_bytes()?.as_slice(),
            new_t.to_bytes()?.as_slice(),
        )?;
        let right_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            token_id.to_bytes()?.as_slice(),
            new_value.to_bytes()?.as_slice(),
        )?;
        let final_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            left_hash.to_bytes()?.as_slice(),
            right_hash.to_bytes()?.as_slice(),
        )?;

        let new_note = FpVar::<Fp256<FrParameters>>::new_input(
            ark_relations::ns!(cs, "new_note_var"),
            || {
                <Fp256<FrParameters> as FromBytes>::read(self.new_note.as_slice())
                    .map_err(|_| SynthesisError::UnexpectedIdentity)
            },
        )?;

        final_hash.enforce_equal(&new_note)?;

        Ok(())
    }

    fn verify_merkle_proof(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let root =
            RootVar::new_input(ark_relations::ns!(cs, "root_var"), || Ok(&self.merkle_root))?;
        let path: PathVar<_, LeafHashGadget, TwoToOneHashGadget, _> =
            PathVar::new_input(ark_relations::ns!(cs, "merkle_proof_var"), || {
                Ok(&self.merkle_proof.path)
            })?;
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;
        let leaf_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.leaf_crh_params)?;

        let old_note = FpVar::<Fp256<FrParameters>>::new_input(
            ark_relations::ns!(cs, "old_note_var"),
            || {
                <Fp256<FrParameters> as FromBytes>::read(self.old_note.as_slice())
                    .map_err(|_| SynthesisError::UnexpectedIdentity)
            },
        )?;

        path.verify_membership(&leaf_crh_params, &two_to_one_crh_params, &root, &old_note)?
            .enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}

impl ConstraintSynthesizer<CircuitField> for WithdrawRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        self.clone().verify_merkle_proof(cs.clone())?;
        self.clone().verify_old_inputs(cs.clone())?;
        self.verify_new_inputs(cs)?;

        Ok(())
    }
}

impl GetPublicInput<CircuitField> for WithdrawRelation {}
