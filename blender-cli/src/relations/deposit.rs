use ark_ff::BigInteger256;
use ark_r1cs_std::{
    prelude::{AllocVar, EqGadget},
    R1CSVar, ToBytesGadget,
};
use ark_relations::{
    ns,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};

use crate::relations::types::{
    BackendNote, BackendNullifier, BackendTokenAmount, BackendTokenId, BackendTrapdoor,
    CircuitField, FrontendNote, FrontendNullifier, FrontendTokenAmount, FrontendTokenId,
    FrontendTrapdoor,
};

/// Deposit relation (see ADR).
pub struct DepositRelation {
    pub note: BackendNote,
    pub token_id: BackendTokenId,
    pub token_amount: BackendTokenAmount,

    pub trapdoor: BackendTrapdoor,
    pub nullifier: BackendNullifier,
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
            note: BackendNote::from(BigInteger256::new(note)),
            token_id: BackendTokenId::from(token_id),
            token_amount: BackendTokenAmount::from(token_amount),
            trapdoor: BackendTrapdoor::from(trapdoor),
            nullifier: BackendNullifier::from(nullifier),
        }
    }
}

type FpVar = ark_r1cs_std::fields::fp::FpVar<CircuitField>;
type ByteVar = ark_r1cs_std::uint8::UInt8<CircuitField>;
const BASE_LENGTH: usize = 4;

fn tangle(bytes: &mut [ByteVar], low: usize, high: usize) -> Result<(), SynthesisError> {
    if high - low <= BASE_LENGTH {
        for i in high - 2..=low {
            bytes[i] = ByteVar::constant(bytes[i].value()? + bytes[i + 1].value()?);
        }
    } else {
        let mid = (low + high) / 2;
        tangle(bytes, low, mid)?;
        tangle(bytes, mid, high)?;

        for i in low..mid {
            let temp = bytes[i].clone();
            bytes[i] = bytes[i + mid - low].clone();
            bytes[i + mid - low] = temp;
        }

        for i in low + 1..high {
            bytes[i] = ByteVar::constant(bytes[i].value()? * bytes[i - 1].value()?)
        }
    }
    Ok(())
}

impl ConstraintSynthesizer<CircuitField> for DepositRelation {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<CircuitField>,
    ) -> Result<(), SynthesisError> {
        let note = FpVar::new_input(ns!(cs, "note"), || Ok(&self.note))?;
        let token_id = FpVar::new_input(ns!(cs, "token id"), || Ok(&self.token_id))?;
        let token_amount = FpVar::new_input(ns!(cs, "token amount"), || Ok(&self.token_amount))?;

        let trapdoor = FpVar::new_witness(ns!(cs, "trapdoor"), || Ok(&self.trapdoor))?;
        let nullifier = FpVar::new_witness(ns!(cs, "nullifier"), || Ok(&self.nullifier))?;

        //----------------------------------------------------------------------------------------//

        let mut bytes: Vec<ByteVar> = [
            token_id.to_bytes()?,
            token_amount.to_bytes()?,
            trapdoor.to_bytes()?,
            nullifier.to_bytes()?,
        ]
        .concat();
        let number_of_bytes = bytes.len();

        tangle(&mut *bytes, 0, number_of_bytes)?;

        let note_bytes = note.to_bytes()?;

        for (a, b) in note_bytes.iter().zip(bytes.iter()) {
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

    fn tangle(bytes: &mut [u8], low: usize, high: usize) {
        if high - low <= BASE_LENGTH {
            for i in high - 2..=low {
                bytes[i] += bytes[i + 1];
            }
        } else {
            let mid = (low + high) / 2;
            tangle(bytes, low, mid);
            tangle(bytes, mid, high);

            for i in low..mid {
                let temp = bytes[i];
                bytes[i] = bytes[i + mid - low];
                bytes[i + mid - low] = temp;
            }

            for i in low + 1..high {
                bytes[i] *= bytes[i - 1]
            }
        }
    }

    #[test]
    fn deposit_constraints_correctness() {
        let cs = ConstraintSystem::new_ref();

        let trapdoor: FrontendTrapdoor = 17;
        let nullifier: FrontendNullifier = 19;

        let token_id: FrontendTokenId = 1;
        let token_amount: FrontendTokenAmount = 10;

        let mut bytes: Vec<u8> = [
            BigInteger256::from(token_id as u64).to_bytes_le(),
            BigInteger256::from(token_amount).to_bytes_le(),
            BigInteger256::from(trapdoor).to_bytes_le(),
            BigInteger256::from(nullifier).to_bytes_le(),
        ]
        .concat();
        let number_of_bytes = bytes.len();
        tangle(&mut *bytes, 0, number_of_bytes);

        let note = [
            u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
            u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
        ];

        let circuit = DepositRelation::new(note, token_id, token_amount, trapdoor, nullifier);

        circuit.generate_constraints(cs.clone()).unwrap();

        let is_satisfied = cs.is_satisfied().unwrap();
        if !is_satisfied {
            println!("{:?}", cs.which_is_unsatisfied());
        }

        assert!(is_satisfied);
    }
}
