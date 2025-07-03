# Coding Standards

This document outlines the coding standards and practices for the mermaid-parser project.

## Code Formatting

We use `rustfmt` to ensure consistent code formatting across the project. The configuration is defined in `.rustfmt.toml`.

### Running rustfmt

```bash
# Format all code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

## Linting

We use `clippy` for code linting with progressively stricter rules. The configuration is defined in `Cargo.toml` under `[lints.clippy]`.

### Running clippy

```bash
# Run clippy with configured rules
cargo clippy --all-targets --all-features

# Run clippy and treat warnings as errors (for CI)
cargo clippy --all-targets --all-features -- -D warnings
```

### Linting Rules

Currently enabled:
- `all` - All default clippy lints

To be enabled after codebase cleanup:
- `pedantic` - More strict style guidelines
- `nursery` - Experimental lints
- `cargo` - Cargo-specific lints

## Pre-commit Hooks

A pre-commit hook is configured to run formatting and linting checks before each commit. This ensures code quality standards are maintained.

The hook runs:
1. `cargo fmt --check` - Ensures code is properly formatted
2. `cargo clippy` - Checks for lint issues
3. `cargo test --no-run` - Ensures tests compile

## Code Style Guidelines

### General Principles

1. **Clarity over cleverness** - Write code that is easy to understand
2. **Consistent naming** - Use descriptive names following Rust conventions
3. **Error handling** - Use proper error types and avoid `unwrap()` in production code
4. **Documentation** - Document public APIs and complex logic
5. **Testing** - Write tests for all parsers and public functions

### Parser-specific Guidelines

1. **Token Types** - Define clear token enums for each parser
2. **Error Messages** - Provide helpful error messages with context
3. **AST Structures** - Keep AST nodes simple and well-documented
4. **Performance** - Consider performance for large diagrams
5. **Compatibility** - Maintain compatibility with Mermaid.js syntax

### Testing Standards

1. **Unit Tests** - Test individual parser functions
2. **Integration Tests** - Test complete diagram parsing
3. **File-based Tests** - Use rstest for comprehensive testing with real examples
4. **Error Cases** - Test error conditions and edge cases
5. **Performance Tests** - Benchmark critical paths

## Continuous Integration

CI runs the following checks on every pull request:
- Code formatting (`cargo fmt --check`)
- Linting (`cargo clippy -- -D warnings`)
- All tests (`cargo test`)
- Documentation build (`cargo doc`)

## Gradual Improvement

The codebase is being gradually improved to meet stricter linting standards. When working on a file:
1. Fix any existing clippy warnings in the code you're modifying
2. Add documentation for public items you're working with
3. Improve error handling where appropriate
4. Add tests for new functionality

## Contributing

When contributing to this project:
1. Run `cargo fmt` before committing
2. Address clippy warnings in your code
3. Write tests for new functionality
4. Update documentation as needed
5. Ensure the pre-commit hook passes