#![allow(unused)]

use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use toml;
use whoami;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sinks: HashMap<String, String>,
}

pub fn get_config_from_path(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn get_default_config() -> Config {
    let username = whoami::username().unwrap_or("".to_string());
    let path1 = PathBuf::from_str(&format!("/home/{username}/.local/sao/config.toml")).unwrap();
    let path2 = PathBuf::from_str(&format!("/home/{username}/.config/sao/config.toml")).unwrap();

    match (get_config_from_path(&path1), get_config_from_path(&path2)) {
        (Ok(config), _) => config,
        (_, Ok(config)) => config,
        _ => Config {
            sinks: HashMap::new(),
        },
    }
}
