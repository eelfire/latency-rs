use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::SystemTime;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::time::sleep;
use tokio::try_join;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: client <ip> <duration_in_seconds>");
        std::process::exit(1);
    }

    let ip = &args[1];
    let duration = args[2].parse::<u64>().expect("Failed to parse duration");

    println!("Connecting to server at {}", ip);
    println!("Sending latency packets for {} seconds", duration);

    client(ip, Duration::from_secs(duration)).await;
}

async fn client(ip: &String, test_duration: Duration) {
    let start_time = Instant::now();
    let end_time = start_time + test_duration;

    // open files for writing latency values
    let mut file_payload =
        File::create("output/latency_from_payload.txt").expect("Failed to create file_payload");

    // connect to the server at the specified IP address and 1155 port
    // let mut stream =
    //     TcpStream::connect(format!("{}:1155", ip)).expect("Failed to connect to server");
    let mut stream = tokio::net::TcpStream::connect(format!("{}:1155", ip))
        .await
        .expect("Failed to connect to server");

    let (mut reader, mut writer) = stream.into_split();

    let mut send_count: u64 = 0;
    let mut recv_count: u64 = 0;

    let send_handle = tokio::spawn(async move {
        while Instant::now() < end_time {
            // println!("Sending packet to server");
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Failed to get timestamp")
                .as_nanos();
            // println!("{:?}", timestamp);

            let packet = &timestamp.to_be_bytes();
            writer
                .write_all(packet)
                .await
                .expect("Failed to send packet to server");
            send_count += 1;
        }
        println!("Sent {} packets", send_count);
    });

    let recv_handle = tokio::spawn(async move {
        let mut buffer = [0; 16];
        // while no response from channel, keep receiving packets
        loop {
            // println!("\tReceiving packet from server");
            // receive packet from server
            let bytes_read = reader
                .read(&mut buffer)
                .await
                .expect("Failed to read from server");
            // println!("Received {} bytes", bytes_read);
            recv_count += 1;

            let current_timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Failed to get timestamp")
                .as_nanos();

            // process the received packet
            // let received_packet_str = String::from_utf8_lossy(received_packet);
            let be_bytes = buffer[..bytes_read].try_into();
            let be_bytes = match be_bytes {
                Ok(bytes) => bytes,
                Err(_) => {
                    eprintln!("Failed to convert to bytes");
                    break;
                }
            };
            let received_timestamp = u128::from_be_bytes(be_bytes);
            // println!("\t{}", received_timestamp);

            let latency_from_payload = current_timestamp - received_timestamp;
            // println!("Latency (from payload): {} ns");

            // write latency values to file
            writeln!(file_payload, "{}", latency_from_payload)
                .expect("Failed to write to file_payload");
        }
        println!("Received {} packets", recv_count);
        file_payload.flush().expect("Failed to flush file_payload");
    });

    try_join!(send_handle, recv_handle).expect("Failed to join threads");

    // close the file latency values file
}
