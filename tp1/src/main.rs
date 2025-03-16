use std::net::{TcpListener, TcpStream};

fn handle_request(stream: TcpStream) {
    println!("Connection succeeded")
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_request(stream);
            }
            Err(_) => { println!("Connection failed") }
        }
    }
    Ok(())
}