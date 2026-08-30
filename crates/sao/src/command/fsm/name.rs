use crate::command::{
    arg::Verbosity,
    output::ErrorType,
    step::{Context, Step},
};
use std::collections::BTreeMap;

fn get_id(
    n: &str,
    sinks: &[String],
    config_sinks: &BTreeMap<String, String>,
) -> Result<u32, ErrorType> {
    let name = n.to_string();
    let real_name = config_sinks.get(&name).unwrap_or(&name);

    for line in sinks {
        if line.contains(real_name) {
            return Ok(line.split_whitespace().next().unwrap().trim().parse()?)
        }
    }

    Err(ErrorType::NamedSinkNotFound)
}

pub fn step(context: Context, sinks: Vec<String>) -> Result<Step, ErrorType> {
    if context.v == Verbosity::Verbose { println!("---- STEP : NAME ----") }

    let id = get_id(&context.args.name.clone().unwrap(), &sinks, &context.config.sinks)?;
    if context.v == Verbosity::Verbose { println!("Id='{id}' retrieved successfully") }

    Ok(Step::Wpctl(context, id))
}
