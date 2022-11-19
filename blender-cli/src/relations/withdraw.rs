use ark_ff::BigInteger256;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use crate::relations::types::{
    BackendLeafIndex, BackendMerklePath, BackendMerkleRoot, BackendNote, BackendNullifier,
    BackendTokenAmount, BackendTokenId, BackendTrapdoor, CircuitField, FrontendLeafIndex,
    FrontendMerklePath, FrontendMerkleRoot, FrontendNote, FrontendNullifier, FrontendTokenAmount,
    FrontendTokenId, FrontendTrapdoor,
};

/// Withdraw relation (see ADR).
pub struct WithdrawRelation {
    pub old_nullifier: BackendNullifier,
    pub merkle_root: BackendMerkleRoot,
    pub new_note: BackendNote,
    pub token_id: BackendTokenId,
    pub token_amount_out: BackendTokenAmount,

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
        }
    }
}

impl ConstraintSynthesizer<CircuitField> for WithdrawRelation {
    fn generate_constraints(
        self,
        _cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        Ok(())
    }
}
