use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::SystemTime;
use std::time::{Duration, Instant};

fn main() {
    client(Duration::from_secs(10));
}

fn client(test_duration: Duration) {
    let start_time = Instant::now();
    let end_time = start_time + test_duration;

    // Open files for writing latency values
    let mut file_payload =
        File::create("output/latency_from_payload.txt").expect("Failed to create file_payload");
    let mut file_instant =
        File::create("output/latency_from_instant.txt").expect("Failed to create file_instant");

    // Simulate sending latency packets for the specified duration
    while Instant::now() < end_time {
        // Connect to the server
        let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");

        // Send packet to server
        let packet_start_time = Instant::now();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_nanos();
        // println!("{:?}", timestamp);

        // let packet = format!("{:?}", timestamp).as_bytes();
        let packet = timestamp.to_be_bytes();
        stream
            .write_all(&packet)
            .expect("Failed to send packet to server");

        // thread::sleep(Duration::from_secs(1)); // Simulate network delay

        // Receive packet from server
        let mut buffer = [0; 1024]; // Buffer to store the received data
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from server");
        // let received_packet = &buffer[..bytes_read];
        // println!("Received packet: {:?}", received_packet);

        let packet_end_time = Instant::now();
        let current_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get current timestamp")
            .as_nanos();
        let latency_from_instant = (packet_end_time - packet_start_time).as_nanos();

        // Process the received packet
        // ...
        // let received_packet_str = String::from_utf8_lossy(received_packet);

        let received_timestamp = u128::from_be_bytes(buffer[..bytes_read].try_into().unwrap());
        // println!("\t{}", received_timestamp);

        let latency_from_payload = current_timestamp - received_timestamp;
        // println!("Latency (from payload): {} ns", latency_from_payload);
        // print!("Latency (from payload): {} ns --- ", latency_from_payload);

        // println!(
        //     "Latency (using Instant::now()): {:?}",
        //     latency_from_instant.as_nanos()
        // );

        // Write latency values to files
        writeln!(file_payload, "{}", latency_from_payload)
            .expect("Failed to write to file_payload");
        writeln!(file_instant, "{}", latency_from_instant)
            .expect("Failed to write to file_instant");
    }

    // Close the files
    file_payload.flush().expect("Failed to flush file_payload");
    file_instant.flush().expect("Failed to flush file_instant");
}
