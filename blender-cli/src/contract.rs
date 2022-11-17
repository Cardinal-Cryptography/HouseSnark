use std::{
    path::Path,
    sync::{mpsc::channel, Arc},
    thread,
    time::Duration,
};

use aleph_client::{
    contract::{
        event::{listen_contract_events, subscribe_events, ContractEvent},
        util::to_u128,
        ContractInstance,
    },
    AccountId, SignedConnection,
};
use anyhow::{anyhow, Result};

use crate::{Note, TokenAmount, TokenId};

#[derive(Debug)]
pub struct Blender {
    contract: Arc<ContractInstance>,
}

impl Blender {
    pub fn new(address: &AccountId, metadata_path: &Path) -> Result<Self> {
        Ok(Self {
            contract: Arc::new(ContractInstance::new(
                address.clone(),
                metadata_path.to_str().unwrap(),
            )?),
        })
    }

    /// Call `deposit` message of the contract. If successful, return leaf idx.
    pub fn deposit(
        &self,
        connection: &SignedConnection,
        token_id: TokenId,
        token_amount: TokenAmount,
        note: Note,
        proof: &[u8],
    ) -> Result<u32> {
        let subscription = subscribe_events(connection)?;
        let (cancel_tx, cancel_rx) = channel();
        let (leaf_tx, leaf_rx) = channel();

        let contract_clone = self.contract.clone();
        thread::spawn(move || {
            listen_contract_events(
                subscription,
                &[contract_clone.as_ref()],
                Some(cancel_rx),
                |event_or_error| {
                    println!("{:?}", event_or_error);
                    if let Ok(ContractEvent { ident, data, .. }) = event_or_error {
                        // todo: contain in the event `note` as well to identify unambiguously
                        if Some(String::from("Deposited")) == ident {
                            let leaf_idx = data.get("leaf_idx").unwrap().clone();
                            leaf_tx.send(to_u128(leaf_idx).unwrap()).unwrap();
                        }
                    }
                },
            );
        });

        self.contract
            .contract_exec(
                connection,
                "deposit",
                &vec![
                    &*token_id.to_string(),
                    &*token_amount.to_string(),
                    &*format!("0x{}", hex::encode(note)),
                    &*format!("0x{}", hex::encode(proof)),
                ],
            )
            .map_err(|e| {
                cancel_tx.send(()).unwrap();
                e
            })?;

        thread::sleep(Duration::from_secs(3));
        cancel_tx.send(()).unwrap();

        if let Ok(leaf_idx) = leaf_rx.try_recv() {
            println!("Successfully deposited tokens.");
            Ok(leaf_idx as u32)
        } else {
            Err(anyhow!(
                "Failed to observe expected event. And actually I do not know where are your tokens."
            ))
        }
    }
}
