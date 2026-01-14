# Contributing to oracledb-rs

Thank you for your interest in contributing to oracledb-rs! We welcome contributions from the community.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/oracledb-rs.git`
3. Create a feature branch: `git checkout -b feature/my-new-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Run formatting: `cargo fmt`
7. Run clippy: `cargo clippy`
8. Commit your changes: `git commit -am 'Add some feature'`
9. Push to the branch: `git push origin feature/my-new-feature`
10. Submit a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Oracle Database (for integration tests)

### Building

```bash
cargo build
```

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests (requires Oracle Database)
cargo test --features integration-tests
```

### Code Style

We use `rustfmt` and `clippy` to maintain code quality:

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy
```

## Pull Request Guidelines

- Keep PRs focused on a single feature or bug fix
- Include tests for new functionality
- Update documentation as needed
- Ensure all tests pass
- Follow the existing code style
- Write clear commit messages

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Oracle Database version
- Operating system
- Minimal code example that reproduces the issue
- Error messages and stack traces

## Feature Requests

We welcome feature requests! Please:

- Check if the feature has already been requested
- Clearly describe the use case
- Explain why this feature would be useful to others

## Code of Conduct

Please be respectful and constructive in all interactions.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).
