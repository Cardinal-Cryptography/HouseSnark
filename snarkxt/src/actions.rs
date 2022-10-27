use anyhow::{anyhow, Result};
use codec::Encode;
use subxt::{
    error::DispatchError,
    events::StaticEvent,
    tx::{Signer, StaticTxPayload},
    Error, OnlineClient,
};

use crate::{
    aleph_api::{
        api,
        api::{
            runtime_types::pallet_snarcos::ProvingSystem,
            snarcos::events::{VerificationKeyStored, VerificationSucceeded},
        },
    },
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
    submit_tx::<_, _, VerificationKeyStored>(client, signer, tx).await
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
    submit_tx::<_, _, VerificationSucceeded>(client, signer, tx).await
}

async fn submit_tx<
    S: Signer<AlephConfig> + Send + Sync,
    CallData: Encode,
    SuccessEvent: StaticEvent,
>(
    client: OnlineClient<AlephConfig>,
    signer: S,
    tx: StaticTxPayload<CallData>,
) -> Result<()> {
    match client
        .tx()
        .sign_and_submit_then_watch_default(&tx, &signer)
        .await?
        .wait_for_finalized_success()
        .await
    {
        Ok(tx_events) => {
            if let Ok(Some(_)) = tx_events.find_first::<SuccessEvent>() {
                println!("✅ Extrinsic has been successful: {}", SuccessEvent::EVENT);
                Ok(())
            } else {
                eprintln!(
                    "❔ Extrinsic was finalized, but there is no error nor confirmation event"
                );
                Err(anyhow!("Unknown status"))
            }
        }
        Err(error) => {
            if let Error::Runtime(DispatchError::Module(ref error)) = error {
                eprintln!(
                    "❌ Extrinsic failed with an error: {}.\n\n{}",
                    error.error,
                    error.description.join("\n")
                )
            };
            Err(error.into())
        }
    }
}
