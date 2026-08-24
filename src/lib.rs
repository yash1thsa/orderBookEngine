#![allow(non_snake_case)]
#![allow(clippy::new_without_default)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::repr_packed_without_abi)]
#![allow(clippy::module_inception)]

//! # orderBookEngine
//!
//! A high-performance ITCH protocol parser for NASDAQ order book data.
//!
//! This library provides efficient parsing of NASDAQ ITCH (Integrated Trading and Clearing House) protocol messages,
//! which are used to disseminate real-time market data. The parser is optimized for performance using memory mapping
//! and zero-copy techniques.
//!
//! ## Features
//!
//! - **High-performance parsing**: Uses memory mapping for efficient file processing
//! - **Zero-copy design**: Minimizes memory allocations during parsing
//! - **Comprehensive message support**: Handles all major ITCH message types
//! - **Arrow IPC output**: Exports parsed data to Apache Arrow format for efficient analytics
//! - **Batch processing**: Configurable batch sizes for memory-efficient processing
//!
//! ## Example
//!
//! ```no_run
//! use orderBookEngine::parser::L3Parser;
//! use orderBookEngine::schema::itchformat::ItchMessage;
//!
//! // Read ITCH file
//! let file = std::fs::File::open("data.itch")?;
//! let mmap = unsafe { memmap2::MmapOptions::new().map(&file)? };
//! let buffer: &[u8] = &mmap;
//!
//! // Parse messages
//! let mut parser = L3Parser::new(&buffer);
//! while let Some(msg) = parser.parse_next() {
//!     match msg {
//!         ItchMessage::SystemEvent(event) => {
//!             println!("System event: {:?}", event);
//!         }
//!         ItchMessage::AddOrder(order) => {
//!             println!("Add order: {:?}", order);
//!         }
//!         // Handle other message types...
//!         _ => {}
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod parser;
pub mod schema;
pub mod utils;
