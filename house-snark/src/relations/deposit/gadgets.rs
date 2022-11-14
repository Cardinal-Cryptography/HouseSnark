use ark_crypto_primitives::{
    crh::{
        injective_map::{
            constraints::{PedersenCRHCompressorGadget, TECompressorGadget},
            TECompressor,
        },
        TwoToOneCRHGadget,
    },
};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective};

use crate::{
    relations::deposit::hash_functions::{TwoToOneHash, TwoToOneWindow},
    CircuitField,
};

pub type TwoToOneHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    TwoToOneWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type TwoToOneHashParamsVar =
    <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, CircuitField>>::ParametersVar;
