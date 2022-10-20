mod config;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use subxt::{
    ext::sp_core::{sr25519::Pair, Pair as _},
    tx::{PairSigner, Signer},
    OnlineClient, PolkadotConfig,
};

use crate::{
    aleph_api::{api, api::runtime_types::pallet_snarcos::ProvingSystem},
    config::{CliConfig, Command, StoreKeyCmd, VerifyCmd},
};

mod aleph_api;

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

/// Calls `pallet_snarcos::store_key` with `identifier` and `vk`.
async fn store_key<S: Signer<AlephConfig> + Send + Sync>(
    client: OnlineClient<AlephConfig>,
    signer: S,
    identifier: VerificationKeyIdentifier,
    vk: RawVerificationKey,
) -> Result<()> {
    let tx = api::tx().snarcos().store_key(identifier, vk);
    let hash = client.tx().sign_and_submit_default(&tx, &signer).await?;

    println!(
        "✅ Successfully submitted storing verification key request. \
        Submission took place in the block with hash: {:?}",
        hash
    );
    Ok(())
}

/// Calls `pallet_snarcos::verify` with `identifier`, `proof`, `public_input` and `system`.
async fn verify<S: Signer<AlephConfig> + Send + Sync>(
    client: OnlineClient<AlephConfig>,
    signer: S,
    identifier: VerificationKeyIdentifier,
    proof: RawProof,
    public_input: RawPublicInput,
    system: ProvingSystem,
) -> Result<()> {
    let tx = api::tx()
        .snarcos()
        .verify(identifier, proof, public_input, system);
    let hash = client.tx().sign_and_submit_default(&tx, &signer).await?;

    println!(
        "✅ Successfully submitted proof verification request. \
        Submission took place in the block with hash: {:?}",
        hash
    );
    Ok(())
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
