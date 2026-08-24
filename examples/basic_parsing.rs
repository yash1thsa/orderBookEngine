//! Basic parsing example for orderBookEngine
//!
//! This example demonstrates how to parse ITCH protocol messages using the library.

use orderBookEngine::parser::L3Parser;
use orderBookEngine::schema::itchformat::ItchMessage;
use orderBookEngine::utils::{L3Writer, StatsCollector};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <itch_file> <output_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let output_path = &args[2];
    let file = std::fs::File::open(file_path)?;

    // Memory map the file for efficient reading
    let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
    let buffer: &[u8] = &mmap;

    println!("File size: {} bytes", buffer.len());

    // Create parser, writer, and stats collector
    let mut parser = L3Parser::new(&buffer);
    let mut l3_writer = L3Writer::new(output_path.to_string(), 3_000_000);
    let mut stats = StatsCollector::new();
    let mut count = 0;

    while let Some(msg) = parser.parse_next() {
        count += 1;

        // Print first 10 messages for demonstration
        if count <= 10 {
            match &msg {
                ItchMessage::SystemEvent(event) => {
                    println!(
                        "SystemEvent: timestamp={}, event_code={}",
                        event.timestamp, event.event_code
                    );
                }
                ItchMessage::AddOrder(order) => {
                    println!(
                        "AddOrder: stock_locate={}, shares={}",
                        order.stock_locate, order.shares
                    );
                }
                ItchMessage::Trade(trade) => {
                    println!("Trade: shares={}, price={}", trade.shares, trade.price);
                }
                _ => {
                    println!("Message type: {}", msg.name());
                }
            }
        }

        stats.process_message(&msg);
        l3_writer.add_message(msg);
    }

    l3_writer.flush_remaining();
    stats.report();
    println!("\nTotal messages parsed: {}", count);
    Ok(())
}
