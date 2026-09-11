use std::{ thread::JoinHandle, sync::{Arc, Mutex, mpsc} };

use super::{verbosity::Verbosity, Job};

pub struct Worker {
    id: usize,
    thread : JoinHandle<()>,
}

impl Worker {
    pub fn new(v: Verbosity, id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        if v == Verbosity::Verbose {
            println!("Worker {id} created")
        }

        let thread = std::thread::spawn(move || { 
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                if v == Verbosity::Verbose {
                    println!("Worker {id} got a job; executing");
                }

                job();
            }
         });

         Worker { id, thread }
    }
}