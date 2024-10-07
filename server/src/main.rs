use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;

            }
            Ok(bytes_read) => {
                let received = str::from_utf8(&buffer[..bytes_read]).unwrap();
                println!("Received: {}", received);
                
                // Ping-pong the msg back to client
                stream.write_all(received.as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }


    }

}

fn main() {
    println!("Server is running!");
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_client(stream);
        });
        // match stream {
        //     Ok(stream) => {
        //         handle_client(stream);
        //         println!("New TCP connection estabished!");
        //     }
        //     Err(e) => {
        //         eprintln!("Failed to establish new connection: {}", e);
        //     }
        // }
    }
}
