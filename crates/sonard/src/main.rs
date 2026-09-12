use pipewire_native::{
    context::Context, main_loop::MainLoop, properties::Properties, proxy::registry::RegistryEvents,
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod event;
use event::{get_global, get_global_remove};

fn main() {
    pipewire_native::init();

    // Properties
    let v = vec![
        (String::from("application.name"), String::from("ans_daemon")),
        (String::from("media.role"), String::from("Notification")),
    ];

    let props = Properties::new_vec(v);

    let main_loop = MainLoop::new(&Properties::new()).expect("Couldn't create a main loop");
    let context = Context::new(&main_loop, Properties::new()).expect("Couldn't create a context");
    let core = context
        .connect(Some(props))
        .expect("Couldn't connect a core");
    let registry = core.registry().expect("Couldn't get a registry");

    // Event
    let map: HashMap<u32, String> = HashMap::new();
    let arc_map = Arc::new(Mutex::new(map));

    let global = get_global(Arc::clone(&arc_map));
    let global_remove = get_global_remove(Arc::clone(&arc_map));

    let _id = registry.add_listener(RegistryEvents {
        global,
        global_remove,
    });

    main_loop.run();
}
