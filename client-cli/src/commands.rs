mod ownable;
mod psp22;

use clap::Subcommand;
pub use ownable::OwnableCmd;
pub use psp22::PSP22Cmd;

#[derive(Clone, Eq, PartialEq, Debug, Subcommand)]
pub enum Command {
    #[clap(subcommand, name = "psp22")]
    PSP22(PSP22Cmd),
    #[clap(subcommand, name = "ownable")]
    Ownable(OwnableCmd),
}
