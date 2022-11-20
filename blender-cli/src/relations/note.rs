//! Module exposing some utilities regarding note generation and verification.

use ark_ff::{BigInteger, BigInteger256};
use ark_r1cs_std::{eq::EqGadget, ToBytesGadget};
use ark_relations::r1cs::SynthesisError;

use crate::relations::{
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
    let mut bytes: Vec<ByteVar> = [
        token_id.to_bytes()?,
        token_amount.to_bytes()?,
        trapdoor.to_bytes()?,
        nullifier.to_bytes()?,
    ]
    .concat();

    tangle_in_field(&mut bytes)?;

    let note_bytes = note.to_bytes()?;

    for (a, b) in note_bytes.iter().zip(bytes.iter()) {
        a.enforce_equal(b)?;
    }
    Ok(())
}

/// Compute note as the result of tangling `(token_id, token_amount, trapdoor, nullifier)`.
///
/// To be precise, the result of tangling will be 128 bytes. Then, the note will be created from
/// the first 32 bytes. The rest will be just abandoned.
///
/// Useful for input preparation and offline note generation.
pub fn compute_note(
    token_id: FrontendTokenId,
    token_amount: FrontendTokenAmount,
    trapdoor: FrontendTrapdoor,
    nullifier: FrontendNullifier,
) -> FrontendNote {
    let mut bytes = [
        BigInteger256::from(token_id as u64).to_bytes_le(),
        BigInteger256::from(token_amount).to_bytes_le(),
        BigInteger256::from(trapdoor).to_bytes_le(),
        BigInteger256::from(nullifier).to_bytes_le(),
    ]
    .concat();

    tangle(&mut bytes);

    note_from_bytes(bytes.as_slice())
}

/// Create a note from the first 32 bytes of `bytes`.
pub(super) fn note_from_bytes(bytes: &[u8]) -> FrontendNote {
    [
        u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
        u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
        u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
        u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
    ]
}
