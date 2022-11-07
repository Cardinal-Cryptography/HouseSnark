use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::{
    environment::{NonUniversalProvingSystem, SomeProvingSystem, UniversalProvingSystem},
    relations::Relation,
};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Parser)]
#[clap(version = "1.0")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Subcommand)]
pub enum Command {
    /// Generate SRS and save it to a binary file.
    GenerateSrs(GenerateSrsCmd),

    // /// Generate verifying and proving key from SRS and save them to separate binary files.
    // GenerateKeysFromSrs(GenerateKeysFromSrsCmd),
    /// Generate verifying and proving key and save them to separate binary files.
    GenerateKeys(GenerateKeysCmd),

    /// Generate proof and public input and save them to separate binary files.
    // GenerateProof(GenerateProofCmd),

    /// Kill all Snarks!
    ///
    /// Remove all artifacts from the current directory.
    RedWedding,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateSrsCmd {
    /// Proving system to use.
    #[clap(long, short, value_enum, default_value = "marlin")]
    pub system: UniversalProvingSystem,

    /// Maximum supported number of constraints.
    #[clap(long, default_value = "100")]
    pub num_constraints: usize,

    /// Maximum supported number of variables.
    #[clap(long, default_value = "100")]
    pub num_variables: usize,

    /// Maximum supported polynomial degree.
    #[clap(long, default_value = "100")]
    pub degree: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateKeysFromSrsCmd {
    // /// Relation to work with.
    // #[clap(long, short)]
    // pub relation: Relation,
    /// Proving system to use.
    #[clap(long, short, value_enum, default_value = "marlin")]
    pub system: UniversalProvingSystem,

    /// Path to a file containing SRS.
    #[clap(long)]
    pub srs_file: PathBuf,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateKeysCmd {
    /// Relation to work with.
    #[clap(subcommand)]
    pub relation: Relation,

    /// Proving system to use.
    #[clap(long, short, value_enum, default_value = "groth16")]
    pub system: NonUniversalProvingSystem,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateProofCmd {
    // /// Relation to work with.
    // #[clap(long, short)]
    // pub relation: Relation,
    /// Proving system to use.
    ///
    /// Accepts either `NonUniversalProvingSystem` or `UniversalProvingSystem`.
    #[clap(long, short, value_enum, default_value = "groth16", value_parser = parse_some_system)]
    pub system: SomeProvingSystem,

    /// Path to a file containing proving key.
    #[clap(long, short)]
    pub proving_key_file: PathBuf,
}

fn parse_some_system(system: &str) -> Result<SomeProvingSystem> {
    let maybe_universal =
        UniversalProvingSystem::from_str(system, true).map(SomeProvingSystem::Universal);
    let maybe_non_universal =
        NonUniversalProvingSystem::from_str(system, true).map(SomeProvingSystem::NonUniversal);
    maybe_universal.or(maybe_non_universal).map_err(Error::msg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
