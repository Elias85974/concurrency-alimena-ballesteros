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

fn extract_filename(body: &str) -> Option<String> {
    for line in body.lines() {
        if line.starts_with("Content-Disposition") {
            if let Some(start) = line.find("filename=\"") {
                let rest = &line[start + 10..];
                if let Some(end) = rest.find('"') {
                    return Some(rest[..end].to_string());
                }
            }
        }
    }
    None
}

impl Route for UploadRoute {
    fn execute(&self, params: Option<Vec<&str>>, body: Option<&str>) -> (u16, String) {
        if let Some(file) = body {
            if let Some(file_name) = extract_filename(file) {
                println!("File name: {}", file_name);

                // Intentamos adquirir el semáforo sin bloquear
                return if let Ok(_permit) = self.log_handler.upload_semaphore.try_acquire() {
                    let grep_result = grep::search("exception", file);
                    drop(_permit);

                    let mut logs = self.log_handler.logs.write().unwrap();
                    logs.entry(file_name.to_string()).or_insert(grep_result);
                    let response = format!("Processed file: {}", file_name);
                    (200, response)
                } else {
                    // No se pudo adquirir el semáforo: 429 Too Many Requests
                    (429, "Too many files being processed".to_string())
                }
            }
        }
        (400, "File not found or empty".to_string())
    }

    fn path(&self) -> &str {
        "/upload"
    }

    fn description(&self) -> &str {
        "Upload a file for analysis"
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
    fn execute(&self, _params: Option<Vec<&str>>, _body: Option<&str>) -> (u16, String) {
        let logs = self.log_handler.logs.read().unwrap();

        let files_processed = logs.len();
        let total_exceptions: usize = logs.values().map(|v| v.len()).sum();

        // Construimos manualmente el string tipo JSON del mapa
        let mut per_file = String::from("{");
        for (i, (filename, results)) in logs.iter().enumerate() {
            per_file.push('"');
            per_file.push_str(filename);
            per_file.push_str("\": ");
            per_file.push_str(&results.len().to_string());
            if i < logs.len() - 1 {
                per_file.push_str(", ");
            }
        }
        per_file.push('}');

        let response = format!(
            "Total exceptions: {}\nFiles processed: {}\nPer file: {}",
            total_exceptions, files_processed, per_file
        );

        (200, response)
    }

    fn path(&self) -> &str {
        "/stats"
    }

    fn description(&self) -> &str {
        "Show statistics"
    }
}


