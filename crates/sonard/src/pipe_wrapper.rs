#![allow(dead_code)]
use crate::exit::SonardError;
use pipewire_native::{
    context::Context,
    core::Core,
    main_loop::MainLoop,
    permission::PermissionBits,
    properties::Properties,
    proxy::{
        HasProxy,
        registry::{Registry, RegistryEvents},
    },
};

pub type Global =
    Option<Box<dyn FnMut(u32, PermissionBits, &str, u32, &Properties) + Send + 'static>>;
pub type GlobalRemove = Option<Box<dyn FnMut(u32) + Send + 'static>>;

pub struct Wrapper {
    main_loop: MainLoop,
    context: Context,
    core: Core,
    registry: Registry,
}

impl Wrapper {
    pub fn new(v: Vec<(String, String)>) -> Result<Self, SonardError> {
        pipewire_native::init();

        let props = Properties::new_vec(v);
        let main_loop = MainLoop::new(&props).ok_or(SonardError::MainLoop)?;
        let context = Context::new(&main_loop, Properties::new())?;
        let core = context.connect(Some(props))?;
        let registry = core.registry()?;

        Ok(Self {
            main_loop,
            context,
            core,
            registry,
        })
    }

    pub fn add_listener(&mut self, global: Global, global_remove: GlobalRemove) -> u32 {
        self.registry.add_listener(RegistryEvents {
            global,
            global_remove,
        })
    }

    pub fn bind(
        &mut self,
        id: u32,
        type_: &str,
        version: u32,
    ) -> Result<Box<dyn HasProxy>, std::io::Error> {
        self.registry.bind(id, type_, version)
    }

    pub fn run(&self) {
        self.main_loop.run()
    }
}
