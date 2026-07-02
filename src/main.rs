use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::time::Instant;

mod parser;
mod schema;

use parser::L3Parser;
use schema::itchformat::ItchMessage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let buffer = fs::read(file_path)?;

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

    // HARD LIMIT SAFETY (prevents runaway parsing)
    let mut messages = Vec::with_capacity(500_000);
    let mut last_pos = 0;
    let mut stuck_counter = 0;

    // integrity stats
    let mut unknown_streak = 0;
    let mut max_unknown_streak = 0;
    let mut timestamp_regressions = 0;
    let mut unknown_count = 0;
    let mut unknown_types: HashMap<u8, usize> = HashMap::new();

    let mut last_timestamp: Option<u64> = None;

    while let Some(msg) = parser.parse_next() {
        let pos = parser.position();

        // 1. STUCK DETECTION (parser not advancing)
        if pos == last_pos {
            stuck_counter += 1;
            if stuck_counter > 10 {
                eprintln!("FATAL: parser stuck at position {}", pos);
                break;
            }
        } else {
            stuck_counter = 0;
        }
        last_pos = pos;

        // 2. UNKNOWN SPIKE DETECTION
        let name = msg.name();
        if name == "Unknown" {
            unknown_streak += 1;
            max_unknown_streak = max_unknown_streak.max(unknown_streak);
            unknown_count += 1;

            if let ItchMessage::Unknown(unknown_msg) = &msg {
                *unknown_types.entry(unknown_msg.message_type).or_insert(0) += 1;
            }

        } else {
            unknown_streak = 0;
        }

        // 3. TIMESTAMP MONOTONIC CHECK (if message has timestamp)
        if let Some(ts) = msg.timestamp() {
            if let Some(prev) = last_timestamp {
                if ts < prev {
                    timestamp_regressions += 1;
                }
            }
            last_timestamp = Some(ts);
        }

        messages.push(msg);
    }

    println!("\nParsed {} messages", messages.len());

    // ---- STATISTICS ----
    let mut message_type_counts: HashMap<&str, usize> = HashMap::new();

    for msg in &messages {
        *message_type_counts.entry(msg.name()).or_insert(0) += 1;
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

    println!("\nIntegrity report:");
    println!("  max_unknown_streak     : {}", max_unknown_streak);
    println!("  timestamp_regressions  : {}", timestamp_regressions);
    println!("  stuck_counter events    : {}", stuck_counter);
    
    println!("\nUnknown message types:");
    let mut unknown_type_vec: Vec<_> = unknown_types.iter().collect();
    unknown_type_vec.sort_by_key(|&(_, count)| std::cmp::Reverse(*count));
    for (msg_type, count) in unknown_type_vec {
        println!("  '{}' (0x{:02x}): {}", *msg_type as char, msg_type, count);
    }

    println!("\nTotal messages processed: {}", messages.len());

    let elapsed = start.elapsed();
    println!("Time taken: {:.2}s", elapsed.as_secs_f64());

    std::io::stdout().flush()?;
    Ok(())
}