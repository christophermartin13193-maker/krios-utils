impl std::fmt::Display for super::SonardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoConfigFile => writeln!(f, "Unable to find a configuration file"),
            Self::Toml(error) => writeln!(f, "Encountered an error during the parsing of the configuration file: {error}"),
            Self::Io(error) => writeln!(f, "Encountered an error trying to retrieve the configuration file's informations: {error}"),
            Self::RetrievingConfiguration => 
                writeln!(f, "Unable to retrieve the configuration file"),
            Self::InvalidConfiguration => writeln!(f, "Invalid configuration file"),
            Self::MainLoop =>
                writeln!(f, "Couldn't create MainLoop structure from pipewire_native"),
            Self::Context(error) => writeln!(
                f,
                "Couldn't create Context structure from pipewire_native and got the error: {error}"
            ),
            Self::Core(error) => writeln!(
                f,
                "Couldn't connect the Context structure from pipewire_native and got the error: {error}"
            ),
            Self::Registry(error) => writeln!(
                f,
                "Couldn't create Registry structure from pipewire_native and got the error: {error}"
            ),
            Self::Bind => writeln!(f, "Couldn't bind the node to the registry"),
        }
    }
}
