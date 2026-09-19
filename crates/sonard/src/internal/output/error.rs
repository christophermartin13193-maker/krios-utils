pub enum SonardError {
    NoConfigFile,
    Toml(toml::de::Error),
    Io(std::io::Error),
    RetrievingConfiguration,
    InvalidConfiguration,
    MainLoop,
    Context(std::io::Error),
    Core(std::io::Error),
    Registry(std::io::Error),
    Bind,
}
