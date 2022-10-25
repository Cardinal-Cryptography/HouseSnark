use std::fmt::Debug;

use ark_gm17::GM17;
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_serialize::CanonicalDeserialize;
use clap::ValueEnum;
use traits::{NonUniversalSystem, ProvingSystem};

use crate::serialization::serialize;

/// For now, we can settle with this curve and its scalar field.
pub type PairingEngine = ark_bls12_381::Bls12_381;
pub type CircuitField = ark_bls12_381::Fr;

/// All available non universal proving systems.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum NonUniversalProvingSystem {
    Groth16,
    Gm17,
}

/// All available universal proving systems.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum UniversalProvingSystem {
    Unimplemented,
}

/// Any proving system.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SomeProvingSystem {
    NonUniversal(NonUniversalProvingSystem),
    Universal(UniversalProvingSystem),
}

/// Common API for all systems.
impl SomeProvingSystem {
    pub fn id(&self) -> String {
        match self {
            SomeProvingSystem::NonUniversal(s) => s.id(),
            SomeProvingSystem::Universal(s) => s.id(),
        }
    }

    /// Generates proof for `circuit` using proving key `pk`. Returns serialized proof.
    pub fn prove<C: ConstraintSynthesizer<CircuitField>>(
        &self,
        circuit: C,
        pk: Vec<u8>,
    ) -> Vec<u8> {
        use SomeProvingSystem::*;

        match self {
            NonUniversal(NonUniversalProvingSystem::Groth16) => {
                self._prove::<_, Groth16<PairingEngine>>(circuit, pk)
            }
            NonUniversal(NonUniversalProvingSystem::Gm17) => {
                self._prove::<_, GM17<PairingEngine>>(circuit, pk)
            }
            _ => unimplemented!(),
        }
    }

    fn _prove<C: ConstraintSynthesizer<CircuitField>, S: ProvingSystem>(
        &self,
        circuit: C,
        pk: Vec<u8>,
    ) -> Vec<u8> {
        let pk = <S::ProvingKey>::deserialize(&*pk).expect("Failed to deserialize proving key");
        let proof = S::prove(&pk, circuit);
        serialize(&proof)
    }
}

/// Serialized keys.
pub struct RawKeys {
    pub pk: Vec<u8>,
    pub vk: Vec<u8>,
}

/// API available only for non universal proving systems.
impl NonUniversalProvingSystem {
    pub fn id(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }

    /// Generates proving and verifying key for `circuit`. Returns serialized keys.
    pub fn generate_keys<C: ConstraintSynthesizer<CircuitField>>(&self, circuit: C) -> RawKeys {
        match self {
            NonUniversalProvingSystem::Groth16 => {
                self._generate_keys::<_, Groth16<PairingEngine>>(circuit)
            }
            NonUniversalProvingSystem::Gm17 => {
                self._generate_keys::<_, GM17<PairingEngine>>(circuit)
            }
        }
    }

    fn _generate_keys<C: ConstraintSynthesizer<CircuitField>, S: NonUniversalSystem>(
        &self,
        circuit: C,
    ) -> RawKeys {
        let (pk, vk) = S::generate_keys(circuit);
        RawKeys {
            pk: serialize(&pk),
            vk: serialize(&vk),
        }
    }
}

/// API available only for universal proving systems.
impl UniversalProvingSystem {
    pub fn id(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }

    /// Generates SRS. Returns in serialized version.
    pub fn generate_srs(&self) -> Vec<u8> {
        match self {
            UniversalProvingSystem::Unimplemented => {
                unimplemented!()
            }
        }
    }

    /// Generates proving and verifying key for `circuit` using `srs`. Returns serialized keys.
    pub fn generate_keys<C: ConstraintSynthesizer<CircuitField>>(
        &self,
        _circuit: C,
        _srs: Vec<u8>,
    ) -> RawKeys {
        match self {
            UniversalProvingSystem::Unimplemented => {
                unimplemented!()
            }
        }
    }
}

pub mod traits {
    use ark_relations::r1cs::ConstraintSynthesizer;
    use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
    use ark_snark::SNARK;
    use ark_std::rand::{rngs::StdRng, SeedableRng};

    use super::CircuitField;

    /// Common API for every proving system.
    pub trait ProvingSystem {
        type Proof: CanonicalSerialize + CanonicalDeserialize;
        type ProvingKey: CanonicalSerialize + CanonicalDeserialize;
        type VerifyingKey: CanonicalSerialize + CanonicalDeserialize;

        /// Generates proof for `circuit` using proving key `pk`
        fn prove<C: ConstraintSynthesizer<CircuitField>>(
            pk: &Self::ProvingKey,
            circuit: C,
        ) -> Self::Proof;
    }

    /// Common API for every universal proving system.
    pub trait UniversalSystem: ProvingSystem {
        type Srs: CanonicalSerialize + CanonicalDeserialize;

        /// Generates SRS.
        fn generate_srs() -> Self::Srs;

        /// Generates proving and verifying key for `circuit` using `srs`.
        fn generate_keys<C: ConstraintSynthesizer<CircuitField>>(
            srs: &Self::Srs,
            circuit: C,
        ) -> (Self::ProvingKey, Self::VerifyingKey);
    }

    /// Common API for every non universal proving system.
    pub trait NonUniversalSystem: ProvingSystem {
        /// Generates proving and verifying key for `circuit`.
        fn generate_keys<C: ConstraintSynthesizer<CircuitField>>(
            circuit: C,
        ) -> (Self::ProvingKey, Self::VerifyingKey);
    }

    impl<S: SNARK<CircuitField>> ProvingSystem for S
    where
        <S as SNARK<CircuitField>>::Proof: CanonicalSerialize + CanonicalDeserialize,
        <S as SNARK<CircuitField>>::ProvingKey: CanonicalSerialize + CanonicalDeserialize,
        <S as SNARK<CircuitField>>::VerifyingKey: CanonicalSerialize + CanonicalDeserialize,
    {
        type Proof = <S as SNARK<CircuitField>>::Proof;
        type ProvingKey = <S as SNARK<CircuitField>>::ProvingKey;
        type VerifyingKey = <S as SNARK<CircuitField>>::VerifyingKey;

        fn prove<C: ConstraintSynthesizer<CircuitField>>(
            pk: &Self::ProvingKey,
            circuit: C,
        ) -> Self::Proof {
            let mut rng = StdRng::from_seed([0u8; 32]);
            <S as SNARK<CircuitField>>::prove(pk, circuit, &mut rng)
                .expect("Failed to generate keys")
        }
    }

    impl<S: SNARK<CircuitField>> NonUniversalSystem for S
    where
        <S as SNARK<CircuitField>>::Proof: CanonicalSerialize + CanonicalDeserialize,
        <S as SNARK<CircuitField>>::ProvingKey: CanonicalSerialize + CanonicalDeserialize,
        <S as SNARK<CircuitField>>::VerifyingKey: CanonicalSerialize + CanonicalDeserialize,
    {
        fn generate_keys<C: ConstraintSynthesizer<CircuitField>>(
            circuit: C,
        ) -> (Self::ProvingKey, Self::VerifyingKey) {
            let mut rng = StdRng::from_seed([0u8; 32]);
            <S as SNARK<CircuitField>>::circuit_specific_setup(circuit, &mut rng)
                .expect("Failed to generate keys")
        }
    }
}
