use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

// Local modules
use crate::messages::{about::ABOUT, author::AUTHOR, version::VERSION};

#[derive(Parser)]
#[command(
    version = VERSION,
    about = ABOUT,
    author = AUTHOR,
)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: CLICommands,
}

#[derive(Subcommand)]
pub enum CLICommands {
    /// Run the whole process. Converts the LLBC files to CoQ, then
    /// runs verification checks on the CoQ files.
    Run {
        #[arg(help = "Path to the LLBC of the original program.")]
        orginal_llbc: PathBuf,

        #[arg(help = "Path to the LLBC of the refactored program.")]
        refactored_llbc: PathBuf,

        /// Optional output directory for the generated `.v` files
        #[arg(short, long, help = "Output directory for generated Coq files")]
        out_dir: Option<PathBuf>,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

    /// Convert the LLBC files to CoQ using AENEAS.
    Convert {
        #[arg(help = "Path to the LLBC of the original program.")]
        orginal_llbc: PathBuf,

        #[arg(help = "Path to the LLBC of the refactored program.")]
        refactored_llbc: PathBuf,

        /// Optional output directory for the generated `.llbc` files
        #[arg(short, long, help = "Output directory for generated llbc files")]
        out_dir: Option<PathBuf>,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

    /// Run various verification checks on the CoQ files.
    Verify {
        #[arg(help = "Path to the CoQ of the original program.")]
        orginal_coq: PathBuf,

        #[arg(help = "Path to the CoQ of the refactored program.")]
        refactored_coq: PathBuf,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,
    },

    /// Run the crate tests.
    Test {
        #[arg(short, long, help = "Enable verbose test output", action = ArgAction::SetTrue)]
        verbose: bool,
    },
}
