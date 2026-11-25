# Contributing

Scindere is scientific software. Changes to molecule assignment, count layers, barcode grouping, checkpoints or output layout need regression evidence for count values and identifiers.

## Development setup

Use Rust 1.89 or newer. The build and runtime require system HDF5 libraries; Unix build hosts also need pkg-config. On Debian or Ubuntu:

```sh
sudo apt-get install -y build-essential libhdf5-dev pkg-config
```

## Validation

Run these checks before proposing a change:

```sh
cargo fmt --all -- --check
cargo check --locked --all-targets --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo check --locked --all-targets --no-default-features
cargo test --locked --no-default-features
cargo build --locked --release
cargo doc --locked --no-deps --all-features
cargo package --list
```

## Documentation site

Install the mdBook version used by CI:

~~~sh
cargo install --locked --version 0.5.4 mdbook
~~~

Build the book and place the generated Rust API beside it:

~~~sh
cargo doc --locked --no-deps --all-features
mdbook build --dest-dir target/mdbook
mkdir -p target/mdbook/api
cp -a target/doc/. target/mdbook/api/
~~~

The published [Rust API reference](docs/api.md) is generated from the public crate.

The library keeps a scoped Clippy allowlist for inherited translation-era lints. Keep it limited to the documented modules and remove allowances when those areas receive behavior-focused cleanup.

The regression suite covers generated BAM fixtures, checkpoint resume and corruption, and exact H5AD/Loom layer values. Do not commit private or identifiable sequencing data; use synthetic fixtures.

## Compatibility and attribution

Read [output and checkpoint recovery](docs/output.md#checkpoint-format-and-compatibility) before changing serialization. Keep feature claims in the project README aligned with the implementation and [roadmap](ROADMAP.md). Preserve upstream notices and scientific credit; see [Citation & Credits](docs/citation.md).

Scindere is not currently publishable to crates.io. Do not change Cargo.toml publication settings or create releases as part of a routine code change.
