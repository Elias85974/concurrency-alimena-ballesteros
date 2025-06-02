use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::grep::LineResult;
use tokio::sync::Semaphore;

pub struct LogHandler {
    pub logs: RwLock<HashMap<String, Vec<LineResult>>>,
    pub upload_semaphore: Semaphore,
}

impl LogHandler {
    pub fn new(upload_limit: i64) -> Arc<Self> {
        Arc::new(Self {
            logs: RwLock::new(HashMap::new()),
            upload_semaphore: Semaphore::new(upload_limit as usize),
        })
    }
}
