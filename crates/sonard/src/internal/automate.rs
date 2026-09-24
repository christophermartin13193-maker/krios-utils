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

/// The finite state machine (FSM) regulating the lifecycle of the application.
/// 
/// # Example
/// 
/// ```no_run
/// use sonard::{Automate, Output, EXIT_CODE_STANDARD};
/// 
/// let mut automate = Automate::default();
/// loop {
///     match automate.run() {
///         Ok(Output::Continue) => continue,
///         Ok(Output::Exit) => std::process::exit(EXIT_CODE_STANDARD),
///         Err(error) => std::process::exit(Automate::handle_error(error)),
///     }
/// }
/// ```
pub struct Automate {
    step: Step,
}

impl Automate {
    /// Creates a new `Automate` instance initialized at the `Init` step.
    pub fn new() -> Self {
        Automate { step: Step::Init }
    }

    /// Advances the state machine by one step.
    ///
    /// Returns an [`Output`] instruction on success, or a `SonardError` if an execution step fails.
    pub fn run(&mut self) -> Result<Output, SonardError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);

        self.step = match current {
            Step::Init => init::step()?,
            Step::Running(optimized, minimized) => running::step(optimized, minimized)?,
            Step::Exit => return Ok(Output::Exit),
        };

        Ok(Output::Continue)
    }

    /// Prints the given error to `stderr` and returns the error exit code (`EXIT_CODE_ERROR`).
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