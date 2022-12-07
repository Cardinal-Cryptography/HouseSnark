use clap::{Args, Parser, Subcommand, ValueEnum};
use relations::{
    serialize, CanonicalDeserialize, CanonicalSerialize, CircuitField, Groth16, Marlin,
    ProvingSystem, RawKeys, UniversalSystem, GM17,
};

use crate::relations::Relation;

/// All available universal proving systems.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum UniversalProvingSystem {
    Marlin,
}

/// All available non universal proving systems.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum NonUniversalProvingSystem {
    Groth16,
    Gm17,
}

/// Any proving system.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AnyProvingSystem {
    NonUniversal(NonUniversalProvingSystem),
    Universal(UniversalProvingSystem),
}

impl AnyProvingSystem {
    pub fn id(&self) -> String {
        match self {
            AnyProvingSystem::NonUniversal(s) => s.id(),
            AnyProvingSystem::Universal(s) => s.id(),
        }
    }

    pub fn prove(&self, relation: Relation, proving_key: Vec<u8>) -> Vec<u8> {
        match self {
            AnyProvingSystem::NonUniversal(NonUniversalProvingSystem::Groth16) => {
                let pk = <<Groth16 as ProvingSystem>::ProvingKey>::deserialize(&*proving_key)
                    .expect("Failed to deserialize proving key");
                let proof = <Groth16 as ProvingSystem>::prove(&pk, relation);
                serialize(&proof)
            }
            AnyProvingSystem::NonUniversal(NonUniversalProvingSystem::Gm17) => {
                let pk = <<GM17 as ProvingSystem>::ProvingKey>::deserialize(&*proving_key)
                    .expect("Failed to deserialize proving key");
                let proof = <GM17 as ProvingSystem>::prove(&pk, relation);
                serialize(&proof)
            }
            AnyProvingSystem::Universal(UniversalProvingSystem::Marlin) => {
                let pk = <<Marlin as ProvingSystem>::ProvingKey>::deserialize(&*proving_key)
                    .expect("Failed to deserialize proving key");
                let proof = <Marlin as ProvingSystem>::prove(&pk, relation);
                serialize(&proof)
            }
        }
    }
}

impl UniversalProvingSystem {
    pub fn id(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }

    /// Generates proving and verifying key for `circuit` using `srs`. Returns serialized keys.
    pub fn generate_keys(&self, relation: Relation, srs: Vec<u8>) -> RawKeys {
        match self {
            UniversalProvingSystem::Marlin => {
                let srs = <<Marlin as UniversalSystem>::Srs>::deserialize(&*srs)
                    .expect("Failed to deserialize srs");
                let (pk, vk) = <Marlin as UniversalSystem>::generate_keys(relation, &srs);

                RawKeys {
                    pk: serialize(&pk),
                    vk: serialize(&vk),
                }
            }
        }
    }

    pub fn generate_srs(
        &self,
        num_constraints: usize,
        num_variables: usize,
        degree: usize,
    ) -> Vec<u8> {
        todo!()
    }
}

impl NonUniversalProvingSystem {
    pub fn id(&self) -> String {
        todo!()
    }

    pub fn generate_keys(&self, circuit: Relation) -> RawKeys {
        // match self {
        //     NonUniversalProvingSystem::Groth16 => self._generate_keys::<_, Groth16>(circuit),
        //     NonUniversalProvingSystem::Gm17 => self._generate_keys::<_, GM17>(circuit),
        // }

        todo!()
    }
}
