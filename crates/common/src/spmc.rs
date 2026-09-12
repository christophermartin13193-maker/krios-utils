#![allow(dead_code)]

use std::sync::{
    Arc, Mutex,
    mpsc::{self, Sender},
};
pub use verbosity::Verbosity;
use worker::Worker;

pub mod error;
pub mod verbosity;
mod worker;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
    size: usize,
}

impl ThreadPool {
    pub fn new(size: usize, v: Verbosity) -> Result<Self, error::ThreadLoopError> {
        if size == 0 {
            return Err(error::ThreadLoopError::SizeSetTo0);
        }

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(v, id, Arc::clone(&receiver)));
        }

        Ok(Self {
            workers,
            sender,
            size,
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
