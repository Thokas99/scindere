# Scindere roadmap

Status is assessed at the initial Scindere 0.1.0 development baseline. “Partial” means useful pieces exist, with stated limits; it does not imply the full contribution is complete. Priorities are ordered by implementation sequence.

## 1. Global memory accounting and resource-aware execution — Partial

- **Current limitation:** `--count-memory-mb` bounds estimated read, evidence and scratch components independently; it is not a whole-process limit and is not automatically reduced to fit available memory. Count batches flush on a fixed barcode ceiling or estimated byte threshold, but an oversized barcode group can fail. Sorting already consults host, cgroup and Slurm memory and thread limits.
- **Proposed solution:** Account for annotation, sorting, read, evidence, scratch and output memory under one job budget; use scheduler/cgroup limits consistently and tune batch size before work is committed.
- **Implementation status:** Component budgets, fixed batch limits, sort resource heuristics and RSS/progress telemetry are implemented. Shared accounting and adaptive counting batches are planned.
- **Validation requirements:** Exercise representative inputs under constrained cgroup and Slurm allocations; compare observed RSS with estimates; force budget exhaustion and verify committed checkpoints remain resumable; compare all count layers across batch settings.
- **Expected computational benefit:** Fewer avoidable out-of-memory failures and more predictable resource use on workstations and HPC jobs.
- **Potential impact on scientific correctness:** Resource changes must not split a barcode group, drop reads or silently skip an over-budget batch. Counts and cell/gene identities must remain identical across budgets.

## 2. Compact molecule-evidence storage — Partial

- **Current limitation:** Sparse checkpoints reduce retained count-matrix memory, but molecule evidence uses allocation-heavy maps and vectors. The GTF annotation remains resident, and each count batch still allocates dense scratch arrays.
- **Proposed solution:** Replace repeated per-molecule allocations with compact, reusable evidence buffers and explicit bounded scratch storage, while keeping sparse count shards as the durable representation.
- **Implementation status:** Sparse checkpoint shards, bounded barcode batches and streamed H5AD/Loom writers are implemented. Compact evidence storage and broader out-of-core annotation work are planned.
- **Validation requirements:** Compare molecule assignments and exact integer layers against the current implementation for every counting rule; measure allocation count, peak RSS and wall time on fixed synthetic and representative BAM fixtures; test high-multiplicity UMIs and large barcode groups.
- **Expected computational benefit:** Lower evidence-buffer memory and allocation overhead, especially for high-depth cells and large batches.
- **Potential impact on scientific correctness:** Evidence must preserve read-to-molecule grouping, UMI identity, strand handling and all classification rules. No approximation or evidence coalescing may alter counts.

## 3. Memory-aware parallel barcode counting — Planned

- **Current limitation:** Barcode count batches run sequentially. Rayon is used in sorting and inherited numerical routines, but not to count multiple barcode batches concurrently. There is no count-worker scheduler, backpressure or deterministic parallel reduction.
- **Proposed solution:** Add a bounded worker pool that schedules complete barcode groups against the shared memory budget, limits thread use, applies backpressure and commits results in deterministic order.
- **Implementation status:** Planned; no parallel barcode counting is claimed in this release.
- **Validation requirements:** Repeatedly compare serial and parallel counts byte-for-byte across logic modes, input order, worker counts and interruption points; stress memory limits and verify bounded in-flight work.
- **Expected computational benefit:** Better CPU utilization and lower wall time on multi-core hosts when memory allows concurrent batches.
- **Potential impact on scientific correctness:** Parallel reduction and commit ordering must not change layer values, barcode order, checkpoint offsets or resume behavior. Determinism is a release gate.

## 4. Complete preprocessing and counting recovery — Partial

- **Current limitation:** Counting checkpoints preserve committed sparse batches, validate shard structure and resume only when input path, size, modification time and settings match. These input checks are not content hashes. The sorter can reuse a verified sorted BAM or publish a completed staged sort after interruption, but a crash during spill or merge can require sorting again.
- **Proposed solution:** Persist recoverable preprocessing boundaries and validate them before reuse; strengthen input identity checks with content digests or another documented integrity scheme; make recovery behavior explicit for every stage.
- **Implementation status:** Counting resume, writer locking, v1-to-v2 checkpoint migration, shard validation and completed-sort recovery are implemented. Mid-sort resume and content-based input validation are planned.
- **Validation requirements:** Inject interruption after each preprocessing and counting boundary; corrupt manifests, shards and staged outputs; resume and compare exact count matrices with uninterrupted runs; verify changed inputs are rejected and existing checkpoints are not deleted on error.
- **Expected computational benefit:** Avoid repeating expensive sorting and counting work after node eviction, preemption or process failure.
- **Potential impact on scientific correctness:** Recovery must neither duplicate nor omit a source or batch. Reuse of stale or corrupted intermediates must fail closed; any stronger input identity must not silently accept changed biological data.

## 5. Scientific diagnostics and reporting — Partial

- **Current limitation:** Count layers are explicit, checkpoints store layer totals, and telemetry reports phases, progress, sort throughput and available Linux RSS/high-water RSS. Per-molecule rejection reasons, a reconciled classification summary and standardized software-version provenance in output files are not yet provided.
- **Proposed solution:** Report molecule-classification and rejection totals with documented denominators, resource summaries, stable execution provenance and validated AnnData/Loom metadata.
- **Implementation status:** Separate output layers, checkpoint totals, input/settings metadata and JSONL execution telemetry are implemented. Detailed scientific diagnostics and output provenance fields are planned.
- **Validation requirements:** Reconcile diagnostic totals with checkpoint and output matrix sums; test known synthetic molecules for every accepted and rejected class; read outputs with AnnData and Loom-compatible readers; verify repeated runs record comparable provenance without changing matrix contents.
- **Expected computational benefit:** Faster quality review and easier diagnosis of data, resource and pipeline problems; no direct count-speed claim.
- **Potential impact on scientific correctness:** Diagnostics must use precise definitions and denominators. They must not change classification or imply that rejected molecules belong to a particular layer without evidence.
