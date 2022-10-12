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
    #[clap(long, short, arg_enum)]
    pub relation: Relation,

    /// Which proving system to use.
    #[clap(long, short, arg_enum, default_value = "groth16")]
    pub system: ProvingSystem,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateProofCmd {
    /// Which relation to work on.
    #[clap(long, short, arg_enum)]
    pub relation: Relation,

    /// Which proving system to use.
    #[clap(long, short, arg_enum)]
    pub system: ProvingSystem,

    /// Path to a file containing proving key.
    #[clap(long, short, parse(from_os_str))]
    pub proving_key_file: PathBuf,
}
