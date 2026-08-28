use crate::command::{
    arg::Verbosity,
    config::Config,
    output::ErrorType,
    step::{Context, Step},
};
use std::{
    collections::HashMap,
};

fn get_id(
    n: &str,
    sinks: &[String],
    config_sinks: &HashMap<String, String>,
) -> Result<u32, Box<dyn std::error::Error>> {
    let name = n.to_string();
    let real_name = config_sinks.get(&name).unwrap_or(&name);

    for line in sinks {
        if line.contains(real_name) {
            return Ok(line.split_whitespace().next().unwrap().trim().parse()?)
        }
    }

    Err(Box::new(ErrorType::NamedSinkNotFound))
}

pub fn step(context: Context, sinks: Vec<String>) -> Result<Step, Box<dyn std::error::Error>> {
    if context.v == Verbosity::Verbose { println!("---- STEP : NAME ----") }

    let id = get_id(&context.args.name.clone().unwrap(), &sinks, &context.config.sinks)?;
    if context.v == Verbosity::Verbose { println!("Id='{id}' retrieved successfully") }

    Ok(Step::Wpctl(context, id))
}
