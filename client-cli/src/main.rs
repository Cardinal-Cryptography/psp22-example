mod actions;
mod cli;
mod commands;
mod contracts;
pub mod utils;

use anyhow::Context;
use clap::Parser;
use contracts::{ownable::OwnableContract, psp22::PSP22Contract};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let unsigned_connection = aleph_client::Connection::new(&cli.node_address).await;

    match cli.command {
        commands::Command::PSP22(psp22_cmd) => {
            let contract = PSP22Contract::new(cli.contract_address, &cli.contract_metadata)
                .context("failed to create an instance of PSP22Contract")?;
            actions::psp22::run(unsigned_connection, &contract, psp22_cmd).await?;
        }
        commands::Command::Ownable(ownable_cmd) => {
            let contract = OwnableContract::new(cli.contract_address, &cli.contract_metadata)
                .context("failed to create an instance of OwnableContract")?;
            actions::ownable::run(unsigned_connection, &contract, ownable_cmd).await?;
        }
    }
    Ok(())
}
