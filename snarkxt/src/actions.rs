use anyhow::Result;
use subxt::{tx::Signer, OnlineClient};

use crate::{
    aleph_api::{api, api::runtime_types::pallet_snarcos::ProvingSystem},
    AlephConfig, VerificationKeyIdentifier,
};

type RawVerificationKey = Vec<u8>;
type RawProof = Vec<u8>;
type RawPublicInput = Vec<u8>;

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

/// Calls `pallet_snarcos::verify` with `identifier`, `proof`, `public_input` and `system`.
pub async fn verify<S: Signer<AlephConfig> + Send + Sync>(
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
