//! Module exposing some utilities regarding note generation and verification.

use ark_ff::{BigInteger, BigInteger256};
use ark_r1cs_std::{eq::EqGadget, ToBytesGadget};
use ark_relations::r1cs::SynthesisError;

use super::{
    tangle::{tangle, tangle_in_field},
    types::{
        ByteVar, FpVar, FrontendNote, FrontendNullifier, FrontendTokenAmount, FrontendTokenId,
        FrontendTrapdoor,
    },
};

/// Verify that `note` is indeed the result of tangling `(token_id, token_amount, trapdoor,
/// nullifier)`.
///
/// For circuit use only.
pub(super) fn check_note(
    token_id: &FpVar,
    token_amount: &FpVar,
    trapdoor: &FpVar,
    nullifier: &FpVar,
    note: &FpVar,
) -> Result<(), SynthesisError> {
    let bytes: Vec<ByteVar> = [
        token_id.to_bytes()?,
        token_amount.to_bytes()?,
        trapdoor.to_bytes()?,
        nullifier.to_bytes()?,
    ]
    .concat();
    let bytes = tangle_in_field::<4>(bytes)?;

    for (a, b) in note.to_bytes()?.iter().zip(bytes.iter()) {
        a.enforce_equal(b)?;
    }
    Ok(())
}

/// Compute note as the result of tangling `(token_id, token_amount, trapdoor, nullifier)`.
///
/// Useful for input preparation and offline note generation.
pub fn compute_note(
    token_id: FrontendTokenId,
    token_amount: FrontendTokenAmount,
    trapdoor: FrontendTrapdoor,
    nullifier: FrontendNullifier,
) -> FrontendNote {
    let bytes = [
        BigInteger256::from(token_id.0 as u64).to_bytes_le(),
        BigInteger256::from(token_amount.0).to_bytes_le(),
        BigInteger256::from(trapdoor.0).to_bytes_le(),
        BigInteger256::from(nullifier.0).to_bytes_le(),
    ]
    .concat();

    FrontendNote::from_bytes(tangle::<4>(bytes).as_slice())
}

pub fn compute_parent_hash(left: FrontendNote, right: FrontendNote) -> FrontendNote {
    let bytes = [
        BigInteger256::new(left.0).to_bytes_le(),
        BigInteger256::new(right.0).to_bytes_le(),
    ]
    .concat();
    FrontendNote::from_bytes(tangle::<2>(bytes).as_slice())
}
