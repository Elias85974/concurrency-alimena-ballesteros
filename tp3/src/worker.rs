use std::thread;
pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}