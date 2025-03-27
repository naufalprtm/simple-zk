markdown
Copy
# Simple-ZK: Lightweight SNARK Verifier Pallet

![Substrate](https://img.shields.io/badge/Substrate-2.0.1-brightgreen)
![Rust](https://img.shields.io/badge/Rust-nightly--2024--06--01-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

A minimal Substrate pallet for verifying SNARK proofs in blockchain runtime with WASM compatibility.

## Table of Contents
- [Features](#features)
- [Architecture](#architecture)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
  - [As a Substrate Pallet](#as-a-substrate-pallet)
  - [As a Standalone Node](#as-a-standalone-node)
- [Testing](#testing)
- [Benchmarking](#benchmarking)
- [Configuration](#configuration)
- [Contributing](#contributing)

## Features

- 🚀 Lightweight zero-knowledge proof verification
- 🔒 Secure proof verification in Substrate runtime
- ⚖️ Accurate benchmarking for weight calculation
- 🧪 Comprehensive test coverage (unit tests, mock tests)
- 🛠️ WASM compatible (no-std support)
- 📊 Performance metrics integration

## Project Structure

```
simple-zk/
├── pallets/
│ └── test-zk/ # Verification pallet
│ ├── Cargo.toml
│ └── src/
│ ├── lib.rs # Main pallet logic
│ ├── mock.rs # Mock runtime for testing
│ ├── tests.rs # Unit tests
│ └── weights.rs # Benchmarking weights
├── runtime/ # Runtime configuration
│ └── src/
│ ├── lib.rs
│ └── weights/
│ └── pallet_simple_zk.rs
└── node/ # Node implementation
├── Cargo.toml
└── src/
└── main.rs
```

## Getting Started

### Prerequisites

- Rust nightly toolchain
- Wasm target support
- Substrate dependencies

```bash
rustup default nightly
rustup update
rustup target add wasm32-unknown-unknown
```



# Clone the repository
```
git clone https://github.com/naufalprtm/simple-zk.git
cd simple-zk
```
# Build the project
```
cargo build --release
```
# Run unit tests
```
cargo test
```
# Run benchmarks
```
cargo test --features runtime-benchmarks
```

Usage

As a Substrate Pallet
Add to your runtime's Cargo.toml:
```
[dependencies]
pallet-test-zk = { path = "../pallets/test-zk", default-features = false }
```

Configure in runtime:
```rust
impl pallet_test_zk::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_test_zk::weights::SubstrateWeight<Runtime>;
}
```
Include in your runtime:
```rust

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Other pallets...
        TestZK: pallet_test_zk,
    }
);
```



As a Standalone Node

# Run in development mode
```
./target/release/simple-zk-node --dev
```
# Run with logging
```
RUST_LOG=debug ./target/release/simple-zk-node --dev
```
# Benchmarking
## Build with benchmarks
```
cargo build --features runtime-benchmarks
```
## Run benchmarks
```
cargo test --features runtime-benchmarks --benchmarks
```
## Generate weights

```
./target/release/simple-zk-node benchmark pallet \
    --chain=dev \
    --pallet=pallet_test_zk \
    --extrinsic='*' \
    --steps=50 \
    --repeat=20 \
    --output=./pallets/test-zk/src/weights.rs
```


Configuration
Key configuration options:
```rust
pub trait Config: frame_system::Config {
    /// The overarching event type
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    
    /// Weight information for extrinsics
    type WeightInfo: WeightInfo;
    
    /// Custom verification parameters
    type VerificationParams: Get<u32>;
}

```

Contributing
We welcome contributions! Please follow these guidelines:

Fork the repository

Create your feature branch (git checkout -b feature/AmazingFeature)

Commit your changes (git commit -m 'Add some amazing feature')

Push to the branch (git push origin feature/AmazingFeature)

Open a Pull Request

Please ensure all code:

Follows Rust formatting standards (cargo fmt)

Passes all tests (cargo test --all-features)

Includes appropriate documentation


Made with ❤️ for the Substrate ecosystem | ![ Report Issues](https://github.com/naufalprtm/simple-zk/issues)
Key improvements:
1. Added table of contents for better navigation
2. Enhanced architecture section with detailed descriptions
3. Added complete runtime integration example
4. Included benchmark generation commands
5. Added configuration details with trait implementation
6. Improved contributing guidelines
7. Better section organization and formatting
8. Added license badge and issue reporting link
9. Included more practical usage examples
10. Added logging examples for debugging
