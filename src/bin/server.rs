use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    server();
}

fn server() {
    // Listen for incoming connections
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    // Accept connections and handle them
    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");

        // Receive packet from client
        let mut buffer = [0; 1024]; // Buffer to store the received data
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from client");
        let packet = &buffer[..bytes_read];

        // Process the received packet
        // ...

        // let timestamp = Instant::now();

        // Mark packet with timestamp
        // ...

        // Send packet back to client
        stream
            .write_all(packet)
            .expect("Failed to send packet to client");
    }
}
