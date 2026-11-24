# Checkpoint format and compatibility

Counting checkpoints are durable directories containing metadata, a manifest and compressed sparse count-batch shards. The manifest records source progress and batch row/layer totals. A writer lock prevents concurrent counting writers. Resume validates the saved metadata against current inputs and settings, then reads and validates each committed shard before continuing.

For input files, the current identity comprises canonical path, byte length and modification time in nanoseconds. This is not a cryptographic content digest. Shard validation checks the format marker, dimensions, row structure, layer nonzeros and totals; it is structural validation rather than a cryptographic checksum.

The current checkpoint format is version 2. The reader also accepts version 1 and migrates it to version 2 during resume. The Scindere rebrand changes only package, library, executable and documentation names. It leaves checkpoint serialization, default checkpoint paths and count settings unchanged, so existing `velocyto-rs-lite` checkpoints remain readable when their original input identities and settings still match.

Do not change the format version, serialized fields or resume rules as part of a name-only rebrand. Future incompatible changes need a versioned migration, fixtures for each supported version, and an explicit compatibility note before release.
