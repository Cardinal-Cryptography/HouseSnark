//! This module provides 'tangling' - some cheap substitute for real hash function.
//!
//! Tangling is a function that takes in a sequence of bytes (either raw bytes (`tangle`) or as
//! field bytes gadgets (`tangle_in_field`)) and manipulates it in place. It operates as follows:
//!  1) For every chunk of length `BASE_LENGTH` it computes suffix sums.
//!  2) We build a binary tree over these chunks.
//!  3) We go bottom-to-top and in every intermediate node we:
//!     a) swap the halves
//!     b) compute prefix products
//!
//! Note, it is **not** hiding like a hashing function.

use ark_r1cs_std::R1CSVar;
use ark_relations::r1cs::SynthesisError;

use crate::relations::types::ByteVar;

const BASE_LENGTH: usize = 4;

pub(super) fn tangle_in_field(bytes: &mut [ByteVar]) -> Result<(), SynthesisError> {
    let number_of_bytes = bytes.len();
    _tangle_in_field(bytes, 0, number_of_bytes)
}

fn _tangle_in_field(bytes: &mut [ByteVar], low: usize, high: usize) -> Result<(), SynthesisError> {
    if high - low <= BASE_LENGTH {
        let mut i = high - 2;
        while i >= low && i < high - 1 {
            bytes[i] = ByteVar::constant(bytes[i].value()? + bytes[i + 1].value()?);
            i -= 1;
        }
    } else {
        let mid = (low + high) / 2;
        _tangle_in_field(bytes, low, mid)?;
        _tangle_in_field(bytes, mid, high)?;

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

pub fn tangle(bytes: &mut [u8]) {
    let number_of_bytes = bytes.len();
    _tangle(bytes, 0, number_of_bytes)
}

fn _tangle(bytes: &mut [u8], low: usize, high: usize) {
    if high - low <= BASE_LENGTH {
        let mut i = high - 2;
        while i >= low && i < high - 1 {
            bytes[i] += bytes[i + 1];
            i -= 1;
        }
    } else {
        let mid = (low + high) / 2;
        _tangle(bytes, low, mid);
        _tangle(bytes, mid, high);

        for i in low..mid {
            bytes.swap(i, i + mid - low);
        }

        for i in low + 1..high {
            bytes[i] *= bytes[i - 1]
        }
    }
}
