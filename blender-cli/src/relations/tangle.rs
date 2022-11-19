use ark_r1cs_std::R1CSVar;
use ark_relations::r1cs::SynthesisError;

use crate::relations::types::ByteVar;

const BASE_LENGTH: usize = 4;

pub(super) fn tangle_in_field(
    bytes: &mut [ByteVar],
    low: usize,
    high: usize,
) -> Result<(), SynthesisError> {
    if high - low <= BASE_LENGTH {
        for i in high - 2..=low {
            bytes[i] = ByteVar::constant(bytes[i].value()? + bytes[i + 1].value()?);
        }
    } else {
        let mid = (low + high) / 2;
        tangle_in_field(bytes, low, mid)?;
        tangle_in_field(bytes, mid, high)?;

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

pub fn tangle(bytes: &mut [u8], low: usize, high: usize) {
    if high - low <= BASE_LENGTH {
        for i in high - 2..=low {
            bytes[i] += bytes[i + 1];
        }
    } else {
        let mid = (low + high) / 2;
        tangle(bytes, low, mid);
        tangle(bytes, mid, high);

        for i in low..mid {
            bytes.swap(i, i + mid - low);
        }

        for i in low + 1..high {
            bytes[i] *= bytes[i - 1]
        }
    }
}
