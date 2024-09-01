use std::io::Write;

use crate::packet_dump::dump_packet;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\nSelect an function to run.");
        println!("1. Packet Dump");
        println!("2. -----------");
        println!("3. -----------");
        println!("4. -----------");
        println!("5. -----------");
        println!("6. -----------");
        println!("0. Exit");

        print!("enter number: ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.trim().parse::<u32>()? {
            1 => dump_packet(),
            2 => {}
            3 => {}
            0 => {
                println!("You choose to exit program. bye.");
                break;
            }
            _ => println!("Invalid number input, please try again."),
        }
    }

    Ok(())
}
