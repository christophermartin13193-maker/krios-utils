use pipewire_native::{
    context::Context,
    main_loop::MainLoop,
    properties::Properties,
    proxy::registry::{Registry, RegistryEvents},
};

use crate::{
    config::Config,
    exit::SonardError,
    fsm::Step,
    pipe_wrapper::{Global, GlobalRemove},
};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

enum Duty {
    UnpluggedSink,
    UnpluggedMic,
    Hidden,
}

struct GlobalRemoveInfo {
    pub description: String,
    pub icon: String,
    pub duty: Duty,
    pub node_id: Option<u32>,
}

type PipeIndex = Arc<Mutex<HashMap<u32, GlobalRemoveInfo>>>;
// type WrapperShared = Arc<Mutex<Wrapper>>;
type SharedRegistry = Arc<Mutex<Registry>>;

mod global;
mod global_remove;
mod optimized_config;

pub fn step(config: Config) -> Result<Step, SonardError> {
    pipewire_native::init();

    let v = vec![
        (String::from("application.name"), String::from("sonard")),
        (String::from("media.role"), String::from("notification")),
    ];

    let props = Properties::new_vec(v);
    let main_loop = MainLoop::new(&props).ok_or(SonardError::MainLoop)?;
    let context = Context::new(&main_loop, Properties::new())?;
    let core = context.connect(Some(props))?;
    let registry = core.registry()?;

    let map: HashMap<u32, GlobalRemoveInfo> = HashMap::new();
    let p = Arc::new(Mutex::new(map));
    // let w = Arc::new(Mutex::new(wrapper));

    let optimized_config = optimized_config::OptimizedConfig::new(config);

    let global_remove = global_remove::get_global_remove(&optimized_config, Arc::clone(&p));
    // Global
    let s = Arc::new(optimized_config);
    let shared_registry = Arc::new(Mutex::new(registry));
    let global = global::get_global(Arc::clone(&s), Arc::clone(&p), Arc::clone(&shared_registry));

    // Run
    let _id = match shared_registry.lock() {
        Ok(registry) => registry.add_listener(RegistryEvents {
            global,
            global_remove,
        }),
        Err(_error) => return Err(SonardError::PlaceHolder),
    };

    main_loop.run();

    Ok(Step::Exit)
}
