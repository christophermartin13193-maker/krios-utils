use super::standard::*;
use crate::internal::SonardError;

pub struct ValidConfiguration {
    pub(super) sinks: Option<SinksConfiguration>,
    pub(super) mic: Option<MicConfiguration>,
}

impl SinksConfiguration {
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

impl MicConfiguration {
    fn valid(self) -> Option<Self> {
        if let None = self.muted
            && let None = self.unmuted
            && let None = self.plugged
            && let None = self.unplugged
        {
            None
        } else {
            Some(self)
        }
    }
}

impl Configuration {
    pub fn get_valid_configuration(self) -> Result<ValidConfiguration, SonardError> {
        let sinks = match self.sinks {
            None => None,
            Some(sinks) => sinks.valid(),
        };
        let mic = match self.mic {
            None => None,
            Some(mic) => mic.valid(),
        };

        if sinks.is_none() && mic.is_none() {
            Err(SonardError::InvalidConfiguration)
        } else {
            Ok(ValidConfiguration { sinks, mic })
        }
    }
}
