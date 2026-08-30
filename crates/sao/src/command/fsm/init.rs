use crate::command::{
    arg::{Args, Verbosity},
    config,
    output::ErrorType,
    step::{Context, Step},
};

pub fn step(args: Args) -> Result<Step, ErrorType> {
    let v = match (args.verbose, args.quiet) {
        (true, _) => Verbosity::Verbose,
        (_, true) => Verbosity::Quiet,
        _ => Verbosity::Normal,
    };

    if v == Verbosity::Verbose {
        println!("---- STEP : INIT ----")
    };

    let config = match &args.path {
        Some(path) => config::get_config_from_path(path)?,
        None => config::get_default_config(),
    };

    let context = Context { v, args, config };

    Ok(Step::Check(context))
}
