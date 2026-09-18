use super::Step;
use crate::{config::*, exit::SonardError};

pub fn step() -> Result<Step, SonardError> {
    let config = get_config()?;
    let config = config.valid()?;

    Ok(Step::Looping(config))
}
