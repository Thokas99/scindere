# Output formats and checkpoints

Scindere writes the same molecule counts to H5AD and Loom with format-specific matrix orientation.

| Format | Matrix orientation | Count layers |
|---|---|---|
| H5AD | Cells by genes; sparse CSR | `spliced`, `unspliced`, `ambiguous` |
| Loom | Genes by cells | `spliced`, `unspliced`, `ambiguous` |

H5AD is the default. Its `X` matrix stores the sum of the layers as `float32`; layer values are exact `uint32` counts. Loom uses `uint32` layers by default. The optional Loom `--dtype uint16` saturates layer values above 65,535; it does not narrow H5AD layers.

Checkpoints store sparse count batches and are streamed into either output. The counting implementation still uses bounded dense scratch arrays inside a batch. Sparse checkpoint and output storage should not be described as fully sparse computation.

## Checkpoint compatibility

The current checkpoint format is version 2. The reader also migrates version 1 checkpoints during resume. The Scindere rebrand did not change serialization or count settings; existing `velocyto-rs-lite` checkpoints remain readable when their inputs and settings match.

Resume checks canonical input paths, byte lengths, modification times and counting settings. These identity checks are not cryptographic content hashes. Keep the original inputs available when resuming an incomplete count.

For an interrupted run, pass the checkpoint path to `--resume-from`. To write output from a completed checkpoint, use `scindere write-output`. See the [checkpoint format notes](development/checkpoints.md) for shard validation and migration details.
