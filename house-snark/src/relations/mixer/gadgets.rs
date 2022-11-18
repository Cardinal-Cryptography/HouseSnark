use ark_bls12_381::FrParameters;
use ark_crypto_primitives::crh::{poseidon, TwoToOneCRHGadget};
use ark_ff::Fp256;

use super::hash_functions::{DummyCRHGadget, PoseidonParams};
use crate::{relations::mixer::hash_functions::TwoToOneHash, CircuitField};

pub type TwoToOneHashGadget = poseidon::constraints::CRHGadget<Fp256<FrParameters>, PoseidonParams>;

pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::ParametersVar;

pub type LeafHashGadget = DummyCRHGadget<Fp256<FrParameters>, PoseidonParams>;
