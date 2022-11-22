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
use contract_transcode::Value;
use house_snark::bytes_from_note;

use crate::{MerkleRoot, Note, Nullifier, TokenAmount, TokenId};

#[derive(Debug)]
pub enum Relation {
    Deposit,
    Withdraw,
}

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

    /// Call `register_vk` message of the contract
    pub fn register_vk(
        &self,
        connection: &SignedConnection,
        relation: Relation,
        vk: Vec<u8>,
    ) -> Result<()> {
        // NOTE: this could still silently fail on-chain due to contract revert
        // we should add an event an listen to it

        let args = [
            &format!("{:?}", relation),
            &*format!("0x{}", hex::encode(vk)),
        ];

        println!("Calling register_vk tx with arguments {:?}", &args);

        self.contract
            .contract_exec(connection, "register_vk", &args)?;

        Ok(())
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
                        if Some(String::from("Deposited")) == ident {
                            let event_note: Value = data.get("note").unwrap().clone();
                            let decoded_note: [u64; 4] =
                                to_seq(&event_note).unwrap().try_into().unwrap();
                            // check the `note` in the event as well to identify unambiguously
                            if note.eq(&decoded_note) {
                                let leaf_idx = data.get("leaf_idx").unwrap().clone();
                                leaf_tx.send(to_u128(leaf_idx).unwrap()).unwrap();
                            }
                        }
                    }
                },
            );
        });

        let note_bytes = bytes_from_note(&note);

        let args = [
            &*token_id.to_string(),
            &*token_amount.to_string(),
            &*format!("0x{}", hex::encode(note_bytes)),
            &*format!("0x{}", hex::encode(proof)),
        ];

        println!("Calling deposit tx with arguments {:?}", &args);

        self.contract
            .contract_exec(connection, "deposit", &args)
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
                "Failed to observe expected event. And actually I do not know where your tokens are."
            ))
        }
    }

    /// Call `withdraw` message of the contract.
    #[allow(clippy::too_many_arguments)]
    pub fn withdraw(
        &self,
        connection: &SignedConnection,
        token_id: TokenId,
        value: TokenAmount,
        recipient: AccountId,
        fee_for_caller: Option<TokenAmount>,
        merkle_root: MerkleRoot,
        nullifier: Nullifier,
        new_note: Note,
        proof: &[u8],
    ) -> Result<u32> {
        let subscription = subscribe_events(connection)?;
        let (cancel_tx, cancel_rx) = channel();
        let (leaf_tx, leaf_rx) = channel();

        let contract_ptr = self.contract.clone();

        thread::spawn(move || {
            listen_contract_events(
                subscription,
                &[&contract_ptr],
                Some(cancel_rx),
                |event_or_error| {
                    println!("{:?}", event_or_error);
                    if let Ok(ContractEvent { ident, data, .. }) = event_or_error {
                        if Some(String::from("Withdrawn")) == ident {
                            let event_note: Value = data.get("new_note").unwrap().clone();
                            let decoded_note: [u64; 4] =
                                to_seq(&event_note).unwrap().try_into().unwrap();
                            // check the `new_note` in the event as well to identify it unambiguously
                            if new_note.eq(&decoded_note) {
                                let leaf_idx = data.get("leaf_idx").unwrap().clone();
                                leaf_tx.send(to_u128(leaf_idx).unwrap()).unwrap();
                            }
                        }
                    }
                },
            );
        });

        let new_note_bytes = bytes_from_note(&new_note);
        // TODO
        let merkle_root_bytes = bytes_from_note(&merkle_root);

        let args = [
            &*token_id.to_string(),
            &*value.to_string(),
            &*recipient.to_string(),
            &*format!("{:?}", fee_for_caller),
            &*format!("0x{}", hex::encode(merkle_root_bytes)),
            &*nullifier.to_string(),
            &*format!("0x{}", hex::encode(new_note_bytes)),
            &*format!("0x{}", hex::encode(proof)),
        ];

        println!("Calling withdraw tx with arguments {:?}", &args);

        self.contract
            .contract_exec(connection, "withdraw", &args)
            .map_err(|e| {
                cancel_tx.send(()).unwrap();
                e
            })?;

        thread::sleep(Duration::from_secs(3));
        cancel_tx.send(()).unwrap();

        if let Ok(leaf_idx) = leaf_rx.try_recv() {
            println!("Successfully withdrawn tokens.");
            Ok(leaf_idx as u32)
        } else {
            Err(anyhow!(
                "Failed to observe expected event. Funds may not be SAFU."
            ))
        }
    }

    /// Fetch the current merkle root.
    pub fn get_merkle_root(&self, _connection: &SignedConnection) -> MerkleRoot {
        Default::default()
    }
}

// TODO: could be made generic over elements
// TODO: move to aleph-client
fn to_seq(value: &Value) -> Result<Vec<u64>> {
    match value {
        Value::Seq(seq) => {
            let mut result = vec![];
            for element in seq.elems() {
                match element {
                    Value::UInt(integer) => result.push(*integer as u64),
                    _ => panic!("Expected {:?} to be an UInt", &element),
                }
            }
            Ok(result)
        }
        _ => Err(anyhow!("Expected {:?} to be a sequence", value)),
    }
}
