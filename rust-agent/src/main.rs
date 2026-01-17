use pcap::{Capture, Device};
use etherparse::SlicedPacket;
use serde::Serialize;
use chrono::Utc;
use redis::{Commands, Connection};

#[derive(Serialize)]
struct PacketLog {
    src_ip: String,
    dst_ip: String,
    protocol: String,
    length: usize,
    timestamp: String,
}

// ---- Redis connection helper ----
fn connect_redis() -> Option<Connection> {
    match redis::Client::open("redis://127.0.0.1:6379") {
        Ok(client) => match client.get_connection() {
            Ok(con) => {
                println!("Connected to Redis");
                Some(con)
            }
            Err(e) => {
                eprintln!("Redis connection error: {:?}", e);
                None
            }
        },
        Err(e) => {
            eprintln!("Redis client error: {:?}", e);
            None
        }
    }
}

fn main() {
    println!("PacketPrism :: Day 4 - Redis Transport Layer\n");

    // 1. Device selection
    let device = Device::list()
        .expect("Failed to list devices")
        .into_iter()
        .find(|d| {
            !d.name.contains("Loopback")
                && d.desc.as_deref().is_some()
                && !d.desc.as_deref().unwrap().contains("WAN Miniport")
        })
        .expect("No suitable device found");

    println!("Using interface: {}\n", device.name);

    // 2. Open capture
    let mut cap = Capture::from_device(device)
        .expect("Failed to create capture")
        .promisc(true)
        .snaplen(65535)
        .timeout(1000)
        .open()
        .expect("Failed to open capture");

    // 3. Redis connection (outside loop)
    let mut redis_con = connect_redis();

    println!("Publishing packets to Redis channel: packet_stream\n");

    // 4. Continuous capture loop
    loop {
        match cap.next_packet() {
            Ok(packet) => {
                let data = packet.data;

                // Safe slicing (no extraction yet — Day 5)
                let _ = SlicedPacket::from_ethernet(data);

                let log = PacketLog {
                    src_ip: "0.0.0.0".to_string(),
                    dst_ip: "0.0.0.0".to_string(),
                    protocol: "TCP".to_string(),
                    length: data.len(),
                    timestamp: Utc::now().to_rfc3339(),
                };

                // Serialize + publish
                if let Ok(json) = serde_json::to_string(&log) {
                    if let Some(con) = redis_con.as_mut() {
                        let publish_result: redis::RedisResult<()> =
                            con.publish("packet_stream", json);

                        if publish_result.is_err() {
                            eprintln!("Redis publish failed. Retrying connection...");
                            redis_con = connect_redis();
                        }
                    } else {
                        // Redis was unavailable earlier, retry connection
                        redis_con = connect_redis();
                    }
                }
            }

            Err(pcap::Error::TimeoutExpired) => {
                // No packet in this window, keep running
                continue;
            }

            Err(e) => {
                eprintln!("Capture error: {:?}", e);
                break;
            }
        }
    }
}
