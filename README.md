<p align="center">
  <img src="docs/assets/scindere-mark.svg" width="88" alt="Scindere branching transcript mark">
</p>
<h1 align="center">Scindere</h1>
<p align="center"><strong>Resource-aware molecule quantification for RNA velocity workflows.</strong></p>
<p align="center">
  <a href="https://github.com/Thokas99/scindere/actions/workflows/rust.yml"><img src="https://github.com/Thokas99/scindere/actions/workflows/rust.yml/badge.svg?branch=main" alt="Scindere Rust CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/Thokas99/scindere" alt="BSD 2-Clause license"></a>
</p>

Scindere counts spliced, unspliced and ambiguous molecules from BAM alignments
using a genome-matched GTF. It quantifies molecules for RNA velocity workflows;
it does not infer velocity. Classification follows the velocyto methodology.

## Core capabilities

- Sequential counting in bounded, complete-barcode batches with resumable,
  sparse checkpoint format v2.
- H5AD and Loom outputs, with workflows for generic BAM, 10x, BD Rhapsody,
  DropEst and Smart-seq2 data.
- Resource estimates and run telemetry. `--count-memory-mb` estimates counting
  components; it is not a process-wide RSS limit. Annotation, sorting and output
  add memory use.

## Installation

Requires Rust 1.89 or newer, HDF5 development libraries, and `pkg-config` on
Unix-like systems. See the [installation guide](https://thokas99.github.io/scindere/installation.html).

```sh
cargo install --locked --git https://github.com/Thokas99/scindere --branch main
```

## Quick start

```sh
scindere run sample.bam genes.gtf --bcfile barcodes.tsv --cb-tag CB --ub-tag UB \
  --sampleid sample --outputfolder results
```

The command writes `sample.h5ad` by default and retains a resumable
`sample.counts` checkpoint. See the [user guide](https://thokas99.github.io/scindere/)
for platform options, configuration, output details and recovery.

## Attribution and license

The classification method follows velocyto and La Manno et al., [“RNA velocity
of single cells”](https://doi.org/10.1038/s41586-018-0414-6). Scindere carries
forward software and engineering work from [velocyto-rs](https://github.com/zqfang/velocyto-rs)
and velocyto-rs-lite. See [provenance and citations](https://thokas99.github.io/scindere/provenance.html)
for attribution to Zhuoqing Fang, Johan Henriksson and other contributors.

Scindere is distributed under the BSD 2-Clause License; see [LICENSE](LICENSE).
