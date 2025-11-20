# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 0.0.x   | :white_check_mark: |

## Reporting a Vulnerability

The Velociplot team and community take security bugs seriously. We appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to [email@ibrahimcesar.com](mailto:email@ibrahimcesar.com).

Include the following information in your report:

- Type of issue (e.g., buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

- You should receive an acknowledgment within 48 hours.
- We will investigate the issue and provide an estimated timeline for a fix.
- We will keep you informed of the progress towards resolving the issue.
- We may ask for additional information or guidance.

### Disclosure Policy

- We will coordinate with you on the disclosure timeline.
- We prefer to fully remediate vulnerabilities before public disclosure.
- We will credit you in the security advisory (unless you prefer to remain anonymous).

## Security Update Process

1. The security report is received and assigned a primary handler.
2. The problem is confirmed and a list of affected versions is determined.
3. Code is audited to find any similar problems.
4. Fixes are prepared for all supported releases.
5. A security advisory is published on GitHub Security Advisories.
6. New versions are released with the fix.
7. The advisory is announced in the project README and release notes.

## Preferred Languages

We prefer all communications to be in English.

## Known Security Considerations

### Input Validation

Velociplot validates data inputs to prevent:
- Empty data arrays
- Mismatched x/y array lengths
- Invalid numeric values (NaN, Infinity)

### File I/O (CLI)

The CLI tool (when implemented) will:
- Validate file paths to prevent directory traversal
- Limit file sizes to prevent DoS attacks
- Sanitize CSV/JSON input to prevent injection attacks

### Dependencies

We minimize external dependencies and regularly audit them:
- `tiny-skia` - Well-maintained, pure Rust rendering
- `fontdue` - Pure Rust font rendering
- `thiserror` - Error handling library
- `clap` - Command-line parsing (CLI feature only)

### Memory Safety

Velociplot is written in pure Rust and does not use `unsafe` code in core functionality, providing strong memory safety guarantees.

## Security Best Practices for Users

### When Using the Library

1. **Validate data sources**: Always validate external data before passing to Velociplot
2. **Limit data sizes**: Be aware of memory usage with large datasets
3. **Sanitize file paths**: When using CLI, validate input/output paths
4. **Keep dependencies updated**: Run `cargo update` regularly

### When Using the CLI

1. **Validate input files**: Ensure CSV/JSON files come from trusted sources
2. **Use absolute paths**: Prefer absolute paths over relative paths
3. **Limit file sizes**: Implement size limits for input files in production
4. **Sanitize output paths**: Validate output paths to prevent overwriting critical files

## Acknowledgments

We would like to thank the following individuals for responsibly disclosing security issues:

- (No vulnerabilities reported yet)

## Contact

For any questions about this security policy, please contact [email@ibrahimcesar.com](mailto:email@ibrahimcesar.com).

---

This security policy is based on best practices from the Rust Security Response WG and other open source projects.
