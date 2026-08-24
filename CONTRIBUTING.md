# Contributing to orderBookEngine

Thank you for your interest in contributing to orderBookEngine! This document provides guidelines and instructions for contributing to the project.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates. When creating a bug report, include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior vs. actual behavior
- Environment information (OS, Rust version)
- Any relevant error messages or logs

### Suggesting Enhancements

Enhancement suggestions are welcome. Please include:

- A clear description of the proposed enhancement
- Motivation for the enhancement
- Possible implementation approaches
- Any relevant examples or references

### Pull Requests

1. **Fork the repository** and create a branch for your changes
2. **Make your changes** following the coding standards below
3. **Write tests** for new functionality (see Testing section)
4. **Ensure all tests pass** with `cargo test`
5. **Update documentation** if needed
6. **Submit a pull request** with a clear description of your changes

## Development Setup

### Prerequisites

- Rust (latest stable version recommended)
- Git

### Building the Project

```bash
# Clone the repository
git clone https://github.com/yash1thsa/orderBookEngine.git
cd orderBookEngine

# Build in development mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Coding Standards

### Rust Style

- Follow standard Rust coding conventions
- Use `cargo fmt` to format code
- Use `cargo clippy` to catch common issues
- Keep functions focused and single-purpose
- Add doc comments for public APIs

### Code Organization

- Parser modules should follow the existing structure in `src/parser/`
- Utility functions go in `src/utils/`
- Schema definitions go in `src/schema/`
- Keep related functionality together

### Documentation

- Document public APIs with `///` doc comments
- Include examples in doc comments where helpful
- Update README for user-facing changes
- Add comments for complex logic

### Commit Messages

Use clear, descriptive commit messages:

```
feat: add support for new ITCH message type
fix: correct timestamp parsing for cross-trade messages
docs: update README with new usage examples
test: add unit tests for order cancellation logic
```

## Testing

### Writing Tests

- Unit tests should be placed in the same module as the code they test
- Integration tests go in the `tests/` directory
- Test files should be named `test_*.rs` or `*_test.rs`

### Test Coverage

- Aim for good test coverage on critical parsing logic
- Test edge cases and error conditions
- Include tests for performance-sensitive code where applicable

## Project Structure

```
orderBookEngine/
├── src/
│   ├── main.rs           # Entry point
│   ├── parser/           # ITCH message parsers
│   ├── schema/           # Data structures
│   └── utils/            # Utilities (writer, stats)
├── tests/                # Integration tests
├── Cargo.toml           # Project configuration
├── README.md            # Project documentation
├── LICENSE              # MIT License
└── CONTRIBUTING.md      # This file
```

## Performance Considerations

This project is optimized for high-performance parsing. When making changes:

- Benchmark performance-critical code before and after changes
- Avoid unnecessary allocations in hot paths
- Consider zero-copy techniques where possible
- Profile using `cargo flamegraph` or similar tools if needed

## Getting Help

- Open an issue for bugs or questions
- Check existing documentation first
- Be respectful and constructive in all communications

## License

By contributing, you agree that your contributions will be licensed under the MIT License, same as the project.
