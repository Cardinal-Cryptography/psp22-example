use aleph_client::{Connection, SignedConnection};

use crate::{
    commands::PSP22Cmd,
    contracts::psp22::{PSP22Contract, TokenMetadata},
};

pub async fn run(conn: Connection, contract: &PSP22Contract, cmd: PSP22Cmd) -> anyhow::Result<()> {
    match cmd {
        PSP22Cmd::Transfer {
            recipient,
            amount,
            account_seed,
        } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_connection(conn, keypair);
            contract
                .transfer(&signed_connection, &recipient, amount)
                .await
        }
        PSP22Cmd::TotalSupply => {
            let total_supply = contract.total_supply(&conn).await?;
            println!("Total supply of the underlying token: {:?}", total_supply);
            Ok(())
        }
        PSP22Cmd::GetTokenMetadata => {
            let token_name = contract.token_name(&conn).await?;
            let token_symbol = contract.token_symbol(&conn).await?;
            let token_decimals = contract.token_decimals(&conn).await?;
            let token_metadata = TokenMetadata {
                name: token_name,
                symbol: token_symbol,
                decimals: token_decimals,
            };
            println!("{:?}", token_metadata);
            Ok(())
        }
        PSP22Cmd::Approve {
            spender,
            amount,
            account_seed,
        } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_connection(conn, keypair);
            contract
                .approve(&signed_connection, &spender, amount)
                .await?;
            println!("Approved {:?} tokens to spend by {:?}", amount, &spender);
            Ok(())
        }
        PSP22Cmd::BalanceOf { account } => {
            let balance = contract.balance_of(&conn, &account).await?;
            println!("Balance of {:?}: {:?}", &account, balance);
            Ok(())
        }
        PSP22Cmd::Mint {
            account,
            balance,
            account_seed,
        } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_connection(conn, keypair);
            contract.mint(&signed_connection, &account, balance).await?;
            println!("Minted {:?} of tokens to {:?}.", balance, &account);
            Ok(())
        }
        PSP22Cmd::Burn {
            account,
            balance,
            account_seed,
        } => {
            let keypair = aleph_client::keypair_from_string(&account_seed);
            let signed_connection = SignedConnection::from_connection(conn, keypair);
            contract.burn(&signed_connection, &account, balance).await?;
            println!("Burnt {:?} of tokens from {:?}.", balance, &account);
            Ok(())
        }
    }
}
