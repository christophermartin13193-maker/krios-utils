use crate::internal::{MinimizedConfiguration, OptimizedConfiguration};
use pipewire_native::proxy::registry::Registry;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod global;
mod information;
mod node;
mod remove;
mod wrapper;

// Internal API
type SharedOptimized = Arc<OptimizedConfiguration>;
type SharedMinimized = Arc<MinimizedConfiguration>;
type SharedRegistry = Arc<Registry>;
type Index = Arc<Mutex<HashMap<u32, GlobalRemoveInformation>>>;

use information::Duty;
use node::get_func;

// Project API
pub use global::get_global;
pub use information::GlobalRemoveInformation;
pub use remove::get_remove;
pub use wrapper::Wrapper;
