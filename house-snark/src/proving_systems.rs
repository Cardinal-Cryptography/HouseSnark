use clap::{Args, Parser, Subcommand, ValueEnum};
use relations::{CircuitField, RawKeys};

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
        todo!()
    }

    pub fn prove(&self, relation: Relation, proving_key: Vec<u8>) -> Vec<u8> {
        todo!()
    }
}

impl UniversalProvingSystem {
    pub fn id(&self) -> String {
        todo!()
    }

    /// Generates proving and verifying key for `circuit` using `srs`. Returns serialized keys.
    pub fn generate_keys(&self, relation: Relation, srs: Vec<u8>) -> RawKeys {
        // match self {
        //     UniversalProvingSystem::Marlin => self._generate_keys::<_, Marlin>(circuit, srs),
        // }

        todo!()
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
