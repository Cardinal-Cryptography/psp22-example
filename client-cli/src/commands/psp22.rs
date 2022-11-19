use aleph_client::{AccountId, Balance};
use clap::Subcommand;

#[derive(Clone, Eq, PartialEq, Debug, Subcommand)]
pub enum PSP22Cmd {
    /// Transfer the desired amount tokens from the caller to the recipient.
    Transfer {
        /// Recipient of the transfer.
        #[arg(long, short = 'r')]
        recipient: AccountId,
        /// Amount of tokens to transfer.
        #[arg(long, short = 'a')]
        amount: Balance,
        /// Account seed which is used for submitting transactions.
        #[arg(long, short = 's')]
        account_seed: String,
    },
    /// Prints total supply of the underlying token.
    TotalSupply,
}
