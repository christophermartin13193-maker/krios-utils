use crate::command::{
    arg::Verbosity::Verbose,
    output::ErrorType,
    step::{Context, Step},
};

pub fn step(context: Context, id: u32) -> Result<Step, ErrorType> {
    if context.v == Verbose {
        println!("---- STEP : WPCTL ----")
    }

    if context.v == Verbose {
        println!("Now modifying default sink")
    }
    std::process::Command::new("wpctl")
        .arg("set-default")
        .arg(format!("{id}"))
        .output()?;
    if context.v == Verbose {
        println!("Done !")
    }

    Ok(Step::Exit)
}
