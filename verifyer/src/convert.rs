//! This Module is responsible for converting the .llbc files to .v files.
//! There is the potential for it to be extended to convert to other formats
//! also supported by AENEAS.
//! I will need to experiment with verification to see what is most useful

use config::{
    Config,
    File as CfgFile,
};
use std::{
    fs::File as StdFile, // Rename the File struct to stdFile to avoid conflicts
    io::Write,
    path::PathBuf,
    process::Command,
};
use log::{info, warn};

// Local Modules
use crate::error::AENEASError; // TODO implement specific error handling for this module.
use crate::local_config::Settings;

///============================================================================
///------------------------------- CoQ Conversion -----------------------------
///============================================================================

/// Top level (public) method to convert the two LLBC files to CoQ files. Also
/// ensures that we have a Primitives.v file in the same directory.
/// The output directory is optional, and if not provided, the files will be
/// saved to the same directory as the original LLBC file. The function returns
/// the paths to the new CoQ files
pub fn coq_conversion(
    original_llbc: &PathBuf,
    refactored_llbc: &PathBuf,
    out_dir: &Option<PathBuf>,
) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let settings: Settings = get_config()?;
    let aeneas_path: PathBuf = get_aeneas_path(&settings);
    let _primitives_path: PathBuf = get_primitives_path(&settings);

    info!("Converting LLBC files to CoQ files");
    info!("Settings: {:?}", settings);
    info!("AENEAS path: {:?}", aeneas_path);

    // Check if the Primitives.v file exists in the directory
    let primitives_save_path: PathBuf = out_dir.clone().unwrap_or_else(|| {
        original_llbc
            .parent() // Get the directory of the original_llbc file.
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| original_llbc.clone())
    });
    create_primitives_file(primitives_save_path)?;

    // Convert the LLBC files to CoQ
    let original_coq_path: PathBuf = convert_llbc_to_coq(&aeneas_path, &original_llbc, out_dir.clone())?;
    let refactored_coq_path: PathBuf = convert_llbc_to_coq(&aeneas_path, &refactored_llbc, out_dir.clone())?;
    Ok((original_coq_path, refactored_coq_path))
}

/// Converts a LLBC file to a CoQ file using AENEAS.
/// Will call AENEAS as a subprocess and handle errors accordingly
/// AENEAS is called as follows:
/// ./<path_to_aeneas> -backend coq <"path_to_llbc"> -dest <"path_to_output">
/// Returns the path to the new CoQ file.
fn convert_llbc_to_coq(
    aeneas_path: &PathBuf,
    llbc_path: &PathBuf,
    out_dir: Option<PathBuf>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_dir: PathBuf = out_dir.unwrap_or_else(|| {
        llbc_path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| llbc_path.clone())
    });

    info!("Output directory: {:?}", output_dir);

    // Call AENEAS
    let output = Command::new(aeneas_path)
        .arg("-backend")
        .arg("coq")
        .arg(llbc_path)
        .arg("-dest")
        .arg(output_dir.clone())
        .output()?;
    if !output.status.success() {
        // Log the stderr and stdout from the process
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        warn!("AENEAS failed to convert the LLBC file to CoQ");
        warn!("Stderr: {}", stderr);
        warn!("Stdout: {}", stdout);
        return Err(Box::new(AENEASError::RuntimeError));
    }

    let returned_path: PathBuf = output_dir
        .join(llbc_path
            .file_stem()
            .unwrap()
        );
    Ok(returned_path)
}

/// Creates the Primitives.v file if needed.
/// Encodes the contents of `Primitives.v` directly into the executable using `include_str!`.
/// This is done to avoid having to deal with file paths and the potential for
/// the file to be missing.
/// The primitives_path is the path to the Primitives.v file (determined from
/// the config file).
fn create_primitives_file(
    out_dir: PathBuf, // A pathbuf must be provided here to the folder.
) -> Result<(), Box<dyn std::error::Error>> {
    let dest_file: PathBuf = out_dir.join("Primitives.v");
    if !dest_file.exists(){
        // Create the file
        let mut file: StdFile = StdFile::create(dest_file)?;
        file.write_all(include_str!("Primitives.v").as_bytes())?;
    } else {
        info!("Skipping creation of `Primitives.v` as it already exists")
    }
    Ok(())
}

fn get_primitives_path(settings: &Settings) -> PathBuf {
    let primitives_str:&String  = &settings.programs.primitives;
    PathBuf::from(primitives_str)
}

///============================================================================
/// ------------------------------ Fstar Conversion ---------------------------
/// ===========================================================================



/// ============================================================================
/// ------------------------------ MISC ----------------------------------------
/// ============================================================================

fn get_config() -> Result<Settings, Box<dyn std::error::Error>> {
    let config: Config = Config::builder()
        .add_source(CfgFile::with_name("Config")
        .required(true))
        .build()?;
    let s: Settings = config.try_deserialize()?;
    Ok(s)
}

fn get_aeneas_path(settings: &Settings) -> PathBuf {
    let aeneas_str:&String  = &settings.programs.aeneas;
    PathBuf::from(aeneas_str)
}