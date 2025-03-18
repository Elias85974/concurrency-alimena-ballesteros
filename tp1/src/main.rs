use std::io::Read;
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

    // let buffer: &mut [u8] = &mut [0;100];
    // stream.read(buffer).unwrap();
    // // The http message
    // let message = String::fromutf8_lossy(Vec::from(buffer)).unwrap();
    // let route = message.split(" ").take(2).collect::<Vec<>>()[1];
    // let parameters = &route.split("/").collect::<Vec<_>>()[1..];
    // println!("Message: {:?}", message);
    // println!("Route: {:?}", route);
    // println!("Parameters: {:?}", parameters)
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