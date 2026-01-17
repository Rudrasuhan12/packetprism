use pcap::{Capture, Device};
use etherparse::SlicedPacket;

fn main() {
    println!("PacketPrism :: Day 2A - Capture Loop with Safe Slicing\n");

    // 1. Automatically select default device (non-loopback)
    let device = Device::list()
        .expect("Failed to list devices")
        .into_iter()
        .find(|d| !d.name.contains("Loopback") && !d.name.contains("loopback"))
        .expect("No suitable default device found");

    println!("Using default interface: {}\n", device.name);

    // 2. Open capture in promiscuous mode
    let mut capture = Capture::from_device(device)
        .expect("Failed to create capture")
        .promisc(true)
        .snaplen(65535)
        .timeout(1000)
        .open()
        .expect("Failed to open capture");

    println!("Capturing packets (safe slicing enabled)...\n");

    // 3. Capture loop (read-only)
    while let Ok(packet) = capture.next_packet() {
        // 4. Safe slice using etherparse (no extraction yet)
        let _ = SlicedPacket::from_ethernet(packet.data);

        // 5. Print packet size only
        println!("Captured packet: {} bytes", packet.header.len);
    }
}
