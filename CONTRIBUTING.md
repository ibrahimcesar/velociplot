# Contributing to velociplot 🦖

Thank you for your interest in contributing to velociplot!

## 🚧 Project Status

This project is in **early development**. The core architecture is being designed.

## 🤝 How to Contribute

### Reporting Bugs

Open an issue with:
- Clear description of the problem
- Minimal reproducible example
- Expected vs actual behavior
- Your environment (OS, Rust version)

### Suggesting Features

Open an issue with:
- Use case description
- Proposed API (if applicable)
- Example code showing intended usage

### Code Contributions

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Run `cargo test` and `cargo clippy`
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📋 Development Setup
```bash
# Clone repository
git clone https://github.com/yourusername/velociplot
cd velociplot

# Build
cargo build

# Run tests
cargo test

# Run examples
cargo run --example basic_line

# Run CLI
cargo run --bin vplot --features cli -- --help
```

## 🎨 Code Style

- Follow Rust standard style (`cargo fmt`)
- Pass `cargo clippy` without warnings
- Add documentation for public APIs
- Include examples in doc comments

## 📝 Documentation

- All public items must have documentation
- Include examples in doc comments where appropriate
- Update README.md if adding major features

## 🧪 Testing

- Add unit tests for new functionality
- Add integration tests for high-level features
- Ensure `cargo test` passes

## 📊 Areas Needing Help

Current priorities:
- [ ] Core plot architecture design
- [ ] Color palette definitions
- [ ] Basic rendering backend
- [ ] Example gallery
- [ ] Documentation

## 💬 Questions?

Open an issue labeled "question" or discussion.

## 📜 License

By contributing, you agree that your contributions will be licensed under MIT.
