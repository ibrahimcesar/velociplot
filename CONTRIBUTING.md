# Contributing to Velociplot

Thank you for your interest in contributing to Velociplot! 🦖

This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Release Process](#release-process)
- [Recognition](#recognition)

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Be respectful, inclusive, and professional in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/velociplot
   cd velociplot
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/ibrahimcesar/velociplot
   ```
4. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-new-feature
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```bash
# Build the library
cargo build

# Build with all features
cargo build --all-features

# Build the CLI tool
cargo build --features cli --bin vplot
```

### Running Tests

```bash
# Run all tests
cargo test --all-features

# Run specific test
cargo test test_name

# Run doc tests
cargo test --doc
```

### Running Examples

```bash
# List all examples
cargo run --example

# Run specific example
cargo run --example basic_line

# Run with features
cargo run --example ndarray_integration --features ndarray-support
```

## How to Contribute

### Types of Contributions

We welcome:

- 🐛 **Bug reports** - Found a bug? Open an issue!
- 💡 **Feature requests** - Have an idea? Let us know!
- 📝 **Documentation** - Improve docs, add examples
- 🎨 **Examples** - Create new plot examples
- 🔧 **Code** - Fix bugs, add features
- 🧪 **Tests** - Improve test coverage
- 🎨 **Colormaps** - Add new color schemes

## Commit Message Guidelines

This project follows the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages. This leads to **more readable messages** and enables automatic changelog generation.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

Must be one of:

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, etc.)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Performance improvement
- **test**: Adding or fixing tests
- **build**: Changes to build system or dependencies
- **ci**: Changes to CI configuration
- **chore**: Other changes that don't modify src or test files
- **revert**: Reverts a previous commit

### Scope

The scope should indicate what part of the codebase is affected:

- **core** - Core plotting engine
- **plots** - Plot types (line, scatter, etc.)
- **legend** - Legend system
- **axes** - Axis system
- **backend** - Rendering backends
- **cli** - Command-line interface
- **examples** - Example code
- **docs** - Documentation
- **deps** - Dependencies

Scope is optional but recommended.

### Subject

- Use imperative, present tense: "add" not "added" nor "adds"
- Don't capitalize first letter
- No period (.) at the end
- Limit to 50 characters

### Examples

```bash
# Feature addition
git commit -m "feat(plots): add DateListPlot for time series visualization"

# Bug fix
git commit -m "fix(legend): correct vertical alignment calculation"

# Documentation
git commit -m "docs(readme): update installation instructions"

# Performance improvement
git commit -m "perf(heatmap): optimize colormap lookup"

# Breaking change
git commit -m "feat(core)!: redesign Drawable trait interface

BREAKING CHANGE: Drawable::draw() now requires mutable canvas reference"
```

### Breaking Changes

Breaking changes must be indicated by:
1. `!` after the type/scope: `feat(api)!:`
2. `BREAKING CHANGE:` in the footer

### Pull Request Titles

PR titles must also follow conventional commits format as they will be used in the changelog:

```
feat(plots): add bubble chart support
fix(cli): handle empty CSV files gracefully
docs: add user guide for custom fonts
```

### Validation

All commits and PR titles are automatically validated by GitHub Actions. Invalid format will cause CI to fail.

### Reporting Bugs

When reporting bugs, include:

1. **Description** - Clear description of the bug
2. **Steps to reproduce** - Minimal code to reproduce the issue
3. **Expected behavior** - What you expected to happen
4. **Actual behavior** - What actually happened
5. **Environment** - OS, Rust version, velociplot version
6. **Error messages** - Full error output or stack trace

**Example:**

```markdown
**Bug:** Heatmap crashes with empty data

**To reproduce:**
```rust
let heatmap = Heatmap::new(vec![], 0, 0);
```

**Expected:** Should return an error
**Actual:** Panics with index out of bounds
**Environment:** macOS 14, Rust 1.75, velociplot 0.1.0
```

### Suggesting Features

When suggesting features, include:

1. **Use case** - Why is this feature needed?
2. **Proposed API** - How would users interact with it?
3. **Examples** - Show example usage
4. **Alternatives** - Other ways to achieve the same goal

## Coding Standards

### Rust Style

- **Follow Rust conventions** - Use `rustfmt` for formatting
- **Run clippy** - Fix all warnings before submitting
- **Document public APIs** - All public items need doc comments
- **Add examples** - Include usage examples in doc comments

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy --all-features -- -D warnings
```

### API Design Principles

1. **Builder pattern** - Use for complex configuration
   ```rust
   let plot = LinePlot::new(data)
       .color(Color::RED)
       .line_width(2.0);
   ```

2. **#[must_use]** - Mark builder methods with `#[must_use]`
   ```rust
   #[must_use]
   pub fn color(mut self, color: Color) -> Self {
       self.color = color;
       self
   }
   ```

3. **Result types** - Use `Result` for fallible operations
   ```rust
   pub fn new(data: Vec<f64>) -> Result<Self> {
       if data.is_empty() {
           return Err(Error::InvalidData("Data cannot be empty".into()));
       }
       Ok(Self { data })
   }
   ```

4. **Implement Drawable** - All plot types implement the Drawable trait
   ```rust
   impl Drawable for MyPlot {
       fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
           // Implementation
       }
   }
   ```

### Code Organization

- Keep modules focused and cohesive
- Limit file size (< 500 lines if possible)
- Group related functionality
- Use `mod.rs` to export public API

### Documentation

All public APIs must have:

1. **Summary** - One-line description
2. **Description** - Detailed explanation (if needed)
3. **Arguments** - Document all parameters
4. **Returns** - What the function returns
5. **Errors** - When it returns an error
6. **Examples** - Code example showing usage
7. **Panics** - When it might panic (if applicable)

**Example:**

```rust
/// Create a new scatter plot
///
/// Creates a scatter plot from x and y coordinate vectors. The vectors
/// must have the same length and cannot be empty.
///
/// # Arguments
///
/// * `x` - X coordinates
/// * `y` - Y coordinates
///
/// # Errors
///
/// Returns an error if:
/// - Vectors are empty
/// - Vectors have different lengths
///
/// # Examples
///
/// ```
/// # use velociplot::prelude::*;
/// let x = vec![1.0, 2.0, 3.0];
/// let y = vec![1.0, 4.0, 9.0];
/// let plot = ScatterPlot::new(x, y)?;
/// # Ok::<(), velociplot::error::Error>(())
/// ```
pub fn new(x: Vec<f64>, y: Vec<f64>) -> Result<Self> {
    // Implementation
}
```

## Testing

### Test Requirements

All contributions must include tests:

1. **Unit tests** - Test individual functions
2. **Doc tests** - Examples in documentation must compile
3. **Integration tests** - Test public API end-to-end

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let plot = LinePlot::new(Series::new(vec![0.0], vec![1.0]).unwrap());
        assert!(plot.is_ok());
    }

    #[test]
    fn test_empty_data() {
        let result = LinePlot::new(Series::new(vec![], vec![]).unwrap());
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# All tests
cargo test --all-features

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Doc tests only
cargo test --doc
```

### Test Coverage

- Aim for >80% test coverage
- Test happy paths and error cases
- Test edge cases (empty data, single point, etc.)
- Test validation logic

## Pull Request Process

### Before Submitting

1. ✅ **Tests pass** - `cargo test --all-features`
2. ✅ **Clippy clean** - `cargo clippy --all-features -- -D warnings`
3. ✅ **Formatted** - `cargo fmt`
4. ✅ **Documentation** - All public APIs documented
5. ✅ **Examples work** - If adding features, add examples
6. ✅ **Changelog updated** - Add entry to CHANGELOG.md

### PR Description

Include:

1. **What** - What does this PR do?
2. **Why** - Why is this change needed?
3. **How** - How was it implemented?
4. **Testing** - How was it tested?
5. **Screenshots** - For visual changes, include before/after images
6. **Breaking changes** - Note any API changes

**Example:**

```markdown
## Add DateListPlot for time series visualization

### What
Implements DateListPlot for plotting data over time, similar to Wolfram's DateListPlot.

### Why
Users need to visualize time series data with automatic time axis formatting.

### How
- Created new DateListPlot struct with TimeSeries components
- Supports Line, Points, and LinePoints styles
- Multi-series support with automatic color assignment
- Optional grid display

### Testing
- Added 5 unit tests
- Added 6 examples covering different use cases
- All 93 tests pass

### Screenshots
![Temperature over time](examples/images/datelistplot_temperature.png)
```

### Review Process

1. **Automated checks** - CI must pass
2. **Code review** - Maintainer reviews code
3. **Discussion** - Address feedback
4. **Approval** - Maintainer approves
5. **Merge** - Squash and merge

### After Merge

- Delete your feature branch
- Update your local main:
  ```bash
  git checkout main
  git pull upstream main
  ```

## Release Process

### Versioning

We follow [Semantic Versioning](https://semver.org/):

- **Major** (1.0.0) - Breaking changes
- **Minor** (0.1.0) - New features, backward compatible
- **Patch** (0.0.1) - Bug fixes, backward compatible

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release notes
3. Run full test suite
4. Create git tag: `git tag -a v1.0.0 -m "Release 1.0.0"`
5. Push tag: `git push upstream v1.0.0`
6. GitHub Action will publish to crates.io

## Adding New Plot Types

To add a new plot type:

1. **Create module** - `src/plots/myplot.rs`
2. **Implement struct** - With builder pattern
3. **Implement Drawable** - Core rendering logic
4. **Add tests** - Unit tests and doc tests
5. **Create example** - `examples/myplot.rs`
6. **Update exports** - Add to `src/plots/mod.rs` and `src/lib.rs`
7. **Update README** - Add to feature list
8. **Update CHANGELOG** - Document the addition

## Questions?

- **GitHub Issues** - For bugs and features
- **GitHub Discussions** - For questions and ideas
- **Email** - email@ibrahimcesar.com

## Recognition

We use [All Contributors](https://allcontributors.org/) to recognize all contributions, not just code!

### Adding Yourself

After your first contribution is merged, you'll be automatically added to the contributors list. You can also add yourself by commenting on your PR:

```
@all-contributors please add @username for code, docs
```

### Contribution Types

We recognize various contribution types:

- 💻 **code** - Code contributions
- 📖 **docs** - Documentation
- 🎨 **design** - Design and UX
- 💡 **ideas** - Ideas and planning
- 🤔 **example** - Examples
- 🐛 **bug** - Bug reports
- 🚧 **infra** - Infrastructure (CI/CD, build tools)
- 🚇 **maintenance** - Repository maintenance
- ✅ **test** - Writing tests
- 📢 **talk** - Talks and presentations
- 📹 **video** - Videos and screencasts
- ⚠️ **review** - Reviewing pull requests

Contributors will also be recognized in:

- **README.md** - All Contributors section (automated)
- **CHANGELOG.md** - For each release
- **GitHub contributors page** - Built-in recognition
- **Release notes** - Major contributions highlighted

### Current Contributors

See the [Contributors section](https://github.com/ibrahimcesar/velociplot#contributors) in the README.

Thank you for contributing to Velociplot! 🦖
