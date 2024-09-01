use std::io::Write;

use anyhow::anyhow;
use chrono::Local;
use pcap::Device;

pub fn handle_packet(packet: pcap::Packet) {
    let now = Local::now();
    println!("{}, len: {}", now.format("%H:%M:%S%.6f"), packet.header.len);
}

pub fn dump_packet() -> Result<(), Box<dyn std::error::Error>> {
    // retrieve the device list
    let devs = Device::list()?;

    // print the list
    for (i, dev) in devs.iter().enumerate() {
        println!(
            "{}. {} ({})",
            i + 1,
            dev.name,
            dev.desc.as_deref().unwrap_or("No description available")
        );
    }

    // input nth of NIC
    println!("Enter the NIC number (1-{}): ", devs.len());
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let inum: usize = input.trim().parse()?;

    // check if it is a valid NIC number
    if inum < 1 || inum > devs.len() {
        println!("NIC number out of range.");
        return Err(anyhow!("NIC number out of range").into());
    }

    // get the selected NIC dev
    let selected_dev = &devs[inum - 1];

    // open the NIC dev
    let mut cap = pcap::Capture::from_device(selected_dev.name.as_str())?
        .promisc(true)
        .snaplen(65_536)
        .timeout(1_000)
        .open()?;

    println!(
        "Listening on {}...",
        selected_dev.desc.as_deref().unwrap_or(&selected_dev.name)
    );

    // start the capture
    loop {
        match cap.next_packet() {
            Ok(packet) => handle_packet(packet),
            Err(pcap::Error::TimeoutExpired) => {
                // continue on timeout error
                continue;
            }
            Err(e) => {
                // error except timeout
                eprintln!("An Error occurred: {e:?}");
                break;
            }
        }
    }

    Ok(())
}
