use crate::internal::SonardError;
use pipewire_native::{
    context::Context, core::Core, main_loop::MainLoop, properties::Properties,
    proxy::registry::Registry,
};

pub struct Wrapper(pub MainLoop, pub Context, pub Core, pub Registry);

impl Wrapper {
    pub fn new() -> Result<Self, SonardError> {
        pipewire_native::init();

        let v = vec![
            (String::from("application.name"), String::from("sonard")),
            (String::from("media.role"), String::from("notification")),
        ];

        let properties = Properties::new_vec(v);
        let main_loop = MainLoop::new(&properties).ok_or(SonardError::MainLoop)?;
        let context = match Context::new(&main_loop, Properties::new()) {
            Ok(context) => context,
            Err(error) => return Err(SonardError::Context(error)),
        };
        let core = match context.connect(Some(properties)) {
            Ok(core) => core,
            Err(error) => return Err(SonardError::Core(error)),
        };
        let registry = match core.registry() {
            Ok(registry) => registry,
            Err(error) => return Err(SonardError::Registry(error)),
        };

        Ok(Self(main_loop, context, core, registry))
    }
}
