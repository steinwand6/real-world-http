use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:18888")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.write_all(
        "HTTP/1.1 200 OK\nContent-Type: text/html; charset=utf-8\n<html><body>hello</body></html>\n"
            .to_string()
            .as_bytes(),
    )?;
    Ok(())
}
