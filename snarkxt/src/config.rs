use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};

use crate::{
    aleph_api::api::runtime_types::pallet_snarcos::ProvingSystem, VerificationKeyIdentifier,
};

#[derive(Clone, Eq, PartialEq, Debug, Parser)]
pub(super) struct CliConfig {
    #[clap(subcommand)]
    pub command: Command,

    /// WS endpoint address of the node to connect to.
    #[clap(long, default_value = "ws://127.0.0.1:9944")]
    pub node: String,

    /// Seed of the submitting account.
    #[clap(long, default_value = "//Alice")]
    pub signer: String,

    /// Whether we want to just submit an extrinsic, without waiting for a confirmation event.
    #[clap(long)]
    pub skip_confirm: bool,
}

#[derive(Clone, Eq, PartialEq, Debug, Subcommand)]
pub(super) enum Command {
    /// Store a verification key under an identifier in the pallet's storage.
    StoreKey(StoreKeyCmd),

    /// Verify a proof against public input with a stored verification key.
    Verify(VerifyCmd),
}

#[derive(Clone, Eq, PartialEq, Debug, Args)]
pub(super) struct StoreKeyCmd {
    #[clap(long, value_parser = parse_identifier)]
    pub identifier: VerificationKeyIdentifier,

    /// Path to a file containing the verification key.
    #[clap(long)]
    pub vk_file: PathBuf,
}

#[derive(Clone, Eq, PartialEq, Debug, Args)]
pub(super) struct VerifyCmd {
    #[clap(long, value_parser = parse_identifier)]
    pub identifier: VerificationKeyIdentifier,

    /// Path to a file containing the proof.
    #[clap(long)]
    pub proof_file: PathBuf,

    /// Path to a file containing the public input.
    #[clap(long)]
    pub input_file: PathBuf,

    /// Which proving system should be used.
    #[clap(long, value_parser = parse_system)]
    pub system: ProvingSystem,
}

/// Try to convert `&str` to `VerificationKeyIdentifier`.
///
/// We handle one, most probable error type ourselves (i.e. incorrect length) to give a better
/// message than the default `"could not convert slice to array"`.
fn parse_identifier(ident: &str) -> Result<VerificationKeyIdentifier> {
    match ident.len() {
        4 => Ok(ident.as_bytes().try_into()?),
        _ => Err(anyhow!(
            "Identifier has an incorrect length (should be 4 characters)"
        )),
    }
}

/// Try to convert `&str` to `ProvingSystem`.
fn parse_system(system: &str) -> Result<ProvingSystem> {
    match system.to_lowercase().as_str() {
        "groth16" => Ok(ProvingSystem::Groth16),
        "gm17" => Ok(ProvingSystem::Gm17),
        _ => Err(anyhow!("Unknown proving system")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CliConfig::command().debug_assert()
    }
}
