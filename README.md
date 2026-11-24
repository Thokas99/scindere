<p align="center">
  <img src="docs/assets/scindere-mark.svg" width="96" alt="Scindere branching transcript mark">
</p>
<h1 align="center">Scindere</h1>
<p align="center"><strong>Resource-aware RNA splicing quantification for single-cell transcriptomics.</strong></p>
<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.89%2B-orange?logo=rust&logoColor=white" alt="Rust 1.89 or newer"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/Thokas99/scindere" alt="BSD 2-Clause license"></a>
  <a href="https://github.com/Thokas99/scindere/actions/workflows/rust.yml"><img src="https://github.com/Thokas99/scindere/actions/workflows/rust.yml/badge.svg?branch=main" alt="Scindere Rust CI"></a>
  <img src="https://img.shields.io/badge/version-0.1.0-42566c" alt="Development version 0.1.0">
  <img src="https://img.shields.io/badge/status-in%20development-7655a6" alt="In development">
</p>

Scindere is a Rust engine for producing spliced, unspliced and ambiguous molecule
count matrices from single-cell and single-nucleus sequencing alignments. It focuses
on bounded counting batches, recoverable count storage and streamed H5AD or Loom
output.

Scindere inherits velocyto's molecule-classification methodology. Its current
counting path uses bounded dense scratch arrays within each batch; sparse
checkpoints and sparse output storage do not mean that the whole computation is
sparse.

<p align="center">
  <img src="docs/assets/quantification-workflow.svg" alt="BAM, GTF and barcode inputs pass through alignment processing and molecule classification into bounded barcode batches, sparse checkpoints, and H5AD or Loom matrices with spliced, unspliced and ambiguous counts." width="100%">
</p>

## Key features

| Feature | Current capability |
|---|---|
| Resource-aware execution | Configurable counting buffers and bounded barcode batches |
| Sparse count storage | Sparse checkpoints and streamed output |
| Fault tolerance | Resumable molecule quantification |
| Platform compatibility | 10x Genomics and BD Rhapsody workflows |
| Native interoperability | H5AD and Loom output |
| Execution diagnostics | Structured progress and resource telemetry |

Barcode counting is sequential in this version. Global memory accounting, adaptive
batching and parallel barcode counting are planned in the [roadmap](ROADMAP.md).

## Installation

Scindere requires Rust 1.89 or newer and system HDF5 development files. Install
instructions, including Debian and Ubuntu packages, are in the [installation
guide](docs/installation.md).

```sh
cargo install --locked --git https://github.com/Thokas99/scindere --branch main
scindere --version
```

## Quick start

Quantify a BAM with cell barcodes in `CB` and UMIs in `UB`:

```sh
scindere run sample.bam genes.gtf --bcfile barcodes.tsv --cb-tag CB --ub-tag UB \
  --sampleid sample --outputfolder results
```

The command writes a `sample.h5ad` file by default and retains a resumable
`sample.counts` checkpoint. See the [usage guide](docs/usage.md) for 10x, BD
Rhapsody, DropEst and Smart-seq2 commands.

## Input and output formats

| Data | Format and use |
|---|---|
| Alignments | BAM input with cell-barcode and, when applicable, UMI tags |
| Annotation | GTF from the same genome reference used for alignment |
| Barcodes | Optional whitelist for generic BAM workflows; 10x reads the filtered list from the sample folder |
| H5AD | Default output; cell-by-gene sparse matrices with separate count layers |
| Loom | Optional gene-by-cell output; choose with `--output-format loom` or `both` |

Output details, checkpoint compatibility and count-layer semantics are in the
[output guide](docs/output.md).

## Documentation

- [Installation](docs/installation.md) · [Usage](docs/usage.md) ·
  [Configuration](docs/configuration.md)
- [Output and checkpoints](docs/output.md) ·
  [Troubleshooting](docs/troubleshooting.md) ·
  [Architecture](docs/architecture/overview.md)
- [Benchmarks](docs/benchmarks/README.md) · [Development roadmap](ROADMAP.md) ·
  [Contributing](CONTRIBUTING.md)
- [Provenance and project history](docs/provenance.md) · [Citation](CITATION.cff)

## Citation and acknowledgements

Scindere builds on the velocyto methodology, the `velocyto-rs` Rust implementation
by Zhuoqing Fang, and Johan Henriksson's original translation contributions.
Resource-aware batching, sparse checkpoints and streamed outputs were carried
forward from `velocyto-rs-lite`. Scindere does not introduce a new
molecule-classification algorithm; the full lineage and upstream license notes are
in [provenance](docs/provenance.md).

Please cite La Manno et al., [“RNA velocity of single
cells”](https://doi.org/10.1038/s41586-018-0414-6), *Nature* 560, 494–498 (2018),
and the software references in [CITATION.cff](CITATION.cff).

## License

Scindere is distributed under the BSD 2-Clause License. See [LICENSE](LICENSE) and
retain the upstream notices when redistributing.
