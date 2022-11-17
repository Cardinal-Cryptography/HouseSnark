use std::{
    path::Path,
    sync::{mpsc::channel, Arc},
    thread,
    time::Duration,
};

use aleph_client::{
    contract::{
        event::{listen_contract_events, subscribe_events},
        ContractInstance,
    },
    AccountId, SignedConnection,
};
use anyhow::Result;

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

    pub fn deposit(
        &self,
        connection: &SignedConnection,
        token_id: TokenId,
        token_amount: TokenAmount,
        note: Note,
        proof: &[u8],
    ) -> Result<()> {
        let subscription = subscribe_events(connection)?;
        let (cancel_tx, cancel_rx) = channel();

        let contract_clone = self.contract.clone();

        thread::spawn(move || {
            listen_contract_events(
                subscription,
                &[contract_clone.as_ref()],
                Some(cancel_rx),
                |event_or_error| println!("{:?}", event_or_error),
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

        thread::sleep(Duration::from_secs(5));
        cancel_tx.send(()).unwrap();

        Ok(())
    }
}
