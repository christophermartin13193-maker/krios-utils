use super::{SonardError, Step};
use crate::internal::get_configuration;

pub fn step() -> Result<Step, SonardError> {
    let configuration = get_configuration()?;
    let valid_configuration = configuration.get_valid_configuration()?;

    let optimized_configuration = valid_configuration.get_optimized_configuration();
    let minimized_configuration = optimized_configuration.get_minimized_configuration();

    Ok(Step::Running(
        optimized_configuration,
        minimized_configuration,
    ))
}
