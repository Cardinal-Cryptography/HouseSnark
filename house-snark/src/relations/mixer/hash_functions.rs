use ark_bls12_381::FrParameters;
use ark_crypto_primitives::{
    crh::poseidon::{self, sbox::PoseidonSbox, PoseidonRoundParams},
    CRHGadget, Error, CRH,
};
use ark_ff::{Fp256, FromBytes, PrimeField};
use ark_std::rand::Rng;

/// Way of calculating hash in parent from child nodes.
pub type TwoToOneHash = poseidon::CRH<Fp256<FrParameters>, PoseidonParams>;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PoseidonParams;

impl<F: PrimeField> PoseidonRoundParams<F> for PoseidonParams {
    const WIDTH: usize = 4;
    const FULL_ROUNDS_BEGINNING: usize = 4;
    const FULL_ROUNDS_END: usize = 4;
    const PARTIAL_ROUNDS: usize = 4;
    const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(2);
}

/// Way of calculating hash in leaf node (from actual data).
pub type LeafHash = DummyCRHCompressor<Fp256<FrParameters>, PoseidonParams>;

pub struct DummyCRHCompressor<F, P>
where
    F: PrimeField,
    P: PoseidonRoundParams<F>,
{
    _crh: poseidon::CRH<F, P>,
}

impl<'a, F, P> CRH for DummyCRHCompressor<F, P>
where
    F: PrimeField + FromBytes,
    P: PoseidonRoundParams<F>,
{
    const INPUT_SIZE_BITS: usize = 32;
    type Output = F;
    type Parameters = <poseidon::CRH<F, P> as CRH>::Parameters;

    fn setup<R: Rng>(rng: &mut R) -> Result<Self::Parameters, Error> {
        poseidon::CRH::setup(rng)
    }

    fn evaluate(_parameters: &Self::Parameters, input: &[u8]) -> Result<Self::Output, Error> {
        Self::Output::read(input).map_err(|e| e.into())
    }
}

pub struct DummyCRHGadget<F, P>
where
    F: PrimeField,
    P: PoseidonRoundParams<F>,
{
    _crh_gadget: poseidon::constraints::CRHGadget<F, P>,
}

impl<'a, F, P> CRHGadget<DummyCRHCompressor<F, P>, F> for DummyCRHGadget<F, P>
where
    F: PrimeField + FromBytes,
    P: PoseidonRoundParams<F>,
{
    type OutputVar =
        <poseidon::constraints::CRHGadget<F, P> as CRHGadget<poseidon::CRH<F, P>, F>>::OutputVar;

    type ParametersVar = <poseidon::constraints::CRHGadget<F, P> as CRHGadget<
        poseidon::CRH<F, P>,
        F,
    >>::ParametersVar;

    fn evaluate(
        parameters: &Self::ParametersVar,
        input: &[ark_r1cs_std::uint8::UInt8<F>],
    ) -> Result<Self::OutputVar, ark_relations::r1cs::SynthesisError> {
        <poseidon::constraints::CRHGadget<F, P> as CRHGadget<poseidon::CRH<F, P>, F>>::evaluate(
            parameters, input,
        )
    }
}
