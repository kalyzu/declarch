use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    pkgs: Vec<String>,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let conf_path = "test-data/config.toml";
    let toml_data = fs::read_to_string(conf_path)?;
    let config: Config = toml::from_str(&toml_data)?;

    println!("{:?}", config.pkgs);
    Ok(())
}
