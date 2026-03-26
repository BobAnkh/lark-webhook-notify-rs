# CONTRIBUTING

We'd be glad for you to contribute to our source code and to make this project better!

Feel free to submit a pull request or an issue, but make sure to use the templates.

It is **required to follow** the **`Language Style`** rules.

## Language Style

Files of different languages should be checked locally according to the following conventions.

Commits should be made after all checks pass or with additional clarifications.

### Rust

Run `cargo fmt`(rustfmt) to format the code.

Run `cargo clippy` to lint the code.

Follow the official [naming convention](https://rust-lang.github.io/api-guidelines/naming.html).

## Development Workflow

### Making Changes

1. **Create a feature branch**:

   ```bash
   git checkout -b your-feature-name
   ```

2. **Make your changes** following the coding guidelines below

3. **Test your changes**:

   ```bash
   cargo clippy --workspace
   cargo fmt --all
   ```

4. **Update documentation** if needed:

   ```bash
   # Update user guide
   cd guide && mdbook build
   ```

5. **Commit with descriptive messages (following conventional commits)**:

   ```bash
   git commit -m "feat(coordinator): add task priority scheduling

   - Implement priority queue for task scheduling
   - Add priority field to task submission API
   - Update database schema with migration

   Closes #123"
   ```

### Code Quality Standards

Before submitting a pull request:

1. **Format code**: `cargo fmt`
2. **Fix linting issues**: `cargo clippy`
3. **Run tests**: `cargo test`
4. **Security audit**: `cargo audit`
5. **Check documentation**: `cargo doc`

### Commit Message Guidelines

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring without functionality changes
- `test:` Adding or modifying tests
- `chore:` Maintenance tasks, dependency updates

**Examples**:

```
feat(client): add interactive task monitoring
fix(worker): resolve memory leak in task execution
docs(api): improve SDK examples and error handling
```

## Testing Guidelines

### Writing Tests

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test API endpoints and workflows
- **Documentation tests**: Ensure code examples work

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_submission() {
        // Test implementation
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_task_submission

# Integration tests only
cargo test --test integration

# With output
cargo test -- --nocapture

# Using nextest (faster)
cargo nextest run
```

## Documentation Standards

### Code Documentation

- Add rustdoc comments to public APIs
- Include examples in documentation
- Document error conditions and panics

````rust
/// Fluent builder for fully custom Lark interactive cards.
///
/// Produces a [`GenericCardTemplate`] via [`CardBuilder::build`].
///
/// # Example
///
/// ```
/// use lark_webhook_notify::{CardBuilder, ColorTheme, TextAlign, TextSize};
///
/// let card = CardBuilder::new()
///     .header("Deploy Complete", Some("success"), Some(ColorTheme::Green), None)
///     .markdown("**Service:** api v1.4", TextAlign::Left, TextSize::Normal)
///     .columns()
///         .column("Env", Some("prod"), "auto", 1)
///         .column("Region", Some("us-east-1"), "weighted", 1)
///     .end_columns()
///     .collapsible("Logs", "Deploy OK", false)
///     .build();
/// ```
///
/// # Panics
///
/// [`build`](CardBuilder::build) panics if [`columns`](CardBuilder::columns) was
/// called without a matching [`end_columns`](CardBuilder::end_columns).
pub struct CardBuilder {
    /// Language carried for consumers (e.g. `create_custom_template`).
    /// Translations are performed by callers (WorkflowTemplates, etc.) before
    /// passing strings into the builder methods — builder methods accept raw strings.
    language: LanguageCode,
    header: Option<Value>,
    elements: Vec<Value>,
    column_stack: Vec<Vec<Value>>,
}
````

### User Documentation

When adding new features, update:

- User guide in `guide/src/`
- API documentation and examples
- Configuration file examples
- Troubleshooting guide if applicable

## Performance Guidelines

### Optimization Principles

1. **Async/Await**: Use async for I/O operations
2. **Connection Pooling**: Reuse database connections
3. **Streaming**: Use streaming for large data transfers
4. **Caching**: Cache frequently accessed data
5. **Batch Operations**: Group operations where possible

### Profiling

```bash
# CPU profiling
cargo install flamegraph
cargo flamegraph --bin lark-webhook-notify

# Memory profiling
valgrind --tool=massif ./target/debug/lark-webhook-notify
```

## Getting Help

- **Questions**: Open a [Discussion](https://github.com/BobAnkh/lark-webhook-notify-rs/discussions)
- **Bug Reports**: Create an [Issue](https://github.com/BobAnkh/lark-webhook-notify-rs/issues)
- **Feature Requests**: Create an [Issue](https://github.com/BobAnkh/lark-webhook-notify-rs/issues) with the "enhancement" label

## Recognition

Contributors are recognized in:

- Release notes and changelog
- Contributors section of documentation
- GitHub contributor graphs

Thank you for contributing to this project! 🚀
