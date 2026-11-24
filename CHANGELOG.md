# Changelog

## Scindere 0.1.0 — initial development

- Establish Scindere as an independent project with its own Cargo package, Rust library and executable names.
- Preserve the existing counting interface, sparse checkpoint format and H5AD/Loom output behavior during the rebrand.
- Carry forward the existing bounded batch counting, sparse checkpointing, streamed export and telemetry implementation.

## Historical `velocyto-rs-lite` development

These entries describe the project state inherited by Scindere; they are not Scindere releases.

### 0.1.2

- Made `velocyto-rs-lite` the primary executable and retained `velocyto-rs` as a compatibility command.
- Added CI linting and Cargo package verification.
- Kept counting rules, checkpoint format and H5AD/Loom schemas unchanged.
- Added BAM sort progress reporting after the 0.1.2 release preparation.

### 0.1.1 — 2026-11-22

- Added Cargo installation instructions, BD Rhapsody WTA guidance and expanded attribution.

### 0.1.0

- Prepared the original `velocyto-rs-lite` Cargo package and documented BAM inputs.
