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

    // open files for writing latency values
    let mut file_payload =
        File::create("output/latency_from_payload.txt").expect("Failed to create file_payload");

    // connect to the server at the specified IP address and 1155 port
    let mut stream =
        TcpStream::connect(format!("{}:1155", ip)).expect("Failed to connect to server");

    let mut send_count: u64 = 0;
    let mut recv_count: u64 = 0;

    // sending latency packets for the specified duration
    while Instant::now() < end_time {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_nanos();
        // println!("{:?}", timestamp);

        let packet = &timestamp.to_be_bytes();
        stream
            .write_all(packet)
            .expect("Failed to send packet to server");
        send_count += 1;

        // thread::sleep(Duration::from_secs(1)); // simulate network delay

        // receive packet from server
        let mut buffer = [0; 16];
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from server");
        // println!("Received {} bytes", bytes_read);
        recv_count += 1;

        let current_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_nanos();

        // process the received packet
        // let received_packet_str = String::from_utf8_lossy(received_packet);
        let received_timestamp = u128::from_be_bytes(buffer[..bytes_read].try_into().unwrap());
        // println!("\t{}", received_timestamp);

        let latency_from_payload = current_timestamp - received_timestamp;
        // println!("Latency (from payload): {} ns", latency_from_payload);

        // write latency values to file
        writeln!(file_payload, "{}", latency_from_payload)
            .expect("Failed to write to file_payload");
    }

    println!("Sent {} packets", send_count);
    println!("Received {} packets", recv_count);

    // close the file latency values file
    file_payload.flush().expect("Failed to flush file_payload");
}
