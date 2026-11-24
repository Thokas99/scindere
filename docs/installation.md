# Installation

Scindere is currently installed from the Git repository. The Cargo package is marked not publishable.

## Requirements

- Rust 1.89 or newer
- HDF5 development headers and libraries
- pkg-config on Unix-like build hosts

On Debian or Ubuntu:

```sh
sudo apt-get update
sudo apt-get install -y build-essential libhdf5-dev pkg-config
```

The `hdf5-metno` dependency links against system HDF5. This is required even when building with `--no-default-features`. The BAM feature uses noodles crates and does not require HTSlib.

## Install from Git

```sh
cargo install --locked --git https://github.com/Thokas99/scindere --branch main
scindere --version
```

To build and install from a local checkout:

```sh
cargo install --locked --path .
```

The executable is installed as `scindere`. The package also exposes the `scindere` Rust library.

## Build without BAM support

The no-default-features build compiles the dependency-light CLI and library path. BAM-based run commands are unavailable in this configuration.

```sh
cargo build --locked --no-default-features
cargo test --locked --no-default-features
```

## Apptainer

The repository includes an [Apptainer definition](https://github.com/Thokas99/scindere/blob/main/apptainer/scindere.def) for building a Linux image with HDF5 runtime libraries.
