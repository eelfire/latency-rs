use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    server();
}

fn server() {
    // listen for incoming connections
    let listener = TcpListener::bind("0.0.0.0:1155").expect("Failed to bind to address");

    // handle the incoming connections
    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");

        println!("Client connected from {}", stream.peer_addr().unwrap());

        let mut buffer = [0; 16];
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                println!("Client disconnected");
                break;
            }
            // println!("Received {} bytes", bytes_read);

            // send packet back to client without any modification
            stream
                .write_all(&buffer[..bytes_read])
                .expect("Failed to send packet to client");
        }
    }
}
