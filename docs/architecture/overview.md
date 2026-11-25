# Architecture overview

## Counting path

1. Read BAM alignments and the matching GTF; apply barcode and UMI settings.
2. Keep complete raw-barcode groups together in bounded count batches.
3. Classify molecule evidence into the requested layers using dense per-batch scratch arrays.
4. Convert each batch to sparse rows, commit a resumable checkpoint shard, then stream checkpoint data to H5AD or Loom.

The internal counter stores each molecule's evidence in a flat vector. It materializes the public `Molitem` representation for one molecule at classification, leaving the public `Logic::count` interface unchanged. The public `count()` API still collects all returned batches; `count_stream()` emits one batch at a time.

## Memory boundaries

- `--cell-batch-size` caps raw barcodes per batch. A barcode group is never split.
- `--count-memory-mb` estimates read, molecule-evidence and dense-scratch memory for counting. It is not a process-wide RSS limit; annotation, sorting, output buffers and runtime overhead add memory.
- Sparse checkpoint shards are durable count storage. Dense arrays remain the scratch representation during classification.
- Sorting uses separate thread and memory settings and consults host, cgroup and Slurm limits.

A group that exceeds the count estimate fails explicitly; previously committed checkpoints remain available. See [configuration](../configuration.md), [output and checkpoint recovery](../output.md#checkpoint-format-and-compatibility) and [benchmarks](../benchmarks/index.md) for the current measurement scope.

## Current execution limits

Barcode counting is sequential; Scindere does not currently schedule barcode
groups across parallel workers. The count-memory setting covers counting
components only, while sorting and output have separate resource use. The
implementation does not provide one process-wide memory cap or adaptive global
budget.
