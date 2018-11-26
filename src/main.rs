use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const MSG_SIZE: usize = 512;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; MSG_SIZE];

    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let port = 1234;
    let address = "127.0.0.1";
    let address_port = format!("{}:{}", address, port);

    let listener = TcpListener::bind(address_port).expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
