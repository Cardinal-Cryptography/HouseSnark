use anyhow::Result;
use subxt::{tx::Signer, OnlineClient};

use crate::{api, AlephConfig, RawVerificationKey, VerificationKeyIdentifier};

/// Calls `pallet_snarcos::store_key` with `identifier` and `vk`.
pub async fn store_key<S: Signer<AlephConfig> + Send + Sync>(
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
