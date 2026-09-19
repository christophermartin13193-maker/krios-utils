use crate::internal::{
    EXIT_CODE_ERROR, ExitCode, MinimizedConfiguration, OptimizedConfiguration, Output, SonardError,
};

mod init;
mod running;

enum Step {
    Init,
    Running(OptimizedConfiguration, MinimizedConfiguration),
    Exit,
}

pub struct Automate {
    step: Step,
}

impl Automate {
    pub fn new() -> Self {
        Automate { step: Step::Init }
    }

    pub fn run(&mut self) -> Result<Output, SonardError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);

        self.step = match current {
            Step::Init => init::step()?,
            Step::Running(optimized, minimized) => running::step(optimized, minimized)?,
            Step::Exit => return Ok(Output::Exit),
        };

        Ok(Output::Continue)
    }

    pub fn handle_error(error: SonardError) -> ExitCode {
        eprintln!("{error}");
        EXIT_CODE_ERROR
    }
}

impl Default for Automate {
    fn default() -> Self {
        Self::new()
    }
}
