#![allow(unused)]
use std::cmp::PartialEq;
use std::error::Error;

pub enum Output {
    Exit,
    Continue,
}

#[derive(Debug)]
pub enum ErrorType {
    AbsentCommand,
    NextButNoConfig,
    NoSinks,
    NamedSinkNotFound,
    NoValidSinkInConfig,
    // Other errors
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    Toml(toml::de::Error),
    Parse(std::num::ParseIntError),
}

impl From<std::io::Error> for ErrorType {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for ErrorType {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

impl From<toml::de::Error> for ErrorType {
    fn from(value: toml::de::Error) -> Self {
        Self::Toml(value)
    }
}

impl From<std::num::ParseIntError> for ErrorType {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::Parse(value)
    }
}

impl Error for ErrorType {}
