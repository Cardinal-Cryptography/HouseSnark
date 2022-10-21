use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::{environment::ProvingSystem, relations::Relation};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Parser)]
#[clap(version = "1.0")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Subcommand)]
pub enum Command {
    /// Generate verifying and proving key into separate binary files.
    GenerateKeys(GenerateKeysCmd),

    /// Generate proof and public input into separate binary files.
    GenerateProof(GenerateProofCmd),

    /// Kill all Snarks!
    RedWedding,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateKeysCmd {
    /// Which relation to work on.
    #[clap(long, short, value_enum)]
    pub relation: Relation,

    /// Which proving system to use.
    #[clap(long, short, value_enum, default_value = "groth16")]
    pub system: ProvingSystem,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateProofCmd {
    /// Which relation to work on.
    #[clap(long, short, value_enum)]
    pub relation: Relation,

    /// Which proving system to use.
    #[clap(long, short, value_enum, default_value = "groth16")]
    pub system: ProvingSystem,

    /// Path to a file containing proving key.
    #[clap(long, short)]
    pub proving_key_file: PathBuf,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
