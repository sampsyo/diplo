use serde::Deserialize;
use std::{collections::HashMap, fs, io};

#[derive(Deserialize)]
pub struct Config {
    pub targets: HashMap<String, Target>,
}

#[derive(Deserialize)]
pub struct Target {
    pub key: String,
    pub dest: String,
}

impl Config {
    // Load the configuration.
    pub fn new() -> Result<Self, io::Error> {
        let config_data = fs::read("diplo.toml")?;
        let config: Config = toml::from_slice(&config_data)?;
        Ok(config)
    }
}
