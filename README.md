# orderBookEngine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/orderBookEngine)](https://crates.io/crates/orderBookEngine)
[![Build Status](https://github.com/yourusername/orderBookEngine/workflows/CI/badge.svg)](https://github.com/yourusername/orderBookEngine/actions)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

A high-performance ITCH protocol parser for NASDAQ order book data written in Rust.

## Overview

This library provides efficient parsing of NASDAQ ITCH (Integrated Trading and Clearing House) protocol messages, which are used to disseminate real-time market data. The parser is optimized for performance using memory mapping and zero-copy techniques.

## Features

- **High-performance parsing**: Uses memory mapping for efficient file processing
- **Zero-copy design**: Minimizes memory allocations during parsing
- **Comprehensive message support**: Handles all major ITCH message types
- **Arrow IPC output**: Exports parsed data to Apache Arrow format for efficient analytics
- **Batch processing**: Configurable batch sizes for memory-efficient processing

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
orderBookEngine = "0.1.0"
```

## Library Usage

### Basic Parsing

```rust
use orderBookEngine::parser::L3Parser;
use orderBookEngine::schema::itchformat::ItchMessage;

// Read ITCH file
let file = std::fs::File::open("data.itch")?;
let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
let buffer: &[u8] = &mmap;

// Parse messages
let mut parser = L3Parser::new(&buffer);
while let Some(msg) = parser.parse_next() {
    match msg {
        ItchMessage::SystemEvent(event) => {
            println!("System event: {:?}", event);
        }
        ItchMessage::AddOrder(order) => {
            println!("Add order: {:?}", order);
        }
        // Handle other message types...
        _ => {}
    }
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

### Writing to Arrow IPC

```rust
use orderBookEngine::parser::L3Parser;
use orderBookEngine::utils::L3Writer;

let file = std::fs::File::open("data.itch")?;
let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
let buffer: &[u8] = &mmap;

let mut parser = L3Parser::new(&buffer);
let mut writer = L3Writer::new("./output".to_string(), 10000);

while let Some(msg) = parser.parse_next() {
    writer.add_message(msg);
}

writer.flush_remaining();
# Ok::<(), Box<dyn std::error::Error>>(())
```

### Collecting Statistics

```rust
use orderBookEngine::parser::L3Parser;
use orderBookEngine::utils::StatsCollector;

let mut parser = L3Parser::new(&buffer);
let mut stats = StatsCollector::new();

while let Some(msg) = parser.parse_next() {
    stats.process_message(&msg);
}

stats.report();
```

## CLI Usage

The library also includes a CLI tool for quick parsing:

```bash
cargo build --release
./target/release/orderBookEngine <input_file>
```

## Supported ITCH Message Types

- System Event
- Stock Directory
- Stock Trading Action
- Add Order
- Add Order with MPID
- Order Executed
- Order Executed with Price
- Order Cancel
- Order Delete
- Order Replace
- Trade
- Cross Trade
- Order Priority Update
- Net Order Imbalance Indicator
- Market Participant Position

## Output Format

When using the Arrow writer, parsed data is written to Arrow IPC files organized by message type:
- `SystemEvent_*.arrow`
- `StockDirectory_*.arrow`
- `AddOrder_*.arrow`
- And so on for each message type

## Performance

The parser is designed for high throughput:
- Memory-mapped file I/O for efficient reading
- Zero-copy parsing to minimize allocations
- Batch writing to Arrow format for efficient serialization

## Dependencies

- `memmap2` - Memory mapping for efficient file I/O
- `flate2` - Gzip decompression
- `arrow` - Apache Arrow for columnar data format
- `parquet` - Apache Parquet support
- `chrono` - Date and time handling

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

## Code of Conduct

This project adheres to a code of conduct - see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

## Acknowledgments

Built for efficient parsing of NASDAQ ITCH market data feeds.
