#![allow(unused)]
use super::{
    arg::{Args, Verbosity},
    output::Output,
    step::Step,
};

use clap::Parser;

mod init;
mod check;
mod name;
mod wpctl;

pub struct Fsm {
    step: Step,
    v: Verbosity,
}

impl Fsm {
    pub fn new() -> Self {
        let args = Args::parse();

        let v = match (args.verbose, args.quiet) {
            (true, _) => Verbosity::Verbose,
            (_, true) => Verbosity::Quiet,
            _ => Verbosity::Normal,
        };

        Self {
            step: Step::Init(args),
            v,
        }
    }

    pub fn next(&mut self) -> Result<Output, Box<dyn std::error::Error>> {
        let current = std::mem::replace(&mut self.step, Step::PlaceHolder);

        self.step = match current {
            Step::Init(args) => init::step(args)?,
            Step::Check(context) => check::step(context)?,
            Step::Name(context,sinks ) => name::step(context, sinks)?,
            Step::Wpctl(context, id) => wpctl::step(context, id)?,
            Step::PlaceHolder => return Ok(Output::Exit),
            _ => Step::PlaceHolder,
        };

        Ok(Output::Continue)
    }
}

impl Default for Fsm {
    fn default() -> Self {
        Self::new()
    }
}
