mod minimized;
mod optimized;
mod standard;
mod valid;

// Internal API
use standard::{MicConfiguration, SinksConfiguration};
use valid::ValidConfiguration;

// Project API
pub use minimized::MinimizedConfiguration;
pub use optimized::OptimizedConfiguration;
pub use standard::get_configuration;
