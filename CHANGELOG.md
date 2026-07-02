# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial open source release
- High-performance ITCH protocol parser for NASDAQ order book data
- Memory-mapped file I/O for efficient reading
- Zero-copy parsing to minimize allocations
- Comprehensive message support for all major ITCH message types
- Arrow IPC output for efficient analytics
- Batch processing with configurable batch sizes
- Statistics collection utilities
- CLI tool for quick parsing

### Supported ITCH Message Types
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

## [0.1.0] - 2026-08-24

### Added
- Initial release
- Core parsing functionality
- Arrow/Parquet integration
- Basic CLI interface
