use clap::ValueEnum;

/// All available proving systems.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, ValueEnum)]
pub enum ProvingSystem {
    Groth16,
    G17,
}
