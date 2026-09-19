use pipewire_native::proxy::registry::RegistryEvents;

use super::{MinimizedConfiguration, OptimizedConfiguration, SonardError, Step};
use crate::internal::{GlobalRemoveInformation, Wrapper, get_global, get_remove};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub fn step(
    optimized: OptimizedConfiguration,
    minimized: MinimizedConfiguration,
) -> Result<Step, SonardError> {
    let Wrapper(main_loop, _context, _core, registry) = Wrapper::new()?;

    let index: HashMap<u32, GlobalRemoveInformation> = HashMap::new();
    let index = Arc::new(Mutex::new(index));

    let remove = get_remove(Arc::clone(&index), &optimized);

    let optimized = Arc::new(optimized);
    let minimized = Arc::new(minimized);
    let registry = Arc::new(registry);

    let global = get_global(
        Arc::clone(&optimized),
        Arc::clone(&minimized),
        Arc::clone(&index),
        Arc::clone(&registry),
    );

    registry.add_listener(RegistryEvents {
        global: global,
        global_remove: remove,
    });

    main_loop.run();

    Ok(Step::Exit)
}
