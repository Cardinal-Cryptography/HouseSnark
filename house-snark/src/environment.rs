use std::{fmt::Debug, marker::PhantomData};

use ark_gm17::GM17;
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_serialize::CanonicalDeserialize;
use clap::ValueEnum;
use either::Either;
use traits::{NonUniversalSystem, ProvingSystem};

use crate::serialization::serialize;

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
pub enum SomeProvingSystem {}

/// For now, we can settle with this curve and its scalar field.
pub type PairingEngine = ark_bls12_381::Bls12_381;
pub type CircuitField = ark_bls12_381::Fr;

/// Umbrella trait gathering all possible types of a proving system.
pub trait SystemClass {}

impl SystemClass for NonUniversalProvingSystem {}
impl SystemClass for UniversalProvingSystem {}
impl SystemClass for SomeProvingSystem {}

/// General type for a proving system.
///
/// Its generic parameter `S` specifies what kind of action can be done, e.g. `Environment` can
/// generate SRS iff `S` is `UniversalProvingSystem`.
///
/// The final system resolution is done under the hood, in the latest moment possible. This is done
/// by investigating `hint`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Environment<S: SystemClass> {
    hint: Either<NonUniversalProvingSystem, UniversalProvingSystem>,
    _phantom: PhantomData<S>,
}

/// API available for any proving system class.
impl<Class: SystemClass> Environment<Class> {
    /// System identifier.
    pub fn system_id(&self) -> String {
        either::for_both!(&self.hint, h => format!("{:?}", h)).to_lowercase()
    }

    /// Creates a non universal environment.
    pub fn with_non_universal_hint(
        hint: NonUniversalProvingSystem,
    ) -> Environment<NonUniversalProvingSystem> {
        Environment::<NonUniversalProvingSystem> {
            hint: Either::Left(hint),
            _phantom: Default::default(),
        }
    }

    /// Creates a universal environment.
    pub fn with_universal_hint(
        hint: UniversalProvingSystem,
    ) -> Environment<UniversalProvingSystem> {
        Environment::<UniversalProvingSystem> {
            hint: Either::Right(hint),
            _phantom: Default::default(),
        }
    }

    /// Converts `self` into `Environment<SomeProvingSystem>`.
    pub fn forget_class(self) -> Environment<SomeProvingSystem> {
        Environment::<SomeProvingSystem> {
            hint: self.hint,
            _phantom: Default::default(),
        }
    }

    /// Generates proof for `circuit` using proving key `pk`. Returns serialized proof.
    pub fn prove<C: ConstraintSynthesizer<CircuitField>>(
        &self,
        circuit: C,
        pk: Vec<u8>,
    ) -> Vec<u8> {
        match (self.hint.left(), self.hint.right()) {
            (Some(NonUniversalProvingSystem::Groth16), _) => {
                self._prove::<_, Groth16<PairingEngine>>(circuit, pk)
            }
            (Some(NonUniversalProvingSystem::Gm17), _) => {
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
impl Environment<NonUniversalProvingSystem> {
    /// Generates proving and verifying key for `circuit`. Returns serialized keys.
    pub fn generate_keys<C: ConstraintSynthesizer<CircuitField>>(&self, circuit: C) -> RawKeys {
        match self.hint.left().unwrap() {
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
impl Environment<UniversalProvingSystem> {
    /// Generates SRS. Returns in serialized version.
    pub fn generate_srs(&self) -> Vec<u8> {
        match self.hint.right().unwrap() {
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
        match self.hint.right().unwrap() {
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
