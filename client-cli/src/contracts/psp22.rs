use std::path::Path;

use aleph_client::{contract::ContractInstance, AccountId, Balance, Connection, SignedConnection};
use anyhow::{Context, Result};

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

    pub fn total_supply(&self, conn: &Connection) -> Result<Balance> {
        self.contract
            .contract_read0(conn, "PSP22::total_supply")?
            .try_into()
    }

    pub fn transfer(&self, conn: &SignedConnection, to: &AccountId, amount: Balance) -> Result<()> {
        self.contract.contract_exec(
            conn,
            "PSP22::transfer",
            &[to.to_string(), amount.to_string(), "0x00".to_string()],
        )
    }

    pub fn mint(&self, conn: &SignedConnection, to: &AccountId, amount: Balance) -> Result<()> {
        self.contract.contract_exec(
            conn,
            "PSP22Mintable::mint",
            &[to.to_string(), amount.to_string()],
        )
    }

    pub fn burn(&self, conn: &SignedConnection, to: &AccountId, amount: Balance) -> Result<()> {
        self.contract.contract_exec(
            conn,
            "PSP22Burnable::burn",
            &[to.to_string(), amount.to_string()],
        )
    }

    pub fn approve(
        &self,
        conn: &SignedConnection,
        spender: &AccountId,
        value: Balance,
    ) -> Result<()> {
        self.contract.contract_exec(
            conn,
            "PSP22::approve",
            &[spender.to_string(), value.to_string()],
        )
    }

    pub fn balance_of(&self, conn: &Connection, account: &AccountId) -> Result<Balance> {
        self.contract
            .contract_read(conn, "PSP22::balance_of", &[account.to_string()])?
            .try_into()
    }
}
