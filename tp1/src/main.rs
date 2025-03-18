use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_request(mut stream: TcpStream) {

    let mut buffer = [0; 1024];
    //this prevents the server from crashing when the client sends an empty request
    if stream.read(&mut buffer).is_err() {
        return;
    }

    //Convert the buffer to a string
    let message = String::from_utf8_lossy(&buffer);
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

    let route = parts[1].split("/").skip(1).collect::<Vec<_>>();
    println!("Route: {:?}", route);

    if route[0].eq("pi") {
        let response = handle_pi(route);
        println!("Response: {}", response);
        stream.write_all(response.as_bytes()).unwrap()
    }
}

fn handle_pi(route: Vec<&str>) -> String {
    if route.len() < 2 {
        return format_response(400, "Bad Request: Missing parameter");
    }

    let param = route[1];

    if let Some(num) = param.strip_prefix(":") {
        if let Ok(i) = num.parse::<u64>() {
            let result = calculate_pi(i);
            return format_response(200, &format!(
                "Valor de pi para el termino {}: {}, (Tiempo: {}s)",
                i, result, "0"
            ));
        }
    }

    format_response(400, "Bad Request: Invalid parameter")
}

fn calculate_pi(i: u64) -> f64 {
    let mut pi = 0.0;
    // (-1)^n
    let mut sign = 1.0;

    for n in 0..i {
        //n is u64 so I convert it to f64
        let iteration = sign / ((2 * n + 1) as f64);
        pi += iteration;
        //alternates the sign
        sign *= -1.0
    }
    pi * 4.0
}

fn format_response(status_code: u16, body: &str) -> String {
    format!(
        "HTTP/1.1 {} OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        status_code,
        body.len(),
        body
    )
}

/*
fn deconstruct_route(route: &str) -> Routes {
    let full_request = route.split(":").collect();
    let route = full_request[0]; /lol/lal/sas/
    let params = full_request[1]; pi/ 20
}

struct Routes {
    route: &'_ str,
    params: &'_[str]
}

 */

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3030")?;
    println!("Server listening in http://localhost:3030");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request(stream)
            }
            Err(_) => {
                println!("Connection failed")
            }
        }
    }
    Ok(())
}