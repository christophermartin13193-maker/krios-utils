use super::SonardError;

// impl From<whoami::Error> for SonardError {
//     fn from(value: whoami::Error) -> Self {
//         SonardError::WhoAmIError(value)
//     }
// }

impl From<toml::de::Error> for SonardError {
    fn from(value: toml::de::Error) -> Self {
        SonardError::Toml(value)
    }
}

impl From<std::io::Error> for SonardError {
    fn from(value: std::io::Error) -> Self {
        SonardError::Io(value)
    }
}