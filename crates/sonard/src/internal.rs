mod icons;

mod automate;
mod configuration;
mod function;
mod output;

// Internal API
use configuration::{MinimizedConfiguration, OptimizedConfiguration, get_configuration};
use function::{GlobalRemoveInformation, Wrapper, get_global, get_remove};
use output::{EXIT_CODE_ERROR, ExitCode, SonardError};

// Project API
pub use automate::Automate;
pub use output::{EXIT_CODE_STANDARD, Output};
