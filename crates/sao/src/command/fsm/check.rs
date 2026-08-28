use crate::command::{arg::Verbosity, config::Config, output::ErrorType, step::{Context, Step}};
use std::process::{Command, Stdio};

fn check_cmd() -> Result<(), Box<dyn std::error::Error>> {
    if std::process::Command::new("which").arg("wpctl").output()?.status.success() {
        Ok(())
    }
    else {
        Err(Box::new(ErrorType::AbsentCommand))
    }
}

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
    if context.v == Verbosity::Verbose { println!("---- STEP : CHECK ----") }

    if context.v == Verbosity::Verbose { println!("Checking whether the system has the command 'wpctl' or not") }
    check_cmd()?;

    if context.args.next && context.config.sinks.is_empty() {
        return Err(Box::new(ErrorType::NextButNoConfig))
    }

    if context.v == Verbosity::Verbose { println!("Checking the actual sinks of the system") }
    let sinks = get_sinks()?;
    if context.v == Verbosity::Verbose {
        for sink in &sinks {
            println!("Sink found : {sink}")
        }
    }

    Ok(Step::PlaceHolder)
}