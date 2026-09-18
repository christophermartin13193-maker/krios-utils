#![allow(dead_code)]
pub type ExitCode = i32;
pub const NORMAL_EXIT_CODE: i32 = 0;
pub const ERROR_EXIT_CODE: i32 = 1;

pub enum Output {
    Continue,
    Exit,
}

pub enum SonardError {
    RetrievingUsername(whoami::Error),
    RetrievingConfig,
    EmptyConfig,

    MainLoop,
    Io(std::io::Error),
    RetrievingWrapper,
    PlaceHolder,
}

impl std::fmt::Display for SonardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RetrievingUsername(err) => {
                writeln!(f, "Couldn't retrieve username and got the error: {err}")
            }
            Self::RetrievingConfig => writeln!(f, "Couldn't retrieve the configuration file."),
            Self::EmptyConfig => writeln!(f, "Configuration file seems to be empty"),

            Self::MainLoop => writeln!(f, "Couldn't create a main loop for PipeWire"),
            Self::Io(err) => writeln!(
                f,
                "Couldn't create a context, core, or registry for PipeWire, and got the error: {err}"
            ),
            Self::RetrievingWrapper => writeln!(f, "Couldn't retrieve the wrapper of the registry"),

            Self::PlaceHolder => writeln!(f, "This is a placeholder error"),
        }
    }
}

impl From<whoami::Error> for SonardError {
    fn from(value: whoami::Error) -> Self {
        Self::RetrievingUsername(value)
    }
}

impl From<std::io::Error> for SonardError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
