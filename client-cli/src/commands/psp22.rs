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
    /// Prints metadata of the underlying token.
    GetTokenMetadata,
    /// Approves `spender` to spend the `amount` from the signer's balance.
    Approve {
        /// Spender.
        #[arg(short = 's')]
        spender: AccountId,
        /// Amount allowed to be spent.
        #[arg(short = 'a')]
        amount: Balance,
        /// Account seed which is used for submitting transactions.
        #[arg(long, short = 's')]
        account_seed: String,
    },
    /// Returns balance of the underlying token for the chosen account.
    BalanceOf {
        /// Account to query for the balance.
        #[arg(short = 'a')]
        account: AccountId,
    },
    /// Mints `amount` of new tokens into `account`'s balance.
    Mint {
        /// Account to mint new tokens into.
        #[arg(short = 'a')]
        account: AccountId,
        /// Amount of new tokens to mint.
        #[arg(short = 'b')]
        balance: Balance,
        /// Account seed which is used for submitting transactions.
        #[arg(long, short = 's')]
        account_seed: String,
    },
    /// Burns the `amount` of tokens from the `account`'s balance.
    Burn {
        /// Account to burn the tokens from.
        #[arg(short = 'a')]
        account: AccountId,
        /// Amount of tokens to be burnt.
        #[arg(short = 'b')]
        balance: Balance,
        /// Account seed which is used for submitting transactions.
        #[arg(long, short = 's')]
        account_seed: String,
    },
}
