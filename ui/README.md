# Zibelina Store UI

Frontend application built with Yew and WebAssembly.

## Technology Stack

- [Yew](https://yew.rs/) - Rust/WebAssembly framework
- [Trunk](https://trunkrs.dev/) - WASM application bundler
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - WebAssembly bindings

## Development Setup

### Prerequisites

```bash
# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk wasm-bindgen-cli
```

### Running Locally

```bash
# Start development server
trunk serve
# or
make dev

# Run tests
cargo test
# or
make test
```

The UI will be available at `http://localhost:8080`

### Building

```bash
# Development build
trunk build

# Production build
trunk build --release
# or
make build
```

### Project Structure

```
ui/
├── src/
│   ├── components/   # Reusable UI components
│   ├── pages/        # Page components
│   ├── api/          # API client code
│   ├── hooks/        # Custom hooks
│   ├── state/        # Application state management
│   ├── types/        # Type definitions
│   ├── utils/        # Utility functions
│   ├── app.rs        # Main application component
│   └── main.rs       # Application entry point
├── static/           # Static assets
├── styles/           # CSS/SCSS files
├── tests/            # Test files
├── Cargo.toml        # Dependencies
├── index.html        # HTML template
└── README.md         # This file
```

### Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use conventional commits (`feat:`, `fix:`, etc.)
- Document all public functions and types

### Testing

```bash
# Run tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test

# Run specific test
cargo test test_name
```

### Best Practices

1. Component Design:
   - Keep components small and focused
   - Use props for component configuration
   - Implement proper cleanup in `use_effect`

2. State Management:
   - Use `use_state` for local state
   - Use `use_reducer` for complex state
   - Consider using contexts for global state

3. Error Handling:
   - Handle all Result/Option types
   - Display user-friendly error messages
   - Log errors for debugging

4. Performance:
   - Use `use_memo` for expensive computations
   - Implement proper caching strategies
   - Lazy load components when possible

## Contributing

1. Follow existing code style
2. Add tests for new features
3. Update documentation
4. Use feature branches
5. Submit pull requests