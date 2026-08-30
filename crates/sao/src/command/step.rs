#![allow(unused)]
use super::{
    arg::{Args, Verbosity},
    config::Config,
};

pub enum Step {
    Init(Args),
    Check(Context),
    Name(Context, Vec<String>),
    Next(Context, Vec<String>),
    Wpctl(Context, u32),
    PlaceHolder,
}

#[derive(Debug)]
pub struct Context {
    pub v: Verbosity,

    pub args: Args,
    pub config: Config,
}
