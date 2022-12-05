use aleph_client::AccountId;
use clap::Subcommand;

#[derive(Clone, Eq, PartialEq, Debug, Subcommand)]
pub enum OwnableCmd {
    /// Renounces ownership over the contract - makes the guarded endpoints not callable.
    RenounceOwnership {
        /// Account seed which is used for submitting transactions.
        #[arg(long, short = 's')]
        account_seed: String,
    },
    /// Transfer ownership over the contract to a new account.
    TransferOwnership {
        /// Account address of the new owner.
        #[arg(short = 'a')]
        address: AccountId,
        /// Account seed which is used for submitting transactions.
        #[arg(long, short = 's')]
        account_seed: String,
    },
}
