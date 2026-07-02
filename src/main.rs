use std::env;
use std::fs;
use std::io::Write;
use std::time::Instant;

mod parser;
mod schema;
mod utils;

use parser::L3Parser;
// Note: ItchMessage now expects a lifetime parameter, e.g., ItchMessage<'a>
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

    // ⚡ ZERO-COPY ROOT: Memory map the file.
    // This buffer's lifetime defines the `'a` constraint for the entire pipeline execution.
    let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
    let buffer: &[u8] = &mmap;

    println!("File size: {} bytes", buffer.len());

    // ---- PARSER ----
    // The compiler automatically ties the lifetime of `parser` and all generated
    // messages directly to the underlying `buffer` slice.
    let mut parser = L3Parser::new(buffer);
    let mut parquet_writer = ParquetWriter::new("./output".to_string(), 100_000);
    // let mut stats = StatsCollector::new();

    let mut message_count = 0;

    // Stream through the file buffer via zero-copy pointer casting
    while let Some(msg) = parser.parse_next() {
        // stats.process_message(&msg);

        // ⚡ CRITICAL: `parquet_writer.add_message` must consume or copy the data
        // into its internal row record groups before the loop steps forward.
        parquet_writer.add_message(msg);

        message_count += 1;
    }

    // Flush remaining buffered rows to disk
    parquet_writer.flush_remaining();

    // ---- STATISTICS ----
    // stats.report();

    let elapsed = start.elapsed();
    println!("Processed {} messages successfully.", message_count);
    println!("Time taken: {:.2}s", elapsed.as_secs_f64());

    std::io::stdout().flush()?;
    Ok(())
}