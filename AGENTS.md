# Scindere development notes

- Preserve molecule-classification behavior and the existing run/write-output interface unless a separate change requires otherwise.
- Keep sparse checkpoint storage distinct from bounded dense scratch arrays in source comments and user documentation.
- Describe --count-memory-mb as a counting-component estimate, not a process-wide RSS limit.
- Preserve the upstream BSD 2-Clause notices and attribution in redistributed files and binaries.
- The Scindere rebrand keeps checkpoint format version 2 unchanged; incompatible changes require a versioned migration and compatibility note.
- Keep README feature claims aligned with implementation and ROADMAP.md.
- Do not publish to crates.io while Cargo.toml sets publish = false.
- Run the validation commands in CONTRIBUTING.md before proposing changes.
