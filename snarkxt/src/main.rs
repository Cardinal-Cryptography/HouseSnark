mod config;

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use pallet_snarcos::VerificationKeyIdentifier;
use subxt::{
    ext::sp_core::{sr25519, Pair},
    tx::{PairSigner, Signer},
    OnlineClient, PolkadotConfig,
};

use crate::config::{CliConfig, Command, StoreKeyCmd, VerifyCmd};

// The binary is supposed to be compiled from the root crate directory.
#[subxt::subxt(runtime_metadata_path = "artifacts/aleph_metadata.scale")]
pub mod aleph {}

fn read_bytes(file: &PathBuf) -> Result<Vec<u8>> {
    fs::read(file).map_err(|e| e.into())
}

/// Calls `pallet_snarcos::store_key` with `identifier` and `vk`.
async fn store_key<S: Signer<PolkadotConfig> + Send + Sync>(
    client: OnlineClient<PolkadotConfig>,
    signer: S,
    identifier: VerificationKeyIdentifier,
    vk: Vec<u8>,
) -> Result<()> {
    let tx = aleph::tx().snarcos().store_key(identifier, vk);
    let hash = client.tx().sign_and_submit_default(&tx, &signer).await?;

    println!(
        "✅ Successfully stored verification key. Submission took place in the block: {:?}",
        hash
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_config: CliConfig = CliConfig::parse();

    let signer = PairSigner::new(
        sr25519::Pair::from_string(&cli_config.signer, None)
            .map_err(|_| anyhow!("Cannot create signer from the seed"))?,
    );

    // We should be quite compatible to Polkadot.
    let client = OnlineClient::<PolkadotConfig>::from_url(&cli_config.node).await?;

    match cli_config.command {
        Command::StoreKey(StoreKeyCmd {
            identifier,
            vk_file,
        }) => {
            let vk = read_bytes(&vk_file)?;
            store_key(client, signer, identifier, vk).await?;
        }
        Command::Verify(VerifyCmd {
            identifier,
            proof_file,
            input_file,
            system,
        }) => {}
    }

    Ok(())
}
