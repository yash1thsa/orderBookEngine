use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use flate2::read::GzDecoder;

mod parser;
mod schema;

use parser::L3Parser;
use schema::itchformat::ItchMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.gz>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path)?;

    let mut decoder = GzDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    println!("Uncompressed size: {} bytes", buffer.len());

    // Hex dump (first 80 bytes)
    println!("\nRaw hexdump (first 80 bytes):");
    for i in 0..buffer.len().min(80) {
        if i % 16 == 0 {
            print!("{:04x}: ", i);
        }
        print!("{:02x} ", buffer[i]);
        if i % 16 == 15 {
            println!();
        }
    }
    if buffer.len() % 16 != 0 {
        println!();
    }

    let mut parser = L3Parser::new(&buffer);

    // LIMIT to 500 messages (prevents hanging)
    let messages = parser.parse_n(200000000);

    println!("\nParsed {} messages", messages.len());

    let mut message_type_counts: HashMap<&str, usize> = HashMap::new();

    for msg in &messages {
        let type_name = msg.name();
        *message_type_counts.entry(type_name).or_insert(0) += 1;
    }

    println!("\nFirst 5 messages:");
    for (i, msg) in messages.iter().take(5).enumerate() {
        println!("Message {}: {}", i + 1, msg.name());
    }

    println!("\nMessage statistics:");
    for (name, count) in &message_type_counts {
        println!(
            "{:<25} {:>10} ({:.2}%)",
            name,
            count,
            (*count as f64 / messages.len() as f64) * 100.0
        );
    }

    println!("\nTotal messages processed: {}", messages.len());

    Ok(())
}