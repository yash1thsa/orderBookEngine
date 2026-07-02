use std::env;
use std::fs;
use std::io::Write;
use std::time::Instant;

mod parser;
mod schema;
mod utils;

use parser::L3Parser;
use schema::itchformat::ItchMessage;
use utils::{ParquetWriter, StatsCollector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = std::fs::File::open(file_path)?;
    let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
    let buffer: &[u8] = &mmap; // Behaves exactly like a normal byte slice!

    println!("File size: {} bytes", buffer.len());

    // ---- HEXDUMP SAFETY ----
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

    // ---- PARSER ----
    let mut parser = L3Parser::new(&buffer);
    let mut parquet_writer = ParquetWriter::new("./output".to_string(), 100_000);
    //let mut stats = StatsCollector::new();

    while let Some(msg) = parser.parse_next() {
        //stats.process_message(&msg);
        parquet_writer.add_message(msg);
    }

    // Flush remaining messages
    parquet_writer.flush_remaining();

    // ---- STATISTICS ----
    //stats.report();

    let elapsed = start.elapsed();
    println!("Time taken: {:.2}s", elapsed.as_secs_f64());

    std::io::stdout().flush()?;
    Ok(())
}