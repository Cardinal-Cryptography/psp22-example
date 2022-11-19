use std::path::PathBuf;

use aleph_client::AccountId;
use clap::Parser;

use crate::commands::Command;

/// Utilities for interacting with a sample PSP22 contract.
#[derive(Parser, Debug)]
pub struct Cli {
    /// WS endpoint address of the node.
    #[clap(short = 'n', long = "node", default_value = "ws://localhost:9944")]
    pub node_address: String,

    /// On-chain address of a contract
    #[clap(short = 'c')]
    pub contract_address: AccountId,

    /// Path to contract's metadata.
    #[clap(short = 'm', default_value = "metadata.json", value_parser  = parsing::parse_path)]
    pub contract_metadata: PathBuf,

    #[clap(subcommand)]
    pub command: Command,
}

mod parsing {
    use std::{path::PathBuf, str::FromStr};

    use anyhow::{Context, Result};

    pub(super) fn parse_path(path: &str) -> Result<PathBuf> {
        let expanded_path = shellexpand::full(path).context("failed to expand path")?;
        PathBuf::from_str(&expanded_path).context("failed to parse path")
    }
}
