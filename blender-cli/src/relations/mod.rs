//! This module contains two relations that are the core of the Blender application: `deposit` and
//! `withdraw`. It also exposes some functions and types that might be useful for input generation.
//!
//! Currently, instead of using some real hash function, we chose to incorporate a simple tangling
//! algorithm. Essentially, it is a procedure that just mangles a byte array in-place. Usually, the
//! input to tangling will be just 128 bytes, but the desired output of the whole step would be a
//! single field element, which is just 32 bytes. Hence, we will save only first quarter of the
//! tangling result and simply abandoning the suffix.

#[allow(dead_code)]
mod deposit;
#[allow(dead_code)]
mod note;
#[allow(dead_code)]
mod tangle;
#[allow(dead_code)]
mod types;
#[allow(dead_code)]
mod withdraw;

pub use types::{
    FrontendNote as Note, FrontendNullifier as Nullifier, FrontendTokenAmount as TokenAmount,
    FrontendTokenId as TokenId, FrontendTrapdoor as Trapdoor,
};
