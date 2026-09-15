use super::Step;
use crate::{config::*, exit::SonardError, pipe_wrapper::Wrapper};

pub fn step() -> Result<Step, SonardError> {
    // get config
    let config = get_config()?;
    let config = config.valid()?;

    // get wrapper
    let v = vec![
        (String::from("application.name"), String::from("sonard")),
        (String::from("media.role"), String::from("notification")),
    ];
    let wrapper = Wrapper::new(v)?;

    Ok(Step::Looping(wrapper, config))
}
