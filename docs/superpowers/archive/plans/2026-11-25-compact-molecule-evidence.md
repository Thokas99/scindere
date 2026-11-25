# Compact Molecule Evidence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Reduce per-molecule evidence allocation overhead without changing velocyto-derived classification, public Rust APIs, or checkpoint data.

**Architecture:** Add a private sorted-vector representation for read mappings and molecule evidence. Share overlap and splice filtering between the public mapping path and the compact path, intersect transcript indices and concatenate segment evidence with the existing semantics, then materialize one public `Molitem` at a time for the unchanged `Logic::count` interface.

**Tech Stack:** Rust 1.89+, existing `FeatureIndex`, `Feature`, `SegmentMatch`, `Molitem`, and `Logic` types. Use standard-library vectors and ranges; add no dependency.

**Spec:** `docs/superpowers/archive/specs/2026-11-24-resource-aware-counting-design.md`

## Global Constraints

- Preserve molecule-classification behavior and the existing run/write-output interface.
- Keep sparse checkpoint storage distinct from bounded dense scratch arrays in source comments and user documentation.
- Describe `--count-memory-mb` as a counting-component estimate, not a process-wide RSS limit.
- Preserve the upstream BSD 2-Clause notices and attribution in redistributed files and binaries.
- Keep checkpoint format version 2 unchanged; incompatible changes require a versioned migration and compatibility note.
- Do not publish to crates.io while `Cargo.toml` sets `publish = false`.
- Keep the public `Molitem`, `SegmentMatch`, and `Logic::count` interfaces behaviorally and structurally compatible.
- Preserve composite sample/barcode/UMI grouping, outer transcript intersection, inner segment union, strand handling, and exact integer outputs.
- Do not mark the compact-evidence roadmap item Complete until matched memory and representative BAM evidence is recorded.

## Review Focus

- **A transcript is absent from one read mapping:** remove it from the molecule, matching outer-key intersection; pin with a compact-merge unit test.
- **A transcript appears in both mappings:** preserve both segment lists in order, matching the existing inner concatenation; pin with a compact-merge unit test.
- **Spliced segments near feature boundaries:** preserve `skip_makes_sense` and feature selection exactly; pin by comparing compact and public mappings.
- **Same barcode and UMI in separate samples:** keep the composite identity distinct; cover with the interleaved-sample count regression.
- **Discordant, stranded, unstranded, or high-multiplicity molecules:** preserve exact layer counts in all logic modes; cover with the existing eight-mode differential fixture plus high-depth cases.

---

## File Structure

- Create `src/compact_evidence.rs` for compact sorted read mappings, molecule intersection/union, conversion to public `Molitem`, and focused unit tests.
- Modify `src/lib.rs` to declare `compact_evidence` as a private module.
- Modify `src/indexes.rs` to share one overlap/filter traversal between the existing public `find_overlapping_ivls` API and a compact sorted-vector output path.
- Modify `src/counter.rs` to retain compact molecule evidence in all three strandedness paths and materialize one legacy `Molitem` at a time in `finalize_batch`.
- Modify `src/count_regression.rs` to compare exact layer output against the existing collecting path for every logic mode and resume boundary.
- Update `docs/development/roadmap.md` and `docs/architecture/overview.md` to describe the compact internal representation and keep the status Partial pending benchmarks.

## Tasks

### Task 1: Add compact evidence types and intersection semantics

**Files:**
- Create: `src/compact_evidence.rs`
- Modify: `src/lib.rs`

**Interfaces:**
- Use a crate-private `CompactSegmentMatch` alias for `SegmentMatch`; read its transcript index from `Feature.transcript_model_idx` instead of storing the same index twice per record.
- Store each compact molecule's sorted transcript set and one flat evidence vector. Append in place while the transcript set is stable; filter retained evidence when it shrinks.
- Consume the compact molecule into the existing public `Molitem` at classification. Reuse the evidence vector directly for a single transcript and group it only for multi-transcript molecules.

- [x] **Step 1: Add failing unit tests** for sorted transcript intersection, dropping a transcript absent from the next read, segment concatenation for shared transcripts, empty mappings, and round-trip materialization to a `Molitem` with equal keys and segment values.
- [x] **Step 2: Run the focused compact-evidence tests** and confirm they fail because the private representation does not exist.
- [x] **Step 3: Implement flat vector storage and a two-way merge**; keep segment order within each transcript and avoid per-transcript hash maps or per-molecule segment vectors.
- [x] **Step 4: Run the focused tests** and confirm the compact and public representations agree.

### Task 2: Share overlap filtering with a compact mapping output

**Files:**
- Modify: `src/indexes.rs`
- Modify: `src/compact_evidence.rs`
- Test: `src/indexes.rs`

**Interfaces:**
- Keep public `FeatureIndex::find_overlapping_ivls(&mut self, read: &Read) -> HashMap<usize, Vec<SegmentMatch>>` unchanged.
- Add crate-private `FeatureIndex::find_overlapping_ivls_compact(&mut self, read: &Read) -> Vec<CompactSegmentMatch>`.
- Factor the traversal so both methods apply the same max-segment-count and `skip_makes_sense` filtering.

- [x] **Step 1: Add a failing test** that feeds the same synthetic reads through both outputs and compares transcript indices, features, segment coordinates, splice flags, and mapping rejection at invalid splice boundaries.
- [x] **Step 2: Run the focused index test** and confirm the compact method is missing.
- [x] **Step 3: Implement one shared overlap/filter traversal** and have the public method build its current hash-map result while the compact method emits sorted flat evidence.
- [x] **Step 4: Run index tests** and confirm public output and compact output match.

### Task 3: Use compact evidence in all batch-counting paths

**Files:**
- Modify: `src/counter.rs`
- Test: `src/count_regression.rs`

**Interfaces:**
- Replace only the private batch-local `HashMap<String, Molitem>` with `HashMap<String, CompactMolitem>`; retain barcode/UMI key construction and all read sorting/filtering.
- Change the three private `count_cell_batch_*_inner` paths to request compact mappings and call `CompactMolitem::add_mapping`.
- Change private `finalize_batch` to call `materialize()` for one molecule and pass it to the existing `self.logic.count(...)` signature.

- [x] **Step 1: Add differential regression coverage** for all eight logic variants, high-multiplicity UMIs, interleaved samples and resume after a committed batch.
- [x] **Step 2: Run the focused regressions** on the existing map-based path before switching the counter.
- [x] **Step 3: Switch all three private batch paths** to compact evidence and consume each molecule at classification.
- [x] **Step 4: Update evidence accounting** to use compact vector capacity and transcript-group counts.
- [x] **Step 5: Run differential regressions** and confirm layers, cell/gene identities, order and resume behavior match.

### Task 4: Document scope and measure the representation change

**Files:**
- Modify: `docs/architecture/overview.md`
- Modify: `docs/development/roadmap.md`
- Modify: public README, mdBook navigation, development index and Rust API page
- Modify: docs workflow to generate and publish rustdoc
- Add: `examples/compact_evidence_bench.rs` and a benchmark record

- [x] **Step 1: Update concise user and developer docs**; document compact internal evidence, sparse checkpoints, dense scratch, the collecting API and generated Rust API reference.
- [x] **Step 2: Run the CONTRIBUTING.md validation commands** and build the mdBook site and rustdoc.
- [x] **Step 3: Benchmark compact and map-based counting** on the same synthetic input; record environment, dimensions, wall time and peak RSS.
- [x] **Step 4: Keep the roadmap item Partial** pending constrained-cgroup and representative BAM measurements.
