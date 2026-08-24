use std::env;
use std::io::Write;
use std::time::Instant;

use orderBookEngine::parser::L3Parser;
use orderBookEngine::utils::{L3Writer, StatsCollector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = std::fs::File::open(file_path)?;
    // SAFETY: memmap2::map() is unsafe because it relies on OS-level memory mapping.
    // The safety is guaranteed by the memmap2 crate which handles:
    // - File handle validity (checked by File::open above)
    // - Memory mapping bounds and alignment
    // - Lifetime management (mmap owns the mapping)
    let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
    let buffer: &[u8] = &mmap; // Behaves exactly like a normal byte slice!

    println!("File size: {} bytes", buffer.len());

    // ---- PARSER ----
    let mut parser = L3Parser::new(buffer);
    let mut l3_writer = L3Writer::new("./output".to_string(), 3_000_000);
    let mut stats = StatsCollector::new();

    while let Some(msg) = parser.parse_next() {
        stats.process_message(&msg);
        l3_writer.add_message(msg);
    }

    // Flush remaining messages
    l3_writer.flush_remaining();

    // ---- STATISTICS ----
    stats.report();

    let elapsed = start.elapsed();
    println!("Time taken: {:.2}s", elapsed.as_secs_f64());

    std::io::stdout().flush()?;
    Ok(())
}
