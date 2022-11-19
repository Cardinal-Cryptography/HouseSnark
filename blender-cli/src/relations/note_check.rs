use ark_r1cs_std::{eq::EqGadget, ToBytesGadget};
use ark_relations::r1cs::SynthesisError;

use crate::relations::{
    tangle::tangle_in_field,
    types::{ByteVar, FpVar},
};

pub(super) fn check_note(
    token_id: FpVar,
    token_amount: FpVar,
    trapdoor: FpVar,
    nullifier: FpVar,
    note: FpVar,
) -> Result<(), SynthesisError> {
    let mut bytes: Vec<ByteVar> = [
        token_id.to_bytes()?,
        token_amount.to_bytes()?,
        trapdoor.to_bytes()?,
        nullifier.to_bytes()?,
    ]
    .concat();
    let number_of_bytes = bytes.len();

    tangle_in_field(&mut bytes, 0, number_of_bytes)?;

    let note_bytes = note.to_bytes()?;

    for (a, b) in note_bytes.iter().zip(bytes.iter()) {
        a.enforce_equal(b)?;
    }
    Ok(())
}
