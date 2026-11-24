# Architecture overview

Scindere preserves the existing molecule-counting pipeline and changes the amount of count data retained at once. The rebrand does not change the classification rules or output schemas.

## Counting path

1. The CLI reads BAM alignments and the supplied gene annotation, applies barcode/tag settings and groups complete raw-barcode groups.
2. It collects a bounded batch of reads and molecule evidence. Batch flushing is controlled by `--cell-batch-size` and an estimated byte threshold; a barcode group is never split.
3. The counter builds dense per-batch scratch arrays for the requested count layers. The `--count-memory-mb` budget applies to estimated read, evidence and scratch components, not the entire process.
4. A completed batch is converted to sparse rows and committed as a durable checkpoint shard with its input cursor and batch totals.
5. The H5AD writer creates sparse CSR matrices and writes checkpoint batches incrementally. The Loom writer writes bounded tiles from the same sparse checkpoint data.

Sparse storage is used for durable count batches and H5AD matrices. Dense arrays remain in use inside each count batch, and the public library `count()` method retains a collecting API that can hold dense results. GTF data, BAM sorting buffers and runtime overhead also contribute to process memory.

## Resource controls

- `--cell-batch-size` caps raw barcodes in a batch; actual batch size may be lower when the estimated byte threshold is reached.
- `--count-memory-mb` is passed as a byte budget to the counting read/evidence/scratch path. A single large barcode group can exceed it; Scindere reports an error and retains committed checkpoints.
- Sorting uses a separate memory/thread setting and consults host memory, cgroup limits and Slurm memory environment variables.
- JSONL telemetry reports phase and count/sort progress. On Linux it samples process RSS and high-water RSS from `/proc/self/status`; those values may be unavailable elsewhere.
- Barcode counting is sequential in this version. The parallel sorter and inherited numerical routines do not make the count loop parallel.

## Interoperability

H5AD output uses cell-by-gene sparse CSR matrices and separate count layers. Loom output uses gene-by-cell layout and bounded tile writes. Regression tests compare exact integer layer counts and read both output layouts. The stored checkpoint is independent of the Rust package name.
