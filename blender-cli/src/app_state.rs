use std::cmp::Ordering;

use aleph_client::AccountId;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{DepositId, TokenAmount, TokenId};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Deposit {
    pub deposit_id: DepositId,
    pub token_id: TokenId,
    pub token_amount: TokenAmount,
    pub leaf_idx: u32,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct AppState {
    #[serde(skip)]
    pub caller_seed: String,
    pub node_address: String,
    pub contract_address: AccountId,

    deposit_counter: DepositId,
    deposits: Vec<Deposit>,
}

const DEFAULT_NODE_ADDRESS: &str = "ws://127.0.0.1:9944";

impl Default for AppState {
    fn default() -> Self {
        Self {
            caller_seed: String::new(),
            node_address: DEFAULT_NODE_ADDRESS.to_string(),
            contract_address: AccountId::new([0u8; 32]),
            deposit_counter: 0,
            deposits: Default::default(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Asset {
    pub token_id: TokenId,
    pub token_amount: TokenAmount,
}

impl PartialOrd<Self> for Asset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Asset {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else if self.token_id < other.token_id
            || (self.token_id == other.token_id && self.token_amount > other.token_amount)
        {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl AppState {
    pub fn get_all_assets(&self) -> Vec<Asset> {
        self.deposits
            .iter()
            .map(|d| Asset {
                token_id: d.token_id,
                token_amount: d.token_amount,
            })
            .sorted()
            .collect()
    }

    pub fn get_single_asset(&self, token_id: TokenId) -> Vec<Asset> {
        self.deposits
            .iter()
            .filter_map(|d| {
                (token_id == d.token_id).then_some(Asset {
                    token_id: d.token_id,
                    token_amount: d.token_amount,
                })
            })
            .sorted()
            .collect()
    }

    pub fn add_deposit(&mut self, token_id: TokenId, token_amount: TokenAmount, leaf_idx: u32) {
        self.deposits.push(Deposit {
            deposit_id: self.deposit_counter,
            token_id,
            token_amount,
            leaf_idx,
        });
        self.deposit_counter += 1;
    }
}
