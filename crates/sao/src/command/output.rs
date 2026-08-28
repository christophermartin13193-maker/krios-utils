#![allow(unused)]
use std::cmp::PartialEq;
use std::error::Error;

#[derive(PartialEq)]
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
    PlaceHolder,
}

impl Error for ErrorType {}
