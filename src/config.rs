use clap::{Args, Parser, Subcommand};

use crate::relations::Relation;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Parser)]
#[clap(version = "1.0")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Subcommand)]
pub enum Command {
    /// Generate proof, verifying key and public input into separate binary files.
    Generate(GenerateCmd),

    /// Kill all Snarks!
    RedWedding,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Args)]
pub struct GenerateCmd {
    /// Which relation to work on.
    #[clap(long, arg_enum)]
    pub relation: Relation,
}
