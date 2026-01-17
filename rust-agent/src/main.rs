use pcap::{Capture, Device};
use etherparse::SlicedPacket;

fn main() {
    println!("PacketPrism :: Day 2 - Final Capture Loop (Read-Only)\n");

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
        .timeout(1000) // timeout is fine — we handle it
        .open()
        .expect("Failed to open capture");

    println!("Capturing packets with safe slicing...\n");

    // 4. Continuous capture loop (CORRECT)
    loop {
        match cap.next_packet() {
            Ok(packet) => {
                let data = packet.data;

                // Safe slicing
                let _ = SlicedPacket::from_ethernet(data);

                println!("Captured packet: {} bytes", data.len());
            }
            Err(pcap::Error::TimeoutExpired) => {
                // No packet in this interval — keep running
                continue;
            }
            Err(e) => {
                eprintln!("Capture error: {:?}", e);
                break;
            }
        }
    }
}
