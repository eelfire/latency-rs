use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::SystemTime;
use std::time::{Duration, Instant};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: client <ip> <duration_in_seconds>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let duration = args[2].parse::<u64>().expect("Failed to parse duration");

    println!("Connecting to server at {}", ip);
    println!("Sending latency packets for {} seconds", duration);

    client(ip, Duration::from_secs(duration));
}

fn client(ip: &String, test_duration: Duration) {
    let start_time = Instant::now();
    let end_time = start_time + test_duration;

    // Open files for writing latency values
    let mut file_payload =
        File::create("output/latency_from_payload.txt").expect("Failed to create file_payload");
    let mut file_instant =
        File::create("output/latency_from_instant.txt").expect("Failed to create file_instant");

    // Connect to the server at the specified IP address and 1155 port
    let mut stream =
        TcpStream::connect(format!("{}:1155", ip)).expect("Failed to connect to server");

    let mut count: u64 = 0;

    // Simulate sending latency packets for the specified duration
    while Instant::now() < end_time {
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
        let mut buffer = [0; 16]; // Buffer to store the received data
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from server");
        // println!("Received {} bytes", bytes_read);
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

        count += 1;
    }

    println!("Sent {} packets", count);

    // Close the files
    file_payload.flush().expect("Failed to flush file_payload");
    file_instant.flush().expect("Failed to flush file_instant");
}
