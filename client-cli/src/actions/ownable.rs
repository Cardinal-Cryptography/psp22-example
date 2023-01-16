use aleph_client::{Connection, SignedConnection};

use crate::{commands::OwnableCmd, contracts::ownable::OwnableContract};

pub async fn run(
    conn: Connection,
    contract: &OwnableContract,
    cmd: OwnableCmd,
) -> anyhow::Result<()> {
    match cmd {
        OwnableCmd::RenounceOwnership { account_seed } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_connection(conn, keypair);
            contract.renounce_ownership(&signed_connection).await?;
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
            let signed_connection = SignedConnection::from_connection(conn, keypair);
            contract
                .transfer_ownership(&signed_connection, &address)
                .await?;
            println!(
                "Transferred ownership over {:?} to {:?}.",
                contract.address(),
                &address
            );
            Ok(())
        }
    }
}
