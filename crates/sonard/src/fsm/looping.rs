use crate::{
    config::Config,
    exit::SonardError,
    fsm::Step,
    pipe_wrapper::{Global, GlobalRemove, Wrapper},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

enum Duty {
    UnpluggedSink,
    UnpluggedMic,
}

struct GlobalRemoveInfo {
    pub description: String,
    pub icon: String,
    pub duty: Duty,
}

type PipeIndex = Arc<Mutex<HashMap<u32, GlobalRemoveInfo>>>;

mod global;
mod global_remove;
mod optimized_config;

pub fn step(mut wrapper: Wrapper, config: Config) -> Result<Step, SonardError> {
    // Creating PipeIndex
    let map: HashMap<u32, GlobalRemoveInfo> = HashMap::new();
    let p = Arc::new(Mutex::new(map));
    // Creating Optimized config
    let optimized_config = optimized_config::OptimizedConfig::new(config);
    // Global_remove
    let global_remove = global_remove::get_global_remove(&optimized_config, Arc::clone(&p));
    // Global
    let s = Arc::new(optimized_config);
    let global = global::get_global(Arc::clone(&s), Arc::clone(&p));
    // Add listener
    let _id = wrapper.add_listener(global, global_remove);
    // Run loop
    wrapper.run();

    Ok(Step::Exit)
}
