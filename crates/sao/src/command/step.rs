#![allow(unused)]
use super::{
    arg::{Args, Verbosity},
    config::Config,
};

pub enum Step {
    Init(Args),
    Check(Context), // Check cmd, check sinks, check --next+config.sinks, check name+sinks,
    Wpctl(Context),
    PlaceHolder,
}

#[derive(Debug)]
pub struct Context {
    pub v: Verbosity,

    pub args: Args,
    pub config: Config,
}
