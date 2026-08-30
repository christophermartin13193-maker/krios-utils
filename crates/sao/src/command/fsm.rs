#![allow(unused)]
use crate::command::output::ErrorType;

use super::{
    arg::{Args, Verbosity},
    output::Output,
    step::Step,
};

use clap::Parser;

mod init;
mod check;
mod name;
mod next;
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

    pub fn next_step(&mut self) -> Result<Output, ErrorType> {
        let current = std::mem::replace(&mut self.step, Step::PlaceHolder);

        self.step = match current {
            Step::Init(args) => init::step(args)?,
            Step::Check(context) => check::step(context)?,
            Step::Name(context, sinks ) => name::step(context, sinks)?,
            Step::Next(context, sinks ) => next::step(context, sinks)?,
            Step::Wpctl(context, id) => wpctl::step(context, id)?,
            Step::PlaceHolder => return Ok(Output::Exit),
        };

        Ok(Output::Continue)
    }

    pub fn handle_error(&self, err: ErrorType) -> i32 {
        if self.v != Verbosity::Quiet {
            eprintln!("{err}")
        }

        match err {
            ErrorType::NextButNoConfig | ErrorType::Toml(_) | ErrorType::NamedSinkNotFound | ErrorType::NoValidSinkInConfig => 1,
            ErrorType::NoSinks | ErrorType::AbsentCommand | ErrorType::Io(_) | ErrorType::Parse(_) | ErrorType::Utf8(_) => 2, 
        }
    }
}

impl Default for Fsm {
    fn default() -> Self {
        Self::new()
    }
}
