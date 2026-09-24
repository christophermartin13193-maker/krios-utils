mod display;
mod error;
mod translation;

// Project API
pub use error::SonardError;

/// Alias representing an OS exit code.
pub type ExitCode = i32;

/// Standard exit code returned when the application terminates successfully.
/// 
/// Since `sonard` is intended to run as a background daemon, it shouldn't
/// return any exit code during regular operation.
pub const EXIT_CODE_STANDARD: ExitCode = 0;

/// Exit code returned when a fatal error occurs during execution.
pub const EXIT_CODE_ERROR: ExitCode = 1;

/// Represents the directive returned after processing an execution step in `Automate`.
/// 
/// It instructs the main control loop whether to proceed with the next step
/// (`Continue`) or gracefully terminate (`Exit`).
pub enum Output {
    Continue,
    Exit,
}
