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
    dirs::data_local_dir().map(|mut p| {
        p.push("sonard");
        p.push("config.toml");
        p
    })
}

fn get_path_2() -> Option<PathBuf> {
    dirs::config_dir().map(|mut p| {
        p.push("sonard");
        p.push("config.toml");
        p
    })
}

fn parse(path: PathBuf) -> Result<Configuration, SonardError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn get_configuration() -> Result<Configuration, SonardError> {
    let paths = [get_path_1(), get_path_2()];

    for path in paths {
        if let Some(path) = path && path.is_file() {
            return parse(path);
        }
    }
    Err(SonardError::NoConfigFile)
}
