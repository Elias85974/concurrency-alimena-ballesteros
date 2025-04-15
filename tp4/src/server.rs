use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::threadpool::ThreadPool;
use crate::log_handler::LogHandler;
use crate::logs_controller::{StatsRoute, UploadRoute};
use crate::router::Router;

fn handle_request(mut stream: TcpStream, router: Arc<Router>) {
    let mut buffer = Vec::new();
    let mut temp_buf = [0; 1024];

    // Read from stream until headers are fully received (\r\n\r\n marks header-body boundary)
    while let Ok(n) = stream.read(&mut temp_buf) {
        if n == 0 {
            return;
        }

        buffer.extend_from_slice(&temp_buf[..n]);

        // Check if we've reached the end of the headers
        if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    // Convert headers to string for parsing
    let headers = String::from_utf8_lossy(&buffer);

    // Extract Content-Length from headers to know how many bytes of body to read
    let content_length = headers
        .lines()
        .find(|line| line.to_lowercase().starts_with("content-length"))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(0);

    // Find where headers end and body begins
    let header_end_index = buffer.windows(4).position(|w| w == b"\r\n\r\n");
    if header_end_index.is_none() {
        return; // Invalid request, no header-body boundary
    }
    let split_at = header_end_index.unwrap() + 4;
    let mut body_bytes = buffer[split_at..].to_vec();

    // If the body isn't fully received yet, continue reading from the stream
    while body_bytes.len() < content_length {
        let n = stream.read(&mut temp_buf).unwrap_or(0);
        if n == 0 {
            break;
        }
        body_bytes.extend_from_slice(&temp_buf[..n]);
    }

    // Convert full body to a string
    let body = String::from_utf8_lossy(&body_bytes).to_string();

    // Now, parse the request line (e.g., "POST /upload HTTP/1.1")
    let request_line = headers.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 3 || !parts[2].starts_with("HTTP") {
        return; // Invalid request line
    }

    let method = parts[0];
    let route_with_params = parts[1];
    let mut route_parts = route_with_params.split(':');
    let route = route_parts.next().unwrap_or("");
    let params: Vec<&str> = route_parts.collect();

    // Execute the corresponding route with method, path, params and the full body
    let params_option = if params.is_empty() { None } else { Some(params) };
    let body_option = if body.is_empty() { None } else { Some(body.as_str()) };
    if let Some(response) = router.execute_route(method, route, params_option, body_option) {
        stream.write_all(format_response(response.0, response.1.as_str()).as_bytes()).unwrap()
    }
    else {
        stream.write_all(format_response(500, "Algo falló").as_bytes()).unwrap()
    }
}

fn format_response(status_code: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {} OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        status_code,
        body.len(),
        body
    )
}

pub fn execute() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3030")?;
    println!("Server listening in http://localhost:3030");
    let pool = ThreadPool::new(8);
    let log_handler = Arc::new(LogHandler::new(4));

    // Inicializar las rutas
    let upload_route = UploadRoute::new(Arc::clone(&log_handler));
    let stats_route = StatsRoute::new(Arc::clone(&log_handler));

    let mut router = Router::new();
    router.add_route("POST", Box::new(upload_route));
    router.add_route("GET", Box::new(stats_route));

    let shared_router = Arc::new(router);


    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let shared_router = Arc::clone(&shared_router); // clone Arc pointer (not the inner value)
                pool.execute(|| handle_request(stream, shared_router));
            }
            Err(_) => {
                println!("Connection failed")
            }
        }
    }
    Ok(())
}