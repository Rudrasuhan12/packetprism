use pcap::{Capture, Device};

fn main() {
    println!("PacketPrism :: Day 2 - Packet Capture (Read-Only)\n");

    // Get all available devices
    let devices = Device::list().expect("Failed to list network devices");

    // Select the Wi-Fi interface using the FULL device ID
    let device = devices
        .into_iter()
        .find(|d| d.name == r"\Device\NPF_{A26049B8-3D36-4830-A9DE-A4AC8C8755DD}")
        .expect("Selected Wi-Fi device not found");

    println!("Using interface: {}\n", device.name);

    // Open capture in promiscuous mode
    let mut cap = Capture::from_device(device)
        .expect("Failed to create capture from device")
        .promisc(true)
        .snaplen(65535)
        .timeout(1000)
        .open()
        .expect("Failed to open capture");

    println!("Capturing packets... (Press Ctrl+C to stop)\n");

    // Infinite read-only capture loop
    while let Ok(packet) = cap.next_packet() {
        println!("Packet captured: {} bytes", packet.header.len);
    }
}
