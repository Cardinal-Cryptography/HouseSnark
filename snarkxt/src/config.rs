use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use pallet_snarcos::{ProvingSystem, VerificationKeyIdentifier};

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
    #[clap(long, parse(try_from_str = parse_identifier))]
    pub identifier: VerificationKeyIdentifier,

    /// Path to a file containing the verification key.
    #[clap(long, parse(from_os_str))]
    pub vk_file: PathBuf,
}

#[derive(Clone, Eq, PartialEq, Debug, Args)]
pub(super) struct VerifyCmd {
    #[clap(long, parse(try_from_str = parse_identifier))]
    pub identifier: VerificationKeyIdentifier,

    /// Path to a file containing the proof.
    #[clap(long, parse(from_os_str))]
    pub proof_file: PathBuf,

    /// Path to a file containing the public input.
    #[clap(long, parse(from_os_str))]
    pub input_file: PathBuf,

    /// Which proving system should be used.
    #[clap(long, parse(try_from_str = parse_system))]
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
    match system {
        "Groth16" | "groth16" => Ok(ProvingSystem::Groth16),
        "GM17" | "Gm17" | "gm17" => Ok(ProvingSystem::Gm17),
        _ => Err(anyhow!("Unknown proving system")),
    }
}
