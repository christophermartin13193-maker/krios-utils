//! # Sonard
//! 
//! `sonard` is a lightweight Linux daemon built on top of [`pipewire-native`](https://docs.rs/pipewire-native) and [`pipewire-native-spa`](https://docs.rs/pipewire-native-spa) .
//! It monitors the state of sound cards and microphones (activation, mute, plug, unplug) in real time
//! and emits notification instructions.
//! 
//! ## Architecture
//! 
//! The core of the application relies on [`Automate`], a finite state machine (FSM)
//! which orchestrates:
//! 
//! 1. Resolution and loading of XDG configuration files.
//! 2. Connection to the PipeWire event loop via a C-API wrapper.
//! 3. An execution loop running without unnecessary heap allocations.
//! 
//! ## Integration
//! 
//! ```no_run
//! use sonard::{Automate, Output, EXIT_CODE_STANDARD};
//! 
//! let mut automate = Automate::default();
//! loop {
//!     match automate.run() {
//!         Ok(Output::Continue) => continue,
//!         Ok(Output::Exit) => std::process::exit(EXIT_CODE_STANDARD),
//!         Err(error) => std::process::exit(Automate::handle_error(error)),
//!     }
//! }
//! ```

mod internal;

// API
pub use internal::Automate;
pub use internal::EXIT_CODE_STANDARD;
pub use internal::Output;
