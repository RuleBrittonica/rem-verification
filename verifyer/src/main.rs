use serde::Deserialize;
use config::{
    Config,
    File,
};
use clap::Parser;
use log::{
    // error,
    info
};
use std::path::PathBuf;

// Local modules
mod args;
use args::{
    CLIArgs,
    CLICommands,
};
mod error;
mod logging;
mod messages;

#[derive(Debug, Deserialize)]
struct Programs {
    aeneas: String,
}

#[derive(Debug, Deserialize)]
struct Coq {
    original: String,
    refactored: String,
    primitives: String,
}

#[derive(Debug, Deserialize)]
struct Llbc {
    original: String,
    refactored: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    programs: Programs,
    coq: Coq,
    llbc: Llbc,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging();

    info!("Application Started");

    let config = Config::builder()
        // "Config" here means it will look for a file named "Config.toml" by default.
        .add_source(File::with_name("Config").required(true))
        .build()?;

    // log the settings
    let s: Settings = config.try_deserialize()?;
    info!("AENEAS path: {}", s.programs.aeneas);
    info!("Original Coq path: {}", s.coq.original);
    info!("Refactored Coq path: {}", s.coq.refactored);
    info!("Primitives Coq path: {}", s.coq.primitives);
    info!("Original LLBC path: {}", s.llbc.original);
    info!("Refactored LLBC path: {}", s.llbc.refactored);

    let args: CLIArgs = CLIArgs::parse();
    match &args.command {
        CLICommands::Run {
            orginal_llbc,
            refactored_llbc,
            out_dir,
            verbose
        } => {
            todo!()
        },

        CLICommands::Convert {
            orginal_llbc,
            refactored_llbc,
            out_dir,
            verbose
        } => {
            todo!()
        },

        CLICommands::Verify {
            orginal_coq,
            refactored_coq,
            verbose } => {
                todo!()
            },

        CLICommands::Test {
            verbose
        } => {
            todo!()
        },
    }
}