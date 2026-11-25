# Release process

Scindere is an unpublished development project. Cargo.toml sets `publish = false`, and CI does not publish packages or create GitHub releases.

Before any future release, explicitly decide whether registry publication is wanted, verify name availability, review repository metadata, run all checks in the [contributor guide](../../CONTRIBUTING.md), inspect `cargo package --list`, update CHANGELOG.md and CITATION.cff, and review [output and checkpoint compatibility](../../docs/output.md#checkpoint-format-and-compatibility). Create a version tag and publish only after that separate release decision.

The earlier velocyto-rs-lite package and repository have separate historical status. Preserve their attribution, BSD 2-Clause terms and local archival backup when maintaining Scindere.
