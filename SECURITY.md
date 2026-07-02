# Security Policy

## Supported Versions

Currently, only the latest version of orderBookEngine is supported with security updates.

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly.

### How to Report

**Do not** open a public issue for security vulnerabilities.

Instead, send an email to: syashwanth87@gmail.com

Please include:
- A description of the vulnerability
- Steps to reproduce the issue
- Potential impact if exploited
- Any suggested mitigation (if known)

### Response Timeline

- **Initial response**: Within 48 hours
- **Detailed response**: Within 7 days
- **Public disclosure**: After a fix has been released

### What to Expect

1. We will acknowledge receipt of your report within 48 hours
2. We will work with you to understand and validate the issue
3. We will determine the severity and impact
4. We will develop a fix and coordinate release with you
5. We will publicly disclose the vulnerability after the fix is released

## Security Best Practices

When using orderBookEngine:
- Keep dependencies updated
- Review and validate input data before parsing
- Use memory mapping carefully with untrusted files
- Monitor resource usage when processing large files

## Dependency Security

This project uses Rust's cargo for dependency management. We regularly update dependencies to address security vulnerabilities. We encourage users to:
- Run `cargo audit` to check for known vulnerabilities
- Keep Rust toolchain updated
- Review security advisories for dependencies

## Security-Related Features

- Memory-safe Rust implementation
- No unsafe code in critical parsing paths
- Bounds checking on all array access
- Input validation for ITCH protocol messages
