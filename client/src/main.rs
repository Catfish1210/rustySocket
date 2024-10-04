use core::str;
use std::{io, net::TcpStream};
use std::io::{Write, Read};
fn main() {
    println!("Client is running!");
    
    // Connect
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    println!("Connected to server");

    // User Input
    loop {
        let mut input = String::new();
        print!("Enter a message: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        // Send to server
        stream.write_all(input.as_bytes()).unwrap();

        // Read Response 
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).unwrap();
        let response = str::from_utf8(&buffer[..bytes_read]).unwrap();

        println!("Response from server: {}", response);

    }
}
