use std::path::Path;

use aleph_client::{contract::ContractInstance, AccountId, Connection, SignedConnection};
use anyhow::{Context, Result};

use crate::utils::Balance;

#[derive(Debug)]
pub struct PSP22Contract {
    contract: ContractInstance,
}

impl PSP22Contract {
    pub fn new(address: AccountId, metadata_path: &Path) -> Result<Self> {
        let metadata_path: &str = metadata_path
            .to_str()
            .context("PSP22Token metadata not set.")?;
        Ok(Self {
            contract: ContractInstance::new(address, metadata_path)?,
        })
    }

    pub async fn approve(
        &self,
        conn: &SignedConnection,
        spender: &AccountId,
        amount: Balance,
    ) -> Result<()> {
        self.contract
            .contract_exec(
                conn,
                "PSP22::approve",
                &[spender.to_string(), amount.to_string()],
            )
            .await
    }

    pub async fn total_supply(&self, conn: &Connection) -> Result<Balance> {
        self.contract
            .contract_read0(conn, "PSP22::total_supply")
            .await
    }

    pub async fn balance_of(&self, conn: &Connection, account: &AccountId) -> Result<Balance> {
        self.contract
            .contract_read(conn, "PSP22::balance_of", &[account.to_string()])
            .await
    }

    pub async fn transfer(
        &self,
        conn: &SignedConnection,
        to: &AccountId,
        amount: Balance,
    ) -> Result<()> {
        self.contract
            .contract_exec(
                conn,
                "PSP22::transfer",
                &[to.to_string(), amount.to_string(), "0x00".to_string()],
            )
            .await
    }

    pub async fn mint(
        &self,
        conn: &SignedConnection,
        to: &AccountId,
        amount: Balance,
    ) -> Result<()> {
        self.contract
            .contract_exec(
                conn,
                "PSP22Mintable::mint",
                &[to.to_string(), amount.to_string()],
            )
            .await
    }

    pub async fn burn(
        &self,
        conn: &SignedConnection,
        to: &AccountId,
        amount: Balance,
    ) -> Result<()> {
        self.contract
            .contract_exec(
                conn,
                "PSP22Burnable::burn",
                &[to.to_string(), amount.to_string()],
            )
            .await
    }

    // NOTE: Missing methods are:
    //       decrease_allowance, increase_allowance,

    ////////////////////////
    // TOKEN METADATA
    ////////////////////////

    //NOTE: The two following PSP22Metadata methods return `Option<ink_lang::String>`
    //      which is a wrapper for `Bytes` - so contract is really returning `Option<Bytes>`.
    //      we do the mapping `Bytes -> String` here.
    pub async fn token_symbol(&self, conn: &Connection) -> Result<Option<String>> {
        self.contract
            .contract_read0(conn, "PSP22Metadata::token_symbol")
            .await
    }

    pub async fn token_name(&self, conn: &Connection) -> Result<Option<String>> {
        self.contract
            .contract_read0(conn, "PSP22Metadata::token_name")
            .await
    }

    pub async fn token_decimals(&self, conn: &Connection) -> Result<u128> {
        self.contract
            .contract_read0(conn, "PSP22Metadata::token_decimals")
            .await
    }

    /// Consumes self and returns the inner contract instance.
    pub fn into(self) -> ContractInstance {
        self.contract
    }
}

#[derive(Debug, Clone)]
pub struct TokenMetadata {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u128,
}
