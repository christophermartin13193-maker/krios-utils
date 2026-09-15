#![allow(dead_code)]
use crate::exit::SonardError;
use serde::Deserialize;
use std::path::PathBuf;

// Configuration
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub sinks: Option<SinksConfig>,
    pub mic: Option<MicConfig>,
}

#[derive(Deserialize, Debug, Default)]
pub struct SinksConfig {
    pub current: Option<String>,
    pub plugged: Option<String>,
    pub unplugged: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct MicConfig {
    pub open: Option<String>,
    pub muted: Option<String>,
    pub plugged: Option<String>,
    pub unplugged: Option<String>,
}

fn get_path1(username: &str) -> PathBuf {
    let mut path = PathBuf::from("/home");
    path.push(username);
    path.push(".local");
    path.push("sonard");
    path.push("config.toml");
    path
}

fn get_path2(username: &str) -> PathBuf {
    let mut path = PathBuf::from("/home");
    path.push(username);
    path.push(".config");
    path.push("sonard");
    path.push("config.toml");
    path
}

fn parse(path: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn get_config() -> Result<Config, SonardError> {
    let username = whoami::username()?;
    let path1 = get_path1(&username);
    let path2 = get_path2(&username);

    match (parse(path1), parse(path2)) {
        (Ok(config), _) | (_, Ok(config)) => Ok(config),
        _ => Err(SonardError::RetrievingConfig),
    }
}

impl SinksConfig {
    fn valid(self) -> Option<Self> {
        if let None = self.current
            && let None = self.plugged
            && let None = self.unplugged
        {
            None
        } else {
            Some(self)
        }
    }
}

impl MicConfig {
    fn valid(self) -> Option<Self> {
        if let None = self.open
            && let None = self.muted
            && let None = self.plugged
            && let None = self.unplugged
        {
            None
        } else {
            Some(self)
        }
    }
}

impl Config {
    pub fn valid(mut self) -> Result<Self, SonardError> {
        self.sinks = match self.sinks {
            None => None,
            Some(sinks) => sinks.valid(),
        };
        self.mic = match self.mic {
            None => None,
            Some(mic) => mic.valid(),
        };

        if let None = self.sinks
            && let None = self.mic
        {
            Err(SonardError::EmptyConfig)
        } else {
            Ok(self)
        }
    }
}
