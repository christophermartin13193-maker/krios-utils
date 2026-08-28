use crate::command::{
    arg::Verbosity::Verbose,
    output::ErrorType,
    step::{Context, Step},
};

use std::process::{Command, Stdio};

pub fn get_sinks() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut cmd1 = Command::new("pw-cli").arg("list-objects").arg("Node").stdout(Stdio::piped()).spawn()?;
    let cmd1_stdout = cmd1.stdout.take().ok_or("Couldn't capture stdout of 'pw-cli'")?;
    let cmd2 = Command::new("grep").arg("-E").arg("node.name = ").stdin(cmd1_stdout).output()?;

    let mut sinks: Vec<String> = Vec::new();
    for sink in String::from_utf8(cmd2.stdout.to_vec())?.lines() {
        sinks.push(sink.to_string());
    }

    if sinks.is_empty() { Err(Box::new(ErrorType::NoSinks)) }
    else { Ok(sinks) }
}

pub fn step(context: Context) -> Result<Step, Box<dyn std::error::Error>> {
    if context.v == Verbose {
        println!("---- STEP : WPCTL ----")
    }

    let sinks = get_sinks()?;
    if sinks.is_empty() {
        return Err(Box::new(ErrorType::PlaceHolder));
    }

    for sink in sinks {
        println!("sink")
    }

    Ok(Step::PlaceHolder)
}
