use ark_bls12_381::Bls12_381;
use ark_ec::PairingEngine;
use ark_gm17::GM17;
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_snark::SNARK;
use ark_std::rand::{CryptoRng, RngCore};
use clap::ValueEnum;

/// All available proving systems.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum ProvingSystem {
    Groth16,
    Gm17,
}

pub struct GrothEnv;

impl Environment for GrothEnv {
    type PairingEngine = Bls12_381;
    type System = Groth16<Bls12_381>;

    fn id() -> &'static str {
        "groth"
    }
}

pub struct GmEnv;

impl Environment for GmEnv {
    type PairingEngine = Bls12_381;
    type System = GM17<Bls12_381>;

    fn id() -> &'static str {
        "gm"
    }
}

/// Some type aliases to make working with `Environment` a bit more concise.
pub type Fr<Env> = <<Env as Environment>::PairingEngine as PairingEngine>::Fr;
pub type ProvingKey<Env> = <<Env as Environment>::System as SNARK<Fr<Env>>>::ProvingKey;
pub type VerifyingKey<Env> = <<Env as Environment>::System as SNARK<Fr<Env>>>::VerifyingKey;
pub type Proof<Env> = <<Env as Environment>::System as SNARK<Fr<Env>>>::Proof;
pub type SystemError<Env> = <<Env as Environment>::System as SNARK<Fr<Env>>>::Error;

/// Full configuration of the proving system
pub trait Environment {
    type PairingEngine: PairingEngine;
    type System: SNARK<Fr<Self>>;

    /// String identifier of the system.
    fn id() -> &'static str;

    /// Alias for `Self::System::circuit_specific_setup`.
    fn setup<C: ConstraintSynthesizer<Fr<Self>>, R: RngCore + CryptoRng>(
        circuit: C,
        rng: &mut R,
    ) -> Result<(ProvingKey<Self>, VerifyingKey<Self>), SystemError<Self>> {
        Self::System::circuit_specific_setup(circuit, rng)
    }

    /// Alias for `Self::System::prove`.
    fn prove<C: ConstraintSynthesizer<Fr<Self>>, R: RngCore + CryptoRng>(
        pk: &ProvingKey<Self>,
        circuit: C,
        rng: &mut R,
    ) -> Result<Proof<Self>, SystemError<Self>> {
        Self::System::prove(pk, circuit, rng)
    }
}
