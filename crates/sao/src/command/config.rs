use serde::Deserialize;
use std::{collections::BTreeMap, path::PathBuf, str::FromStr};

use toml;
use whoami;

use crate::command::output::ErrorType;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sinks: BTreeMap<String, String>,
}

pub fn get_config_from_path(path: &PathBuf) -> Result<Config, ErrorType> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn get_default_config() -> Config {
    let username = whoami::username().unwrap_or("".to_string());
    let path1 = PathBuf::from_str(&format!("/home/{username}/.local/share/sao/config.toml")).unwrap();
    let path2 = PathBuf::from_str(&format!("/home/{username}/.config/sao/config.toml")).unwrap();

    match (get_config_from_path(&path1), get_config_from_path(&path2)) {
        (Ok(config), _) | (_, Ok(config)) => config,
        _ => Config {
            sinks: BTreeMap::new(),
        },
    }
}
