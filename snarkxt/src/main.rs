mod config;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use subxt::{
    ext::sp_core::{sr25519::Pair, Pair as _},
    tx::PairSigner,
    OnlineClient, PolkadotConfig,
};

use crate::{
    actions::{store_key, verify},
    config::{CliConfig, Command, StoreKeyCmd, VerifyCmd},
};

mod actions;
#[allow(clippy::all)]
mod aleph_api;

/// This corresponds to `pallet_snarcos::VerificationKeyIdentifier`.
///
/// We copy this type alias to avoid a heavy dependency. In case of mismatch, subxt will detect it
/// in compilation time.
type VerificationKeyIdentifier = [u8; 4];

/// We should be quite compatible to Polkadot.
type AlephConfig = PolkadotConfig;

fn read_bytes(file: &PathBuf) -> Result<Vec<u8>> {
    fs::read(file).map_err(|e| e.into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_config = CliConfig::parse();

    let signer = PairSigner::new(
        Pair::from_string(&cli_config.signer, None)
            .map_err(|e| anyhow!("Cannot create signer from the seed: {:?}", e))?,
    );

    let client = OnlineClient::<AlephConfig>::from_url(&cli_config.node).await?;

    match cli_config.command {
        Command::StoreKey(StoreKeyCmd {
            identifier,
            vk_file,
        }) => {
            let vk = read_bytes(&vk_file)?;
            store_key(client, signer, identifier, vk).await
        }
        Command::Verify(VerifyCmd {
            identifier,
            proof_file,
            input_file,
            system,
        }) => {
            let proof = read_bytes(&proof_file)?;
            let input = read_bytes(&input_file)?;
            verify(client, signer, identifier, proof, input, system).await
        }
    }
    .map_err(|e| e.into())
}
