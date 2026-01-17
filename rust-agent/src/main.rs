use pcap::{Capture, Device};
use etherparse::SlicedPacket;
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize, Debug)]
struct PacketLog {
    src_ip: String,
    dst_ip: String,
    protocol: String,
    length: usize,
    timestamp: String,
}

fn main() {
    println!("PacketPrism :: Day 3 - Packet Contract Serialization\n");

    // 1. List devices
    let devices = Device::list().expect("Failed to list devices");

    // 2. Select default active device (non-loopback, non-WAN)
    let device = devices
        .into_iter()
        .find(|d| {
            !d.name.contains("Loopback")
                && d.desc.as_deref().is_some()
                && !d.desc.as_deref().unwrap().contains("WAN Miniport")
        })
        .expect("No suitable active device found");

    println!("Using default interface: {}\n", device.name);

    // 3. Open capture
    let mut cap = Capture::from_device(device)
        .expect("Failed to create capture")
        .promisc(true)
        .snaplen(65535)
        .timeout(1000)
        .open()
        .expect("Failed to open capture");

    println!("Capturing packets and serializing to JSON...\n");

    // 4. Continuous capture loop
    loop {
        match cap.next_packet() {
            Ok(packet) => {
                let data = packet.data;

                // 5. Safe slicing (no field extraction yet)
                let _ = SlicedPacket::from_ethernet(data);

                // 6. Create PacketLog (mocked IPs & protocol for Day 3)
                let log = PacketLog {
                    src_ip: "0.0.0.0".to_string(),
                    dst_ip: "0.0.0.0".to_string(),
                    protocol: "TCP".to_string(),
                    length: data.len(),
                    timestamp: Utc::now().to_rfc3339(),
                };

                // 7. Serialize to JSON
                let json = serde_json::to_string(&log)
                    .expect("Failed to serialize PacketLog");

                println!("{}", json);
            }
            Err(pcap::Error::TimeoutExpired) => {
                // No packets in this interval — keep running
                continue;
            }
            Err(e) => {
                eprintln!("Capture error: {:?}", e);
                break;
            }
        }
    }
}
