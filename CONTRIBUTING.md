# Contributing

Scindere is scientific software. Changes to molecule assignment, count layers, barcode grouping or output layout need regression evidence that count values and identifiers remain correct.

## Development setup

Use Rust 1.89 or newer. On Debian or Ubuntu, install `libhdf5-dev` and `pkg-config`, then build with:

```sh
cargo build
```

## Before submitting a change

Run the repository checks:

```sh
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features
cargo test --all-features
cargo build --release
cargo package --list
```

The regression suite includes generated BAM fixtures, checkpoint resume and corruption checks, and H5AD/Loom layer comparisons. Keep biological counts, gene IDs, cell IDs and checkpoint recovery behavior covered when changing those paths. Do not commit private or identifiable sequencing data; use small synthetic fixtures.

## Compatibility and attribution

See [checkpoint compatibility](docs/development/checkpoints.md) before touching checkpoint serialization. Keep the README and roadmap explicit about which capabilities are implemented. Preserve upstream notices and credit; see [Origins and acknowledgements](README.md#origins-and-acknowledgements).
