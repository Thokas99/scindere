# Release process

Scindere starts at development version 0.1.0. The repository workflow checks source quality, compilation, tests and package contents; it does not publish to crates.io or create GitHub releases.

Before a future release, verify the Scindere crate name directly with crates.io, review repository metadata and topics, run all commands in `CONTRIBUTING.md`, inspect `cargo package --list`, and update `CHANGELOG.md` and `CITATION.cff`. Create a release tag and publish only as a separate, explicit maintainer action. Do not add automatic Cargo publishing to CI without a later release-policy decision.

The old `velocyto-rs-lite` registry versions and repository are separate historical assets. Do not delete or archive the original GitHub repository as part of a Scindere release; retain its original license and attribution.
