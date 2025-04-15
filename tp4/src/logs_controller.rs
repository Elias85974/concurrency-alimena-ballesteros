use std::sync::{Arc};
use crate::grep;
use crate::log_handler::LogHandler;
use crate::router::Route;

pub struct UploadRoute {
    pub log_handler: Arc<LogHandler>,
}

impl UploadRoute {
    pub fn new(log_handler: Arc<LogHandler>) -> Self {
        Self { log_handler }
    }
}

impl Route for UploadRoute {
    fn execute(&self, params: Option<Vec<&str>>, body: Option<&str>) -> String {
        if let Some(file) = body {
            // Esperamos por el permiso del semáforo (limita a 4 uploads)
            let _ = self.log_handler.upload_semaphore.acquire();
            let grep_result = grep::search("exception", file);
            let _ = self.log_handler.upload_semaphore.close();

            // Escribimos en logs bajo exclusión mutua
            let mut logs = self.log_handler.logs.write().unwrap();
            // Ejemplo de escritura
            logs.entry("upload.txt".to_string()).or_insert(grep_result);

            "Upload recibido correctamente".to_string()
        }
        else {
            "No me pasaste un file gordo".to_string()
        }
    }

    fn matches(&self, path: &str) -> bool {
        path == "/upload"
    }
}


pub struct StatsRoute {
    pub log_handler: Arc<LogHandler>,
}

impl StatsRoute {
    pub fn new(log_handler: Arc<LogHandler>) -> Self {
        Self { log_handler }
    }
}

impl Route for StatsRoute {
    fn execute(&self, params: Option<Vec<&str>>, body: Option<&str>) -> String {
        todo!()
    }

    fn matches(&self, path: &str) -> bool {
        path == "/stats"
    }
}
