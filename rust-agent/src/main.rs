use pcap::Device;

fn main() {
    println!("PacketPrism :: Day 1 - Interface Visibility Check\n");

    match Device::list() {
        Ok(devices) => {
            if devices.is_empty() {
                println!("No network interfaces found.");
                return;
            }

            println!("Available Network Interfaces:");
            for (index, device) in devices.iter().enumerate() {
                let desc = device
                    .desc
                    .as_deref()
                    .unwrap_or("No description available");

                println!(
                    "{}. Name: {:<40} | Description: {}",
                    index + 1,
                    device.name,
                    desc
                );
            }
        }
        Err(err) => {
            eprintln!("Failed to list network interfaces: {}", err);
        }
    }
}
