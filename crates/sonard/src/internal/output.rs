mod display;
mod error;
mod translation;

// Project API
pub use error::SonardError;
pub type ExitCode = i32;
pub const EXIT_CODE_STANDARD: i32 = 0;
pub const EXIT_CODE_ERROR: i32 = 1;

pub enum Output {
    Continue,
    Exit,
}
