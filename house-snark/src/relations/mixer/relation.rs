use ark_crypto_primitives::{
    crh::{TwoToOneCRH, TwoToOneCRHGadget},
    merkle_tree::Config,
    Path, PathVar,
};
use ark_r1cs_std::{
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

pub type NoteVar = <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::OutputVar;

#[derive(Clone)]
pub struct DepositRelation<Trapdoor, Nullifier, TokenId> {
    pub t: Trapdoor,
    pub n: Nullifier,

    pub note: NoteVar,
    pub token_id: TokenId,
    pub value: u128,

    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl<Trapdoor, Nullifier, TokenId> ConstraintSynthesizer<CircuitField>
    for DepositRelation<Trapdoor, Nullifier, TokenId>
where
    Trapdoor: ToBytesGadget<CircuitField>,
    Nullifier: ToBytesGadget<CircuitField>,
    TokenId: ToBytesGadget<CircuitField>,
{
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        let n = self.n.to_bytes()?;
        let n = n.as_slice();
        let t = self.t.to_bytes()?;
        let t = t.as_slice();

        let token_id = self.token_id.to_bytes()?;
        let token_id = token_id.as_slice();
        let value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&self.value))?;

        let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, n, t)?;
        let right_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            token_id,
            value.to_bytes()?.as_slice(),
        )?;
        let final_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            left_hash.to_bytes()?.as_slice(),
            right_hash.to_bytes()?.as_slice(),
        )?;

        final_hash.enforce_equal(&self.note)?;

        Ok(())
    }
}

impl<Trapdoor, Nullifier, TokenId> GetPublicInput<CircuitField>
    for DepositRelation<Trapdoor, Nullifier, TokenId>
{
}

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
pub struct WithdrawRelation<Trapdoor: Clone, Nullifier: Clone, TokenId: Clone> {
    pub old_t: Trapdoor,
    pub new_t: Trapdoor,
    pub new_value: u128,
    pub merkle_proof: MerklePathWrapper,
    pub old_n: Nullifier,
    pub old_note: NoteVar,

    pub token_id: TokenId,
    pub value: u128,
    pub value_out: u128,
    pub merkle_root: Root,
    pub new_n: Nullifier,
    pub new_note: NoteVar,
    pub _recipient: u128,
    pub _fee: u128,

    pub two_to_one_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
    pub leaf_crh_params: <TwoToOneHash as TwoToOneCRH>::Parameters,
}

impl<Trapdoor, Nullifier, TokenId> WithdrawRelation<Trapdoor, Nullifier, TokenId>
where
    Trapdoor: ToBytesGadget<CircuitField> + Clone,
    Nullifier: ToBytesGadget<CircuitField> + Clone,
    TokenId: ToBytesGadget<CircuitField> + Clone,
{
    fn verify_old_inputs(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        let old_n = self.old_n.to_bytes()?;
        let old_n = old_n.as_slice();
        let old_t = self.old_t.to_bytes()?;
        let old_t = old_t.as_slice();

        let token_id = self.token_id.to_bytes()?;
        let token_id = token_id.as_slice();
        let value_out = UInt128::new_input(ark_relations::ns!(cs, "value_out_var"), || {
            Ok(&self.value_out)
        })?;

        let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, old_n, old_t)?;
        let right_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            token_id,
            value_out.to_bytes()?.as_slice(),
        )?;
        let final_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            left_hash.to_bytes()?.as_slice(),
            right_hash.to_bytes()?.as_slice(),
        )?;

        final_hash.enforce_equal(&self.old_note)?;

        Ok(())
    }

    fn verify_new_inputs(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let two_to_one_crh_params =
            TwoToOneHashParamsVar::new_constant(cs.clone(), &self.two_to_one_crh_params)?;

        let new_n = self.new_n.to_bytes()?;
        let new_n = new_n.as_slice();
        let new_t = self.new_t.to_bytes()?;
        let new_t = new_t.as_slice();

        let token_id = self.token_id.to_bytes()?;
        let token_id = token_id.as_slice();
        let new_value = UInt128::new_input(ark_relations::ns!(cs, "new_value_var"), || {
            Ok(&self.new_value)
        })?;
        let value_out = UInt128::new_input(ark_relations::ns!(cs, "value_out_var"), || {
            Ok(&self.value_out)
        })?;

        let value = UInt128::new_input(ark_relations::ns!(cs, "value_var"), || Ok(&self.value))?;
        let sum = UInt128::addmany(&[new_value.clone(), value_out])?;
        sum.enforce_equal(&value)?;

        let left_hash = TwoToOneHashGadget::evaluate(&two_to_one_crh_params, new_n, new_t)?;
        let right_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            token_id,
            new_value.to_bytes()?.as_slice(),
        )?;
        let final_hash = TwoToOneHashGadget::evaluate(
            &two_to_one_crh_params,
            left_hash.to_bytes()?.as_slice(),
            right_hash.to_bytes()?.as_slice(),
        )?;

        final_hash.enforce_equal(&self.old_note)?;

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
        let leaf_crh_params = TwoToOneHashParamsVar::new_constant(cs, &self.leaf_crh_params)?;

        path.verify_membership(
            &leaf_crh_params,
            &two_to_one_crh_params,
            &root,
            &self.old_note,
        )?
        .enforce_equal(&Boolean::TRUE)?;

        Ok(())
    }
}

impl<Trapdoor, Nullifier, TokenId> ConstraintSynthesizer<CircuitField>
    for WithdrawRelation<Trapdoor, Nullifier, TokenId>
where
    Trapdoor: ToBytesGadget<CircuitField> + Clone,
    Nullifier: ToBytesGadget<CircuitField> + Clone,
    TokenId: ToBytesGadget<CircuitField> + Clone,
{
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

impl<Trapdoor: Clone, Nullifier: Clone, TokenId: Clone> GetPublicInput<CircuitField>
    for WithdrawRelation<Trapdoor, Nullifier, TokenId>
{
}
