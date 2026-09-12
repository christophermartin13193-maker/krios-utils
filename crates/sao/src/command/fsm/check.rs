use crate::command::{
    arg::Verbosity,
    output::ErrorType,
    step::{Context, Step},
};

fn check_cmd() -> Result<(), ErrorType> {
    if std::process::Command::new("which")
        .arg("wpctl")
        .output()?
        .status
        .success()
    {
        Ok(())
    } else {
        Err(ErrorType::AbsentCommand)
    }
}

fn get_sinks() -> Result<Vec<String>, ErrorType> {
    let cmd = std::process::Command::new("wpctl")
        .arg("list")
        .arg("audio")
        .arg("sinks")
        .output()?;
    let mut sinks: Vec<String> = Vec::new();
    for sink in String::from_utf8(cmd.stdout.to_vec())?.lines() {
        sinks.push(sink.to_string());
    }

    if sinks.is_empty() {
        Err(ErrorType::NoSinks)
    } else {
        Ok(sinks)
    }
}

pub fn step(context: Context) -> Result<Step, ErrorType> {
    if context.v == Verbosity::Verbose {
        println!("---- STEP : CHECK ----")
    }

    if context.v == Verbosity::Verbose {
        println!("Checking whether the system has the command 'wpctl' or not")
    }
    check_cmd()?;

    if context.args.next && context.config.sinks.is_empty() {
        return Err(ErrorType::NextButNoConfig);
    }

    if context.v == Verbosity::Verbose {
        println!("Checking the actual sinks of the system")
    }
    let sinks = get_sinks()?;
    if context.v == Verbosity::Verbose {
        for sink in &sinks {
            println!("Sink found : {sink}")
        }
    }

    if context.args.next {
        Ok(Step::Next(context, sinks))
    } else {
        Ok(Step::Name(context, sinks))
    }
}
