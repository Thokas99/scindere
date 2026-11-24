# Configuration and resource controls

The generic `run` command exposes count and sort controls. Specialized commands use their existing defaults.

| Option | Default | Effect |
|---|---:|---|
| `--cell-batch-size N` | 100 | Maximum raw barcodes per batch; a barcode group is kept intact |
| `--count-memory-mb N` | 512 | Estimated budget for read, molecule-evidence and dense scratch components |
| `--samtools-threads N` | 16 | Threads used by barcode sorting |
| `--samtools-memory N` | 2048 MiB | Sort memory setting per chunk |
| `--telemetry-jsonl PATH` | off | Append structured phase and progress events |

The count budget is not a whole-process memory cap. Annotation data, sorting, output buffers and runtime overhead use additional memory. Sorting applies host, cgroup and Slurm limits separately. See the [architecture overview](architecture/overview.md) for the current accounting boundaries.

A barcode group is never split between batches. If one group exceeds the count budget, the command reports an error and retains previously committed checkpoints. Lowering the batch size helps when batches contain multiple groups; increase the count budget when a single group is too large.

## Telemetry

The JSONL file records run phases, batch progress, sort progress and resource samples. Linux process RSS and high-water RSS are included when available. Telemetry is observational and does not alter molecule classification or output counts.
