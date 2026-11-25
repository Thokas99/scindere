# Development roadmap

This page tracks implemented work, remaining work and the evidence required to close each item. “Partial” means useful parts are shipped while listed limits remain.

## 1. Global memory accounting and resource-aware execution — Partial

- **Implemented:** Fixed barcode batches, a counting-component estimate, separate sort resource heuristics, and RSS/progress telemetry.
- **Remaining:** One job budget for annotation, sorting, reads, molecule evidence, dense scratch and output; adaptive batch sizing.
- **Validation:** Compare estimates with RSS under cgroup and Slurm limits. Force budget failures, resume committed checkpoints, and compare exact counts across batch settings.

## 2. Compact molecule evidence — Partial

- **Implemented:** All three counter paths store per-molecule evidence in flat vectors and intersect sorted transcript indices. Count regressions cover eight logic modes, high-multiplicity UMIs and interleaved-sample resume. Public mapping and `Logic::count` APIs and checkpoint format v2 remain unchanged.
- **Evidence:** A 200,000-read synthetic case showed no measurable RSS change; see the [benchmark results](../benchmarks/index.md).
- **Remaining:** Measure on representative BAMs and under constrained cgroups; test large barcode groups and exact resumed output.

## 3. Memory-aware parallel barcode counting — Planned

- **Current state:** Barcode counting is sequential. Sorting and inherited numerical routines may use Rayon; the count loop has no worker scheduler or backpressure.
- **Next:** Schedule complete barcode groups against a shared memory budget and commit results in deterministic order.
- **Validation:** Compare exact counts, order, offsets and resume behavior across logic modes, worker counts and interruptions; verify bounded in-flight memory.

## 4. Complete preprocessing and counting recovery — Partial

- **Implemented:** Counting resume, writer locks, shard validation, v1-to-v2 checkpoint migration, and reuse of completed staged sorts.
- **Remaining:** Resume interrupted spill/merge stages and strengthen input identity beyond path, length and modification time.
- **Validation:** Interrupt each boundary, corrupt manifests and shards, change inputs, and compare resumed output with an uninterrupted run. Fail closed on stale or corrupt data.

## 5. Scientific diagnostics and reporting — Partial

- **Implemented:** Separate count layers, checkpoint totals and JSONL phase/resource telemetry.
- **Remaining:** Per-molecule rejection reasons, reconciled classification summaries and standardized software-version provenance in outputs.
- **Validation:** Reconcile diagnostics with checkpoint and matrix totals; verify known accepted and rejected molecules and H5AD/Loom metadata.

See [contributing](contributing.md) for validation commands, [output and checkpoint recovery](../output.md) for compatibility rules, and the [Rust API](../api.md) for library documentation. Archived implementation plans and design notes are in the [development archive](../superpowers/archive/index.md).
