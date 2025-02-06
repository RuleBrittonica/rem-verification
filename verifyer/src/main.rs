use config::{
    Config,
    File,
};
use clap::Parser;
use log::{
    // error,
    info
};
// use std::path::PathBuf;

// Local modules
mod args;
use args::{
    CLIArgs,
    CLICommands,
};
mod error;
mod logging;
mod messages;
mod convert;
use convert::coq_conversion;

mod local_config;
use crate::local_config::Settings;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging();

    info!("Application Started");

    let config = Config::builder()
        // "Config" here means it will look for a file named "Config.toml" by default.
        .add_source(File::with_name("Config")
        .required(true))
        .build()?;

    // log the settings
    let s: Settings = config.try_deserialize()?;
    info!("AENEAS path: {}", s.programs.aeneas);
    info!("Primitives.v path: {}", s.programs.primitives);
    info!("Original Coq path: {}", s.coq.original);
    info!("Refactored Coq path: {}", s.coq.refactored);
    info!("Original LLBC path: {}", s.llbc.original);
    info!("Refactored LLBC path: {}", s.llbc.refactored);

    let args: CLIArgs = CLIArgs::parse();
    match &args.command {
        CLICommands::Run {
            original_llbc,
            refactored_llbc,
            out_dir,
            verbose,
            top_level_function,
        } => {
            todo!()
        },

        CLICommands::Verify {
            original_coq,
            refactored_coq,
            verbose,
            top_level_function,
        } => {
            todo!()
        },

        CLICommands::Convert {
            original_llbc,
            refactored_llbc,
            out_dir,
            verbose,
        } => {
            if *verbose {
                info!("Starting LLBC to Coq conversion...");
                info!("Original LLBC: {:?}", original_llbc);
                info!("Refactored LLBC: {:?}", refactored_llbc);
                info!("Output directory: {:?}", out_dir);
            }

            // Call the conversion function.
            // Clone the paths because they are borrowed from the CLIArgs.
            coq_conversion(
                original_llbc.clone(),
                refactored_llbc.clone(),
                out_dir.clone()
            )?;

            info!("LLBC to Coq conversion completed successfully.");
            Ok(())
        },

        CLICommands::Test {
            verbose
        } => {
            todo!()
        },
    }
}