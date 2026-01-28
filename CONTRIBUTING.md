# Contributing to Nova

Thank you for your interest in contributing to Nova! 🎉

## Code of Conduct

Be respectful, inclusive, and collaborative.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported
2. Create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - System information (OS, Rust version)

### Suggesting Features

1. Check existing feature requests
2. Create an issue describing:
   - The problem you're solving
   - Your proposed solution
   - Alternative approaches considered

### Pull Requests

1. **Fork the repository**
2. **Create a branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes**
4. **Add tests** for new functionality
5. **Run tests**: `cargo test`
6. **Format code**: `cargo fmt`
7. **Commit**: `git commit -m 'Add amazing feature'`
8. **Push**: `git push origin feature/amazing-feature`
9. **Open a Pull Request**

## Development Setup

```bash
# Clone the repo
git clone https://github.com/nova-container/core
cd core

# Build
cargo build

# Run tests
cargo test

# Run examples
cargo run -- run examples/hello.wat
```

## Code Style

- Follow Rust conventions
- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Write meaningful commit messages

## Testing

- Add tests for new features
- Ensure all tests pass: `cargo test`
- Test on multiple platforms if possible

## Good First Issues

Look for issues labeled `good first issue` - these are great for newcomers!

## Questions?

- Join our [Discord](https://discord.gg/nova)
- Open a [Discussion](https://github.com/nova-container/core/discussions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
