use std::cmp::Ordering;

use aleph_client::AccountId;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{TokenAmount, TokenId};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct Deposit {
    pub token_id: TokenId,
    pub token_amount: TokenAmount,
    pub leaf_idx: u32,
}

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct AppState {
    pub caller_seed: String,
    pub node_address: String,
    pub contract_address: AccountId,

    pub deposits: Vec<Deposit>,
}

const DEFAULT_SEED: &str = "//Alice";
const DEFAULT_NODE_ADDRESS: &str = "ws://127.0.0.1:9944";

impl Default for AppState {
    fn default() -> Self {
        Self {
            caller_seed: DEFAULT_SEED.to_string(),
            node_address: DEFAULT_NODE_ADDRESS.to_string(),
            contract_address: AccountId::new([0u8; 32]),
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
}
