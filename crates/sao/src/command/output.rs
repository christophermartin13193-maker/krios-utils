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
    PlaceHolder,
    // Other error
    Io(std::io::Error)
}

impl From<std::io::Error> for ErrorType {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl Error for ErrorType {}
