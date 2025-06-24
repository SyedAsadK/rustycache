# rustycache

**rustycache** is a Redis-clone attempt in Rust, built for learning and fun.  
This project aims to explore core concepts of cache servers, networking, and async programming using the Rust language.

## Features

- Basic Redis-like functionality (work in progress)
- Built on Rust's async ecosystem (`tokio`)
- Modern Rust (edition 2024)

## Getting Started

### Prerequisites

- [Rust](https://rust-lang.org/) (edition 2024 or later)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Installation

Clone the repository:

```bash
git clone https://github.com/SyedAsadK/rustycache.git
cd rustycache
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

## Project Structure

- `src/` - Source code for the cache server
- `Cargo.toml` - Project manifest, dependencies

## Dependencies

- [tokio](https://tokio.rs/) - Asynchronous runtime for Rust
- [tokio-macros](https://crates.io/crates/tokio-macros)

## Contributing

This project is intended for personal learning, but contributions, suggestions, or issues are welcome!

## License

This project is currently unlicensed.

---

> _A Redis clone attempt in Rust for learning/fun._  
> [SyedAsadK/rustycache](https://github.com/SyedAsadK/rustycache)
