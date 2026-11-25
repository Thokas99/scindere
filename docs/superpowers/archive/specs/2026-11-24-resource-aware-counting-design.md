# Scindere allocation-aware counting and compact evidence

**Status:** Draft for user approval
**Decision:** Shared Slurm/cgroup-aware memory envelope and compact internal molecule evidence; preserve the public Rust API and checkpoint format.

## Intent

Close roadmap contributions 1, **Global memory accounting and resource-aware execution**, and 2, **Compact molecule-evidence storage**. This work targets users with constrained workstation memory and HPC allocations of 128 GiB or more. It should let a user request a useful count budget while preventing Scindere from treating host RAM or one component estimate as the job's safe memory limit.

The resource model is an estimate of Scindere-managed working memory. The operating system's cgroup limit remains the actual enforcement boundary; Scindere must not promise a strict RSS cap.

## Current state

- `--count-memory-mb` defaults to 512 MiB. Read buffers, molecule evidence and dense matrices each compare against that value separately, so their simultaneous peak can exceed it.
- Sorting independently consults host, Slurm and cgroup values. Annotation is resident during counting, and the count/output path has transient conversion buffers. No shared allocation budget coordinates those phases.
- Telemetry reports process RSS and peak RSS on Linux, but the count estimate only reports read storage and dense layers.
- Each molecule currently retains a transcript-keyed `HashMap<usize, Vec<SegmentMatch>>`. Each `SegmentMatch` owns a cloned `Feature`; merging read evidence allocates new hash maps and temporary key vectors.
- `Molitem` and `Logic::count` are public library APIs. The checkpoint format is version 2 and does not serialize molecule evidence.
- Counting flushes only at complete raw-barcode groups. This invariant is required because sample tags can interleave within a barcode and UMI deduplication must see every read for that raw barcode together.

## Goals

- Use the minimum applicable Slurm or cgroup allocation as the run envelope, including current cgroup usage where available; use host-available memory only as a fallback.
- Reserve explicit headroom for allocator fragmentation, runtime, BAM I/O and untracked allocations. Start validation with a 15% allocation reserve, then retain or adjust it based on constrained-run measurements.
- Coordinate sort, annotation, count and output phases against the same envelope. Include resident baseline and phase-specific buffers in the limit calculation.
- Treat the configured count limit as one combined batch working-set ceiling covering reads, compact molecule evidence, count matrices, barcode/cell keys, and dense-to-sparse/output conversion overlap.
- Adapt flush size to current safe capacity while keeping raw barcode groups intact. If a single complete group cannot fit, fail clearly and preserve already committed checkpoint shards.
- Replace internal allocation-heavy evidence retention with compact vectors and feature references, while keeping the public `Molitem`, `SegmentMatch`, `Logic` trait, CLI option names, and checkpoint serialization intact.
- Preserve molecule identity, transcript intersection, segment union, strand handling, and exact counts across every logic mode.
- Measure estimated memory alongside process RSS and cgroup usage so later reserve adjustments have evidence.

## Non-goals

- No changes to velocyto-derived molecule classification, gene assignment, UMI grouping, output schemas or checkpoint format.
- No parallel barcode counting, out-of-core GTF loading, new workflow, or new third-party dependency.
- No guarantee that a user-space estimate exactly equals RSS or controls unrelated processes in the same allocation.
- No performance claim until matched data runs establish it.
- Do not mark either roadmap item complete solely because code and unit tests pass; constrained allocation and representative BAM validation are release gates.

## Approaches considered

1. **Shared allocation envelope — selected.** Derive a run ceiling from Slurm and cgroup limits, account for phase baseline and live working set, and cap count batches at the smaller of the user limit and safe remaining capacity. This addresses the whole-run limitation and makes large allocations usable without treating the requested count budget as a process RSS guarantee.
2. **Counting-stage adaptive limit only.** Combine the read, evidence and dense estimates and adapt batches, but keep sorting and output accounting separate. This is a smaller change, but leaves roadmap item 1 only partially implemented.
3. **Replace the public evidence API directly.** Change `Molitem` and the `Logic` trait to consume a compact evidence type. This could avoid materializing legacy maps, but breaks public Rust library callers and is unnecessary for the requested internal memory reduction.

The user selected approach 1 and requested compact evidence with existing public interfaces and checkpoint format preserved. The third approach is rejected to avoid a public API break.

## Design

### Shared memory envelope

Introduce one internal budget model for the `run` lifecycle. Its limit is the minimum valid value from the active Slurm request and cgroup limit (`memory.max`, with `memory.high` treated as a lower preferred ceiling when set). Use cgroup `memory.current` to account for other current usage in the job. If those sources are unavailable, fall back to process RSS and host `MemAvailable`; retain a conservative fallback when neither is readable. Invalid or overflowing values must fail to a safe fallback rather than become an unlimited budget.

At phase entry, record the resident baseline. Compute managed capacity as the applicable allocation ceiling less baseline/current untracked usage and the safety reserve. Recheck the available envelope at batch boundaries so other allocations or changing cgroup usage reduce the next batch's budget. The initial reserve is 15% of the applicable allocation; validation must include lower and higher reserves to establish whether this is safe for supported workloads.

The sorter, count reader, evidence builder, dense count matrices, checkpoint conversion, and output writer use the shared model rather than independent interpretations of host memory. Count batches use the smaller of the remaining shared capacity and `--count-memory-mb`. Keep the current 512 MiB default and numeric CLI behavior; users on 128 GiB allocations can raise `--count-memory-mb`, while the shared envelope clamps it to safe capacity. This avoids silently increasing memory use for existing commands. The option remains a requested batch working-set ceiling, not a process-wide RSS promise.

Batch accounting must include the overlap that exists in practice: buffered reads remain resident while evidence is accumulated; evidence remains resident while dense layers are filled; and dense layers may coexist briefly with sparse checkpoint/output conversion. Count those components together instead of granting each component the full limit. Keep arithmetic checked/saturating as appropriate, and check capacity before growing compact evidence buffers where possible.

Telemetry records the effective allocation source and ceiling, reserve, phase baseline, effective count budget, estimated live batch components, process RSS/high-water RSS, and cgroup usage when available. Existing JSONL event names and fields remain available; new fields are additive.

### Compact internal evidence

Use a private batch representation built from standard-library vectors. Store transcript model indices in sorted order and segment evidence as compact records containing segment coordinates, a feature reference/index, and splice status. Resolve feature data from the resident annotation/index during classification rather than cloning a full `Feature` for every segment match.

Merge each read's sorted transcript mappings with a two-way intersection. For transcript indices present in both observations, append segment evidence in the same order as the existing union operation; discard indices absent from either observation. Preserve the current read-level overlap filtering and `skip_makes_sense` checks in one shared traversal so the public mapping method and compact internal path cannot drift scientifically.

At the internal classification boundary, materialize the existing public `Molitem` representation for one molecule at a time, invoke the unchanged `Logic::count` interface, then release that temporary representation. This preserves downstream Rust callers and limits legacy allocation overlap to one molecule. The combined memory budget must include that temporary materialization. Keep the existing public `Molitem` methods and mapping helpers available and behaviorally unchanged.

The compact representation is in-memory only. Checkpoint version, settings format, count layers, output metadata and serialization remain unchanged. Resume with the same inputs and settings must behave as before.

## Scientific correctness and compatibility

The transformation is representation-only. It must preserve:

- Composite sample/barcode/UMI grouping.
- The outer transcript-model intersection and inner segment-evidence union.
- Feature coordinates, exon/intron kind, validation state, transcript identity and splice flags.
- Forward, reverse, stranded, unstranded and discordant handling.
- Complete raw-barcode grouping and the existing failure behavior for a single over-budget group.
- Exact integer layer values, cell/gene identities, barcode order, checkpoint offsets and resume results.

No stale checkpoint may be removed or overwritten because a budget is exceeded. A resource failure must retain committed shards and report the requested budget, effective budget, and component that crossed it.

## Validation and completion gates

1. Unit-test the budget source selection, Slurm per-node/per-CPU conversion, cgroup ancestor limits/current usage, host fallback, overflow handling, reserve calculation and dynamic reduction when usage rises.
2. Unit-test compact mapping creation, sorted transcript intersection, segment concatenation, feature resolution and equivalence to the public mapping representation.
3. Run differential count tests for every logic mode and compare exact H5AD/Loom layers, barcodes, genes, checkpoint offsets and resumed results against the pre-change baseline.
4. Exercise high-multiplicity UMIs, interleaved sample tags, discordant/unstranded mappings, a barcode group at the limit, and an over-budget complete group.
5. In a constrained cgroup, compare the model's estimates with cgroup peak/current memory and process RSS; force a budget reduction and verify batches shrink without splitting barcodes or losing checkpoints.
6. On a scheduled Slurm allocation of at least 128 GiB and a representative real BAM/GTF/barcode set, record resource limit sources, allocation, effective budget, peak RSS/cgroup usage, exact matrix checksums, elapsed time and checkpoint resume behavior. Do not run this workload on a login node.
7. Benchmark the compact evidence path against the unchanged baseline on the same fixtures. Report allocation/peak-memory and wall-time measurements without inventing or extrapolating performance claims.

Mark roadmap items 1 and 2 **Complete** only after the constrained cgroup and scheduled real-data gates pass, exact counts and resume behavior match, and the measured headroom is acceptable. If the required Slurm dataset/allocation is unavailable, document the code/test completion but leave the roadmap status Partial.

## Calls left to the implementation plan

- Exact internal compact vector layout and the smallest shared traversal that serves both the public mapping API and compact counter path.
- How the existing sorter and streaming writers report phase budgets without changing their public APIs.
- The fixed minimum headroom floor in addition to the initial 15% reserve, selected from constrained-run measurements.
- The exact locally available real-data fixture and the scheduled Slurm allocation used for the final validation gate.

## Approval gate

This document describes the proposed work. Do not modify Rust code, checkpoint formats, CLI defaults or roadmap status until the user approves it. After approval, prepare a concrete implementation plan before changing code.
