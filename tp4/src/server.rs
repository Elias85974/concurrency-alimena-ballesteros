use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::threadpool::ThreadPool;
use crate::log_handler::LogHandler;
use crate::logs_controller::{StatsRoute, UploadRoute};
use crate::router::Router;

fn handle_request(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    //this prevents the server from crashing when the client sends an empty request
    if stream.read(&mut buffer).is_err() {
        return;
    }


    //Convert the buffer to a string
    let message = String::from_utf8_lossy(&buffer);
    println!("{}",message);
    //Split the message by lines
    let lines: Vec<&str> = message.lines().collect();
    if lines.is_empty() {
        return;
    }

    //Get the first line of the message
    let first_line = lines[0];
    let parts = first_line.split_whitespace().collect::<Vec<_>>();
    //Verify that it is an HTTP call
    if !parts[2].starts_with("HTTP") {
        return;
    }
    //Get the route separated by "/", skipping the first element that is empty
    let route = parts[1].split("/").skip(1).collect::<Vec<_>>();
    println!("Route: {:?}", route);

    match &route[0] {
        _ => {}
    }

    // stream.write_all(response.as_bytes()).unwrap()
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

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| handle_request(stream));
            }
            Err(_) => {
                println!("Connection failed")
            }
        }
    }
    Ok(())
}