use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Instant;

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
    //Get the route separated by "/", skipping the first element that is empty
    let route = parts[1].split("/").skip(1).collect::<Vec<_>>();
    println!("Route: {:?}", route);

    //Check if the route is pi
    if route[0].eq("pi") {
        //Proceed to calculate pi
        let response = handle_pi(route);
        println!("Response: {}", response);
        //Send the response to the client
        stream.write_all(response.as_bytes()).unwrap()
    }
}


fn handle_pi(route: Vec<&str>) -> String {
    //Verifies that a param is given
    if route.len() < 2 {
        return format_response(400, "Bad Request: Missing parameter");
    }

    let param = route[1];

    if let Some(num) = param.strip_prefix(":") {
        //Check if the parameter is a number
        if let Ok(i) = num.parse::<u64>() {
            let start = Instant::now();
            let result = calculate_pi(i);
            let finish: f64 = start.elapsed().as_millis() as f64 / 1000.0;
            return format_response(200, &format!(
                "Valor de pi para el termino {}: {}, (Tiempo: {}s)",
                i, result, finish
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