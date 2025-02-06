use serde::Deserialize;
use config::{
    Config,
    File,
};

#[derive(Debug, Deserialize)]
struct Programs {
    aeneas: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    programs: Programs,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        // "Config" here means it will look for a file named "Config.toml" by default.
        .add_source(File::with_name("Config").required(true))
        .build()?;

    let s: Settings = config.try_deserialize()?;
    println!("AENEAS path: {}", s.programs.aeneas);

    Ok(())
}