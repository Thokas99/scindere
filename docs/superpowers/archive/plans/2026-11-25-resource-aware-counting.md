# Resource-Aware Counting Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Coordinate Scindere's sorting, counting, and output work against one conservative memory estimate, and adapt count batches without splitting raw-barcode groups.

**Architecture:** Add a private resource-budget module that reads Slurm, cgroup v2, and host memory signals and reports a conservative available batch budget. Pass that budget into the existing streaming counter, where read, evidence, dense scratch, and checkpoint-conversion estimates share the same ceiling; continue committing sparse checkpoint batches through the existing callback.

**Tech Stack:** Rust 1.89+, standard-library filesystem/environment access, existing `serde_json` telemetry, existing BAM/checkpoint/output code. Add no dependency.

**Spec:** `docs/superpowers/archive/specs/2026-11-24-resource-aware-counting-design.md`

## Global Constraints

- Preserve molecule-classification behavior and the existing run/write-output interface.
- Keep sparse checkpoint storage distinct from bounded dense scratch arrays in source comments and user documentation.
- Describe `--count-memory-mb` as a counting-component estimate, not a process-wide RSS limit.
- Preserve the upstream BSD 2-Clause notices and attribution in redistributed files and binaries.
- Keep checkpoint format version 2 unchanged; incompatible changes require a versioned migration and compatibility note.
- Do not publish to crates.io while `Cargo.toml` sets `publish = false`.
- Keep the 512 MiB `--count-memory-mb` default and the current CLI option name.
- Keep all raw-barcode groups intact; if one complete group cannot fit, fail and retain committed checkpoints.
- Keep existing JSONL event names and fields; telemetry additions are additive.
- Do not mark the roadmap item Complete without the constrained cgroup and scheduled representative BAM gates in the spec.

## Review Focus

- **Malformed or overflowing memory sources:** never turn invalid input into an unlimited budget; covered by resource-source unit tests.
- **Nested cgroup limits and changing usage:** choose the tightest applicable ancestor and reduce the next batch budget as outside usage rises; covered by resource-budget unit tests.
- **Slurm per-CPU requests with missing CPU counts:** reject the incomplete value and use the next safe source; covered by Slurm parsing tests.
- **A single oversized barcode group:** report a budget error without splitting it or removing prior checkpoints; covered by count regression tests.
- **Interleaved sample tags and resume lookahead:** keep all reads for one raw barcode together and preserve the next input cursor; covered by existing streaming/resume tests plus the adaptive-budget regression.

---

## File Structure

- Create `src/resource_budget.rs` for memory-source discovery, conservative limit selection, dynamic snapshots, and focused unit tests.
- Modify `src/lib.rs` to declare the private `resource_budget` module.
- Modify `src/commands/run.rs` to use the shared resource budget for the sorter and count/export phases, and remove the duplicate memory heuristic.
- Modify `src/counter.rs` to accept a dynamic internal budget provider while keeping public `count_stream` unchanged; check combined component estimates before growing evidence or dense scratch.
- Modify `src/commands/write_output.rs` and `src/count_output.rs` to use the same internal budget for checkpoint-backed export without changing public signatures.
- Modify `src/count_checkpoint.rs` to enforce an internal byte ceiling while decoding a sparse batch, preserving public `read_batch` behavior.
- Modify `src/telemetry.rs` to write an additive resource-budget event and retain existing event fields.
- Modify `src/count_regression.rs` for adaptive complete-group and checkpoint-resume coverage.
- Update `docs/configuration.md`, `docs/architecture/overview.md`, and `docs/development/roadmap.md` to describe the effective estimate and its limits.

## Tasks

### Task 1: Add conservative resource-source discovery

**Files:**
- Create: `src/resource_budget.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Produce `pub(crate) struct ResourceBudget` with `discover() -> Self`, `snapshot(phase_baseline_rss_bytes: u64) -> ResourceSnapshot`, and `sort_limit_mb(requested_mb: usize) -> usize`.
- Produce `pub(crate) struct ResourceSnapshot` with the selected source, allocation ceiling, reserve, and current safe batch capacity in bytes.
- Keep source parsing and capacity arithmetic in small pure helpers so tests can supply host, Slurm, and cgroup values without changing the process environment.

- [ ] **Step 1: Add failing tests** for Slurm per-node precedence, checked per-CPU conversion, tightest cgroup ancestor, `memory.high` as the preferred lower ceiling, reserve arithmetic, changing `memory.current`, malformed/overflowing values, and the conservative 512 MiB fallback when no source is usable.
- [ ] **Step 2: Run the focused resource-budget tests** and confirm they fail because the module and helpers do not exist.
- [ ] **Step 3: Implement source discovery and snapshots** in `src/resource_budget.rs`; walk cgroup v2 ancestors, sample `memory.current` again when creating a snapshot, apply the initial 15% reserve, use checked arithmetic, and fall back to host `MemAvailable` or 512 MiB when needed.
- [ ] **Step 4: Run the focused tests** and confirm all source-selection and arithmetic cases pass.

### Task 2: Use the shared budget across run phases and telemetry

**Files:**
- Modify: `src/commands/run.rs`
- Modify: `src/telemetry.rs`
- Test: `src/commands/run.rs` or `src/resource_budget.rs`

**Interfaces:**
- Consume `ResourceBudget::discover`, `snapshot`, and `sort_limit_mb` from Task 1.
- Keep `run_inner`, `run_inner_with_options`, `CountingOptions`, and the public command interface unchanged.
- Add a telemetry method that records selected source, ceiling, reserve, phase baseline, and effective budget as additive fields/event data.

- [ ] **Step 1: Add failing tests** showing sort memory is capped by the shared estimate and resource telemetry includes the selected source and effective ceiling without removing existing RSS fields.
- [ ] **Step 2: Run those focused tests** and confirm they fail against the current independent `read_mem_available_mb` heuristic.
- [ ] **Step 3: Replace `read_mem_available_mb` use** with the shared budget; use a fresh snapshot at the start of sort and counting, preserving the user-supplied sorter cap and thread limit.
- [ ] **Step 4: Emit resource telemetry additively** and retain the existing event names, fields, and best-effort failure behavior.
- [ ] **Step 5: Run focused run/telemetry tests** and confirm they pass.

### Task 3: Adapt counting batches against combined component estimates

**Files:**
- Modify: `src/counter.rs`
- Modify: `src/commands/run.rs`
- Test: `src/count_regression.rs`

**Interfaces:**
- Add a crate-private streaming entry point that accepts a `FnMut() -> usize` budget provider and optional existing progress callback.
- Route the CLI through that entry point in telemetry and non-telemetry modes; keep public `count_stream` and `count()` signatures unchanged by wrapping them with their existing fixed limit.
- Track read, evidence, dense scratch, cell-key, and dense-to-sparse conversion estimates under one limit. Keep estimates explicitly described as managed-component estimates, not RSS guarantees.

- [ ] **Step 1: Add failing regression coverage** for a budget that shrinks between complete raw-barcode groups, exact output equivalence across batch sizes, an oversized complete group, interleaved samples, and resume after a committed batch.
- [ ] **Step 2: Run the focused count regressions** and confirm the adaptive-budget cases fail against the fixed threshold.
- [ ] **Step 3: Add the dynamic internal budget path** and sample capacity at raw-barcode boundaries; when the estimate reaches the current limit, flush before the next raw barcode, never midway through the current barcode.
- [ ] **Step 4: Check combined usage before growth** in `charge_mapping` and before dense layer allocation in `finalize_batch`; include the read buffer and checkpoint conversion overlap in the estimate and report requested limit, effective limit, and exhausted component on failure.
- [ ] **Step 5: Run focused count regressions** and confirm exact layers, barcode identities, cursors, and resumability are unchanged.

### Task 4: Keep checkpoint-backed output within the shared estimate

**Files:**
- Modify: `src/commands/write_output.rs`
- Modify: `src/count_output.rs`
- Modify: `src/count_checkpoint.rs`

**Interfaces:**
- Keep public `export`, `write_h5ad`, and `write_loom` signatures unchanged; add private budget-aware paths used by `run_inner_with_options` and the existing public wrappers.
- Add `Checkpoint::read_batch_with_limit(index: usize, max_bytes: usize) -> Result<SparseBatch>` for budget-aware output; keep `read_batch(index)` as the existing public wrapper and preserve the checkpoint format.
- Continue reading sparse checkpoint batches, not dense whole-run matrices; do not change H5AD or Loom schemas.

- [ ] **Step 1: Add failing output/checkpoint tests** for identical H5AD/Loom metadata and matrix values across multiple checkpoint batches, empty batches, a small tile budget, and a sparse shard that exceeds its decode budget.
- [ ] **Step 2: Run the focused output tests** and confirm they fail for the new bounded-metadata/tile expectations.
- [ ] **Step 3: Stream cell IDs and sample labels per checkpoint batch** instead of retaining all of them across the full dataset; choose H5AD conversion chunks and Loom gene/cell tiles from the effective output budget.
- [ ] **Step 4: Check vector growth while decoding** in `read_batch_with_limit`, include the loaded sparse batch and conversion tile in output estimates, and fail clearly if one indivisible stored batch cannot fit, without altering checkpoint files.
- [ ] **Step 5: Run the output tests** and confirm exact schemas, cell/gene identities, and layer values remain unchanged.

### Task 5: Document the estimate and keep external gates visible

**Files:**
- Modify: `docs/configuration.md`
- Modify: `docs/architecture/overview.md`
- Modify: `docs/development/roadmap.md`

- [ ] **Step 1: Update documentation** to distinguish the shared allocation estimate from process RSS, explain the retained 512 MiB default and dynamic whole-barcode flush boundary, and keep the roadmap item Partial.
- [ ] **Step 2: Review the docs against the CLI help and implementation** so claims about cgroup/Slurm support and telemetry match code.
- [ ] **Step 3: Run the CONTRIBUTING.md validation commands** before proposing the implementation.
- [ ] **Step 4: Record the constrained cgroup and scheduled real-BAM validation as remaining gates**; do not claim full completion until the spec's matrix, resume, resource, and measurement criteria pass.
