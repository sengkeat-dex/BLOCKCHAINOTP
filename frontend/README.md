# Blockchain OTP Frontend

This is the frontend application for the Blockchain OTP System, built with Rust and Yew using WebAssembly.

## Prerequisites

- Rust and Cargo
- Trunk
- wasm32-unknown-unknown target

## Setup

1. Install Rust: https://www.rust-lang.org/
2. Install Trunk: `cargo install trunk`
3. Add WebAssembly target: `rustup target add wasm32-unknown-unknown`

## Building and Running

```bash
# Build and serve the frontend
trunk serve
```

The application will be available at http://localhost:8080

## Project Structure

```
frontend/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── header.rs        # Application header
│   │   ├── otp_request_form.rs  # OTP request form
│   │   ├── otp_verify_form.rs   # OTP verification form
│   │   ├── status_display.rs    # Status display component
│   │   └── wallet_connector.rs  # Wallet connector component
│   ├── models/              # Data models and types
│   ├── services/            # API service layer
│   ├── utils/               # Utility functions
│   └── lib.rs              # Main application entry point
├── index.html              # HTML entry point
├── styles.css              # Global styles
├── metamask.js             # MetaMask integration JavaScript
├── Cargo.toml              # Rust package configuration
└── Trunk.toml              # Trunk build configuration
```

## Architecture

The frontend follows a component-based architecture with clear separation of concerns:

1. **Components** - UI elements that manage their own state
2. **Models** - Data structures and types
3. **Services** - API communication layer
4. **Utils** - Helper functions

This structure makes the codebase maintainable and scalable.

## MetaMask Integration

The application includes MetaMask integration for Ethereum wallet connectivity:

- Connect to MetaMask with a single button click
- Supports both Ethereum and Solana networks
- Handles account and network changes
- Provides clear user feedback during connection process

## Development

### Adding New Components

1. Create a new file in the `src/components/` directory
2. Export the component in `src/components/mod.rs`
3. Import and use the component in other files

### Adding New Services

1. Create a new file in the `src/services/` directory
2. Export the service in `src/services/mod.rs`
3. Use the service in components

### Styling

The application uses a single CSS file with:
- Component-specific classes
- Responsive design
- Consistent color scheme
- Accessible contrast ratios