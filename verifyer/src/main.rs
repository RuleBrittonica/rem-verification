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

mod verify;
use verify::coq_verification;

pub mod local_config;
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
    info!("CHARON path: {}", s.programs.charon);
    info!("Primitives path: {}", s.files.primitives);

    let args: CLIArgs = CLIArgs::parse();
    match &args.command {
        CLICommands::Run {
            original_llbc,
            refactored_llbc,
            out_dir,
            verbose,
            top_level_function,
            cleanup,
        } => {
            // First we need to convert the files!
            let (original_coq, refactored_coq) = coq_conversion(
                &original_llbc,
                &refactored_llbc,
                &out_dir
            )?;

            info!("LLBC to CoQ conversion completed successfully.");

            // Now we can run the verification.
            let (coq_project, equivcheck, primitives, success) = coq_verification(
                &original_coq,
                &refactored_coq,
                &top_level_function
            )?;

            if success {
                info!("Verification completed successfully.");
            } else {
                info!("Verification failed.");
            }

            if *cleanup {
                // Cleanup the files
                std::fs::remove_file(&original_coq)?;
                std::fs::remove_file(&refactored_coq)?;
                std::fs::remove_file(&coq_project)?;
                std::fs::remove_file(&equivcheck)?;
                std::fs::remove_file(&primitives)?;
            }

            eprintln!("COQ_PROJECT: {}", coq_project.display());
            eprintln!("EQUIVCHECK: {}", equivcheck.display());
            eprintln!("PRIMITIVES: {}", primitives.display());
            eprintln!("SUCCESS: {}", success);
            eprintln!("OUTPUT_END");

            // If successful, exit with 0. Otherwise, exit with 1.
            if success {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }

            Ok(())
        },

        CLICommands::Verify {
            original_coq,
            refactored_coq,
            top_level_function,
            verbose,
        } => {
            let paths: (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf, bool) = coq_verification(
                &original_coq,
                &refactored_coq,
                &top_level_function,
            )?;

            Ok(())
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
            let coq_converstion_res: (std::path::PathBuf, std::path::PathBuf) = coq_conversion(
                &original_llbc,
                &refactored_llbc,
                &out_dir,
            )?;

            info!("LLBC to Coq conversion completed successfully.");
            Ok(())
        },

        CLICommands::Test {
            verbose
        } => {
            todo!("Implement the test command.");
        },
    }
}