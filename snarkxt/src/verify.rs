use anyhow::Result;
use subxt::{tx::Signer, OnlineClient};

use crate::{api, AlephConfig, ProvingSystem, RawProof, RawPublicInput, VerificationKeyIdentifier};

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
