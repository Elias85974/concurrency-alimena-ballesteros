use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_request(mut stream: TcpStream) {
    let buffer: &mut [u8] = &mut [0;100];
    stream.read(buffer).unwrap();
    // The http message
    let message = String::from_utf8(Vec::from(buffer)).unwrap();
    let route = message.split(" ").take(2).collect::<Vec<_>>()[1];
    println!("Message: {:?}", message);
    println!("Route: {:?}", route);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request(stream)
            }
            Err(_) => { println!("Connection failed") }
        }
    }
    Ok(())
}