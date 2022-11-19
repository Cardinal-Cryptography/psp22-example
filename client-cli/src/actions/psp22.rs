use aleph_client::{AccountId, Balance, Connection, SignedConnection};

use crate::{commands::PSP22Cmd, contracts::PSP22Contract};

fn transfer(
    conn: &SignedConnection,
    contract: &PSP22Contract,
    recipient: AccountId,
    amount: Balance,
) -> anyhow::Result<()> {
    contract.transfer(&conn, &recipient, amount)
}

pub fn run(conn: &Connection, contract: &PSP22Contract, cmd: PSP22Cmd) -> anyhow::Result<()> {
    match cmd {
        PSP22Cmd::Transfer {
            recipient,
            amount,
            account_seed,
        } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_any_connection(conn, keypair);
            transfer(&signed_connection, contract, recipient, amount)
        }
    }
}
