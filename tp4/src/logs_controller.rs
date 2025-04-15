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
    async fn execute(&self, params: Vec<&str>, body: &str) -> String {
        // Esperamos por el permiso del semáforo (limita a 4 uploads)
        let _ = self.log_handler.upload_semaphore.acquire().await.unwrap();
        let grep_result = grep::search("exception", body);
        let _ = self.log_handler.upload_semaphore.close().await.unwrap();

        // Escribimos en logs bajo exclusión mutua
        let mut logs = self.log_handler.logs.write().unwrap();
        // Ejemplo de escritura
        logs.entry("upload.txt".to_string())
            .or_default()
            .push(grep_result);

        "Upload recibido correctamente".to_string()
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
    async fn execute(&self, _params: Vec<&str>, _body: &str) -> String {
        todo!()
    }

    fn matches(&self, path: &str) -> bool {
        path == "/stats"
    }
}
