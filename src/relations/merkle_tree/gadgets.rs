use ark_crypto_primitives::{
    crh::{
        injective_map::{
            constraints::{PedersenCRHCompressorGadget, TECompressorGadget},
            TECompressor,
        },
        TwoToOneCRHGadget,
    },
    CRHGadget,
};
use ark_ed_on_bls12_381::{constraints::EdwardsVar, EdwardsProjective};

use crate::relations::merkle_tree::hash_functions::{
    LeafHash, LeafWindow, TwoToOneHash, TwoToOneWindow,
};

pub type TwoToOneHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    TwoToOneWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashGadget = PedersenCRHCompressorGadget<
    EdwardsProjective,
    TECompressor,
    LeafWindow,
    EdwardsVar,
    TECompressorGadget,
>;

pub type LeafHashParamsVar<ConstraintF> =
    <LeafHashGadget as CRHGadget<LeafHash, ConstraintF>>::ParametersVar;
pub type TwoToOneHashParamsVar<ConstraintF> =
    <TwoToOneHashGadget as TwoToOneCRHGadget<TwoToOneHash, ConstraintF>>::ParametersVar;
