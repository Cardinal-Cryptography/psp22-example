use aleph_client::{Connection, SignedConnection};

use crate::{commands::PSP22Cmd, contracts::PSP22Contract};

pub fn run(conn: &Connection, contract: &PSP22Contract, cmd: PSP22Cmd) -> anyhow::Result<()> {
    match cmd {
        PSP22Cmd::Transfer {
            recipient,
            amount,
            account_seed,
        } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_any_connection(conn, keypair);
            contract.transfer(&signed_connection, &recipient, amount)
        }
        PSP22Cmd::TotalSupply => {
            let total_supply = contract.total_supply(&conn)?;
            println!("Total supply of the underlying token: {:?}", total_supply);
            Ok(())
        },
    }
}
