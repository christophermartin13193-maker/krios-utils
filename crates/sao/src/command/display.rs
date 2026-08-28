use crate::command::{
    arg::Verbosity,
    output::ErrorType,
};
use std::fmt::Display;

impl Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verbosity::Normal => write!(f, "Verbosity : Normal"),
            Verbosity::Verbose => write!(f, "Verbosity : Verbose"),
            Verbosity::Quiet => write!(f, "Verbosity : Quiet"),
        }
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AbsentCommand => write!(f, "PlaceHolder"),
            _ => writeln!(f, "PlaceHolder"),
        }
    }
}
