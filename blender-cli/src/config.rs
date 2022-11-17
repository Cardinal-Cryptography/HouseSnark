use std::path::PathBuf;

use aleph_client::AccountId;
use clap::{Args, Parser, Subcommand};

use crate::{TokenAmount, TokenId};

#[derive(Clone, Eq, PartialEq, Debug, Parser)]
pub(super) struct CliConfig {
    #[clap(long, default_value = "~/.blender-state.json", value_parser = parsing::parse_path)]
    pub state_file: PathBuf,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clone, Eq, PartialEq, Debug, Subcommand)]
pub(super) enum Command {
    SetSeed(SetSeedCmd),
    SetNode(SetNodeCmd),
    SetContractAddress(SetContractAddressCmd),

    Deposit(DepositCmd),
}

#[derive(Clone, Eq, PartialEq, Debug, Args)]
pub(super) struct SetSeedCmd {
    /// Seed of the submitting account.
    pub seed: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Args)]
pub(super) struct SetNodeCmd {
    /// WS endpoint address of the node to connect to.
    pub node: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Args)]
pub(super) struct SetContractAddressCmd {
    /// Address of the Blender contract.
    pub address: AccountId,
}

#[derive(Clone, Eq, PartialEq, Debug, Args)]
pub(super) struct DepositCmd {
    /// Registered token id.
    pub token_id: TokenId,

    /// Registered token id.
    pub amount: TokenAmount,
}

mod parsing {
    use std::{path::PathBuf, str::FromStr};

    use anyhow::{anyhow, Result};

    pub fn parse_path(path: &str) -> Result<PathBuf> {
        let expanded_path =
            shellexpand::full(path).map_err(|e| anyhow!("Failed to expand path: {e:?}"))?;
        PathBuf::from_str(expanded_path.as_ref())
            .map_err(|e| anyhow!("Failed to interpret path: {e:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CliConfig::command().debug_assert()
    }
}
