use crate::command::{
    arg::Verbosity,
    config::Config,
    output::ErrorType,
    step::{Context, Step},
};
use std::collections::BTreeMap;

fn get_id(sinks: &[String], config_sinks: &BTreeMap<String, String>) -> Result<u32, Box<dyn std::error::Error>> {
    let mut first_sink_id: Option<u32> = None;
    let mut current_found = false;

    for (_, value) in config_sinks {
        for line in sinks {
            if line.contains(value) {
                let mut split = line.split_whitespace();
                let id: u32 = split.next().unwrap().trim().parse()?;

                if current_found {
                    return Ok(id)
                }
                if first_sink_id.is_none() {
                    first_sink_id = Some(id)
                }

                let _ = split.next();
                let _ = split.next();
                if split.next().is_some() {
                    current_found = true
                }
            }
        }

    }

    if let Some(id) = first_sink_id {
        Ok(id)
    }
    else {
        Err(Box::new(ErrorType::NoValidSinkInConfig))
    }
}

pub fn step(context: Context, sinks: Vec<String>) -> Result<Step, Box<dyn std::error::Error>> {
    if context.v == Verbosity::Verbose { println!("---- STEP : NEXT ----") }

    if context.v == Verbosity::Verbose { println!("Searching for the right id") }
    let id = get_id(&sinks, &context.config.sinks)?;
    if context.v == Verbosity::Verbose { println!("Id='{id}'") }

    Ok(Step::Wpctl(context, id))
}