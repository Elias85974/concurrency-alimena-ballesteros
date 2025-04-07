use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use crate::threadpool::Job;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} recibió un trabajo; ejecutando.", id);
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
