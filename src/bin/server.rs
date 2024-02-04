use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    server();
}

fn server() {
    // Listen for incoming connections
    let listener = TcpListener::bind("0.0.0.0:1155").expect("Failed to bind to address");

    // Accept connections and handle them
    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");

        println!("Client connected from {}", stream.peer_addr().unwrap());

        let mut buffer = [0; 16]; // Buffer to store the received data
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                println!("Client disconnected");
                break;
            }
            // println!("Received {} bytes", bytes_read);

            // Send packet back to client
            stream
                .write_all(&buffer[..bytes_read])
                .expect("Failed to send packet to client");
        }
    }
}
