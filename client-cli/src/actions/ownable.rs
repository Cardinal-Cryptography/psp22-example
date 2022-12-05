use aleph_client::{Connection, SignedConnection};

use crate::{commands::OwnableCmd, contracts::ownable::OwnableContract};

pub fn run(conn: &Connection, contract: &OwnableContract, cmd: OwnableCmd) -> anyhow::Result<()> {
    match cmd {
        OwnableCmd::RenounceOwnership { account_seed } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_any_connection(conn, keypair);
            let _ = contract.renounce_ownership(&signed_connection)?;
            println!(
                "Renounced ownership over contract {:?}.",
                contract.address()
            );
            Ok(())
        }
        OwnableCmd::TransferOwnership {
            address,
            account_seed,
        } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_any_connection(conn, keypair);
            let _ = contract.transfer_ownership(&signed_connection, &address)?;
            println!(
                "Transferred ownership over {:?} to {:?}.",
                contract.address(),
                &address
            );
            Ok(())
        }
    }
}
