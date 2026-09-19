use crate::internal::SonardError;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize, Default)]
pub struct Configuration {
    pub(super) sinks: Option<SinksConfiguration>,
    pub(super) mic: Option<MicConfiguration>,
}

#[derive(Deserialize, Default)]
pub struct SinksConfiguration {
    pub(super) current: Option<String>,
    pub(super) plugged: Option<String>,
    pub(super) unplugged: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct MicConfiguration {
    pub(super) unmuted: Option<String>,
    pub(super) muted: Option<String>,
    pub(super) plugged: Option<String>,
    pub(super) unplugged: Option<String>,
}

fn get_path_1() -> Option<PathBuf> {
    if let Some(mut path) = dirs::config_local_dir() {
        path.push("sonard");
        path.push("config.toml");
        Some(path)
    } else {
        None
    }
}

fn get_path_2() -> Option<PathBuf> {
    if let Some(mut path) = dirs::config_dir() {
        path.push("sonard");
        path.push("config.toml");
        Some(path)
    } else {
        None
    }
}

fn parse(path: PathBuf) -> Result<Configuration, SonardError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn get_configuration() -> Result<Configuration, SonardError> {
    let path_1 = get_path_1();
    let path_2 = get_path_2();

    match (path_1, path_2) {
        (Some(path), _) | (_, Some(path)) => parse(path),
        _ => Err(SonardError::NoConfigFile),
    }
}
