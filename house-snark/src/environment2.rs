use std::{fmt::Debug, marker::PhantomData};

use ark_gm17::GM17;
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_serialize::CanonicalDeserialize;
use clap::ValueEnum;
use either::Either;
use traits::ProvingSystem;

use crate::{environment2::traits::NonUniversalSystem, serialization::serialize};

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

/// For now, we can settle with this curve and its scalar field.
pub type PairingEngine = ark_bls12_381::Bls12_381;
pub type CircuitField = ark_bls12_381::Fr;

pub trait SystemClass {}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SomeSystemClass {}

impl SystemClass for NonUniversalProvingSystem {}
impl SystemClass for UniversalProvingSystem {}
impl SystemClass for SomeSystemClass {}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Environment<S: SystemClass> {
    hint: Either<NonUniversalProvingSystem, UniversalProvingSystem>,
    _phantom: PhantomData<S>,
}

impl<Class: SystemClass> Environment<Class> {
    pub fn system_id(&self) -> String {
        either::for_both!(&self.hint, h => format!("{:?}", h)).to_lowercase()
    }

    pub fn with_non_universal_hint(
        hint: NonUniversalProvingSystem,
    ) -> Environment<NonUniversalProvingSystem> {
        Environment::<NonUniversalProvingSystem> {
            hint: Either::Left(hint),
            _phantom: Default::default(),
        }
    }

    pub fn with_universal_hint(
        hint: UniversalProvingSystem,
    ) -> Environment<UniversalProvingSystem> {
        Environment::<UniversalProvingSystem> {
            hint: Either::Right(hint),
            _phantom: Default::default(),
        }
    }

    pub fn forget_class(self) -> Environment<SomeSystemClass> {
        Environment::<SomeSystemClass> {
            hint: self.hint,
            _phantom: Default::default(),
        }
    }

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

pub struct RawKeys {
    pub pk: Vec<u8>,
    pub vk: Vec<u8>,
}

impl Environment<NonUniversalProvingSystem> {
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

impl Environment<UniversalProvingSystem> {
    pub fn generate_srs(&self) -> Vec<u8> {
        match self.hint.right().unwrap() {
            UniversalProvingSystem::Unimplemented => {
                unimplemented!()
            }
        }
    }

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

    use crate::environment2::CircuitField;

    pub trait ProvingSystem {
        type Proof: CanonicalSerialize + CanonicalDeserialize;
        type ProvingKey: CanonicalSerialize + CanonicalDeserialize;
        type VerifyingKey: CanonicalSerialize + CanonicalDeserialize;

        fn prove<C: ConstraintSynthesizer<CircuitField>>(
            pk: &Self::ProvingKey,
            circuit: C,
        ) -> Self::Proof;
    }

    pub trait UniversalSystem: ProvingSystem {
        type Srs: CanonicalSerialize + CanonicalDeserialize;

        fn generate_srs() -> Self::Srs;

        fn generate_keys<C: ConstraintSynthesizer<CircuitField>>(
            srs: &Self::Srs,
            circuit: C,
        ) -> (Self::VerifyingKey, Self::ProvingKey);
    }

    pub trait NonUniversalSystem: ProvingSystem {
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
