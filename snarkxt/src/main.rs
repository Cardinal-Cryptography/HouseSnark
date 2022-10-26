mod config;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use store_key::store_key;
use subxt::{
    ext::sp_core::{sr25519::Pair, Pair as _},
    tx::{PairSigner, Signer},
    OnlineClient, PolkadotConfig,
};
use verify::verify;

use crate::{
    aleph_api::{api, api::runtime_types::pallet_snarcos::ProvingSystem},
    config::{CliConfig, Command, StoreKeyCmd, VerifyCmd},
};

#[allow(clippy::all)]
mod aleph_api;
mod store_key;
mod verify;

/// This corresponds to `pallet_snarcos::VerificationKeyIdentifier`.
///
/// We copy this type alias to avoid a heavy dependency. In case of mismatch, subxt will detect it
/// in compilation time.
type VerificationKeyIdentifier = [u8; 4];

type RawVerificationKey = Vec<u8>;
type RawProof = Vec<u8>;
type RawPublicInput = Vec<u8>;

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
