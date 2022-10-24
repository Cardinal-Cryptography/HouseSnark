use std::path::PathBuf;

use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::{
    environment::ProvingSystem,
    environment2::{
        Environment, NonUniversalProvingSystem, SomeSystemClass, UniversalProvingSystem,
    },
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

    /// Generate verifying and proving key from SRS and save them to separate binary files.
    GenerateKeysFromSrs(GenerateKeysFromSrsCmd),

    /// Generate verifying and proving key and save them to separate binary files.
    GenerateKeys(GenerateKeysCmd),

    /// Generate proof and public input and save them to separate binary files.
    GenerateProof(GenerateProofCmd),

    /// Kill all Snarks!
    ///
    /// Remove all artifacts from the current directory.
    RedWedding,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateSrsCmd {
    /// Proving system to use. Must be universal.
    ///
    /// Accepts `UniversalProvingSystem` which will be converted to an
    /// `Environment<UniversalProvingSystem>`.
    #[clap(long = "system", short = 's', value_enum, default_value = "unimplemented", value_parser = parse_universal)]
    pub env: Environment<UniversalProvingSystem>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateKeysFromSrsCmd {
    /// Relation to work with.
    #[clap(long, short, value_enum)]
    pub relation: Relation,

    /// Proving system to use. Must be universal.
    ///
    /// Accepts `UniversalProvingSystem` which will be converted to an
    /// `Environment<UniversalProvingSystem>`.
    #[clap(long = "system", short = 's', value_enum, default_value = "unimplemented", value_parser = parse_universal)]
    pub env: Environment<UniversalProvingSystem>,

    /// Path to a file containing SRS.
    #[clap(long, short)]
    pub srs_file: PathBuf,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateKeysCmd {
    /// Relation to work with.
    #[clap(long, short, value_enum)]
    pub relation: Relation,

    /// Proving system to use. Must be non universal.
    ///
    /// Accepts `NonUniversalProvingSystem` which will be converted to an
    /// `Environment<NonUniversalProvingSystem>`.
    #[clap(long = "system", short = 's', value_enum, default_value = "groth16", value_parser = parse_non_universal)]
    pub env: Environment<NonUniversalProvingSystem>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateProofCmd {
    /// Relation to work with.
    #[clap(long, short, value_enum)]
    pub relation: Relation,

    /// Proving system to use.
    ///
    /// Accepts either `NonUniversalProvingSystem` or `UniversalProvingSystem` which will be
    /// converted to `Environment<SomeSystemClass>`.
    #[clap(long = "system", short = 's', value_enum, default_value = "groth16", value_parser = parse_some)]
    pub env: Environment<SomeSystemClass>,

    /// Path to a file containing proving key.
    #[clap(long, short)]
    pub proving_key_file: PathBuf,
}

fn parse_universal(system: &str) -> Result<Environment<UniversalProvingSystem>> {
    let system = UniversalProvingSystem::from_str(system, true).map_err(Error::msg)?;
    Ok(Environment::<SomeSystemClass>::with_universal_hint(system))
}

fn parse_non_universal(system: &str) -> Result<Environment<NonUniversalProvingSystem>> {
    let system = NonUniversalProvingSystem::from_str(system, true).map_err(Error::msg)?;
    Ok(Environment::<SomeSystemClass>::with_non_universal_hint(
        system,
    ))
}

fn parse_some(system: &str) -> Result<Environment<SomeSystemClass>> {
    let maybe_universal = UniversalProvingSystem::from_str(system, true)
        .map(|s| Environment::<SomeSystemClass>::with_universal_hint(s))
        .map(|e| e.forget_class());
    let maybe_non_universal = NonUniversalProvingSystem::from_str(system, true)
        .map(|s| Environment::<SomeSystemClass>::with_non_universal_hint(s))
        .map(|e| e.forget_class());
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
