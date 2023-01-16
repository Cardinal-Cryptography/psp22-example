use std::path::Path;

use aleph_client::{contract::ContractInstance, AccountId, Connection, SignedConnection};
use anyhow::{Context, Result};

#[derive(Debug)]
pub struct OwnableContract {
    contract: ContractInstance,
}

impl OwnableContract {
    pub fn new(address: AccountId, metadata_path: &Path) -> Result<Self> {
        let metadata_path: &str = metadata_path
            .to_str()
            .context("Ownable metadata not set.")?;
        Ok(Self {
            contract: ContractInstance::new(address, metadata_path)?,
        })
    }

    pub async fn renounce_ownership(&self, conn: &SignedConnection) -> Result<()> {
        self.contract
            .contract_exec0(conn, "Ownable::renounce_ownership")
            .await
    }

    pub async fn transfer_ownership(
        &self,
        conn: &SignedConnection,
        new_owner: &AccountId,
    ) -> Result<()> {
        self.contract
            .contract_exec(
                conn,
                "Ownable::transfer_ownership",
                &[new_owner.to_string()],
            )
            .await?;
        Ok(())
    }

    pub fn address(&self) -> AccountId {
        self.contract.address().clone()
    }
}
