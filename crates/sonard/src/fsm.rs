#![allow(dead_code)]
use crate::{
    config::Config,
    exit::{ERROR_EXIT_CODE, ExitCode, Output, SonardError},
    pipe_wrapper::Wrapper,
};

mod init;
mod looping;

enum Step {
    Init,
    Looping(Config),
    Exit,
}

pub struct Fsm {
    step: Step,
}

impl Fsm {
    pub fn new() -> Self {
        Fsm { step: Step::Init }
    }

    pub fn next_step(&mut self) -> Result<Output, SonardError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);

        self.step = match current {
            Step::Init => init::step()?,
            Step::Looping(config) => looping::step(config)?,
            Step::Exit => return Ok(Output::Exit),
        };

        Ok(Output::Continue)
    }

    pub fn handle_error(&self, err: SonardError) -> ExitCode {
        eprintln!("{err}");
        ERROR_EXIT_CODE
    }
}

impl Default for Fsm {
    fn default() -> Self {
        Self::new()
    }
}
