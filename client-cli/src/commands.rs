mod psp22;

use clap::Subcommand;
pub use psp22::PSP22Cmd;

#[derive(Clone, Eq, PartialEq, Debug, Subcommand)]
pub enum Command {
    #[clap(subcommand, name = "psp22")]
    PSP22(PSP22Cmd),
}
