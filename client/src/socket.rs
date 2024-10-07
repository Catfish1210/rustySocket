use std::net::TcpStream;
use std::io::{Write, Read};

pub fn connect_socket() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
        stream.write_all("HELLOMSG".as_bytes()).unwrap();
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server..");
    }
}