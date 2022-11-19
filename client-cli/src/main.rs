mod actions;
mod cli;
mod commands;
mod contracts;

use anyhow::Context;
use clap::Parser;
use contracts::PSP22Contract;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let unsigned_connection = aleph_client::create_connection(&cli.node_address);

    match cli.command {
        commands::Command::PSP22(psp22_cmd) => {
            let contract = PSP22Contract::new(cli.contract_address, &*cli.contract_metadata)
                .context("failed to create an instance of PSP22Contract")?;
            actions::psp22::run(&unsigned_connection, &contract, psp22_cmd)?;
        }
    }
    Ok(())
}
