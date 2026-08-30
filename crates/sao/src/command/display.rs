use crate::command::{arg::Verbosity, output::ErrorType};
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
            Self::AbsentCommand => write!(f, "The command 'wpctl' cannot be find in the system"),
            Self::Io(err) => write!(f, "Encountered the Io error : {err}"),
            Self::Utf8(err) => write!(f, "Couldn't transform the sinks parsed with 'wpctl' into a vector properly : {err}"),
            Self::NoSinks => write!(f, "No audio sinks were find in the system"),
            Self::NextButNoConfig => write!(f, "Option '--next' enabled but no config file were found"),
            Self::Toml(err) => write!(f, "Couldn't deserialize the config file properly : {err}"),
            Self::Parse(err) => write!(f, "Failed to parse the id of a sink : {err}"),
            Self::NamedSinkNotFound => write!(f, "Couldn't find the the targeted sink"),
            Self::NoValidSinkInConfig => write!(f, "There is no valid sink in the config file"),
        }
    }
}
