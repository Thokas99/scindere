# Output and checkpoint recovery

Scindere writes spliced, unspliced and ambiguous molecule counts to H5AD or Loom.

| Format | Matrix orientation | Storage |
|---|---|---|
| H5AD (default) | Cells by genes; sparse CSR | `X` is the layer sum as `float32`; layers are exact `uint32` counts |
| Loom | Genes by cells | `uint32` layers by default; `--dtype uint16` saturates values above 65,535 |

Count batches use dense scratch arrays. Completed batches are stored as sparse
checkpoint shards and streamed to output; this does not make the counting
computation fully sparse.

## Checkpoint format and compatibility

A checkpoint is a durable directory containing metadata, a manifest and
compressed sparse count-batch shards. The manifest records source progress and
batch totals. A writer lock prevents concurrent counting writers. Resume
validates metadata and every committed shard before continuing.

The current format is version 2. The reader migrates version 1 checkpoints to
version 2 during resume. The Scindere rebrand did not change serialization,
default paths or count settings, so matching `velocyto-rs-lite` checkpoints
remain readable.

Resume compares canonical input paths, file sizes, modification times and
counting settings. These checks are not cryptographic content hashes. Shard
checks are structural validation, not cryptographic checksums. Keep the original
inputs and settings when resuming. Incompatible serialization changes require a
versioned migration and a compatibility note.

## Resume or export

Resume an interrupted run with the same inputs and options:

```sh
scindere run sample.bam genes.gtf --bcfile barcodes.tsv --cb-tag CB --ub-tag UB \
  --sampleid sample --outputfolder results --resume-from results/sample.counts
```

Export a completed checkpoint without reopening the BAM or GTF:

```sh
scindere write-output results/sample.counts --outputfolder results --output-format h5ad
```
