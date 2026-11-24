# Scindere

### Resource-aware RNA splicing quantification for large-scale single-cell transcriptomics.

Scindere is a Rust-based RNA molecule quantification engine designed to generate spliced, unspliced and ambiguous count matrices from single-cell and single-nucleus sequencing alignments. It focuses on scalable execution, memory-efficient count storage, fault tolerance and interoperability with modern single-cell analysis ecosystems.

The name comes from the Latin *scindere*, “to split” or “divide,” reflecting the classification of RNA molecules by splicing state.

## Why Scindere

Keeping a dense gene-by-cell matrix for each count layer in memory becomes expensive as datasets grow. Concatenating dense batches can also require old and new matrices to coexist, multiplying peak memory. A workflow that retains every dense matrix until export can therefore become limited by RAM even when most gene-cell entries are zero.

Scindere’s command-line counting path processes bounded barcode batches, writes completed counts to sparse checkpoints, and streams those checkpoints into H5AD or Loom output. Count matrices are stored sparsely between batches and in H5AD output. Counting still uses bounded dense scratch arrays within a batch; Scindere does not perform entirely sparse computation. The public library `count()` helper also retains its collecting behavior for callers that need dense arrays.

The `--count-memory-mb` option limits estimated read, molecule-evidence and scratch components. It is not a cap on total process memory: the annotation, sorting, runtime and output buffers add to process RSS. See the [architecture notes](docs/architecture/overview.md) for details.

## Five defining contributions

The status labels describe the current implementation at the initial Scindere baseline. More detail and validation criteria are in the [roadmap](ROADMAP.md).

### I. Resource-aware execution — Partial

- Bounded barcode batches and a configurable count-buffer budget.
- Sorting adjusts its thread and memory settings using host, cgroup and Slurm limits.
- Progress events report phase, batch, sort and process-memory telemetry.
- Global memory accounting and automatic barcode-batch tuning remain planned.

### II. Memory-efficient molecule quantification — Partial

- The CLI commits sparse count batches and streams H5AD and Loom output.
- Dense scratch matrices remain bounded to a counting batch.
- Molecule evidence still uses allocation-heavy Rust collections; a compact representation is planned.

### III. Scalable parallel execution — Planned

- Barcode counting is currently sequential. Rayon is used in sorting and other inherited numerical routines, but not to process barcode count batches in parallel.
- Memory-aware count workers, controlled thread use, backpressure and deterministic parallel reduction remain planned.

### IV. Fault-tolerant quantification — Partial

- Counting resumes from durable sparse checkpoints, validates checkpoint shards and input/settings identity, and retains committed batches after errors.
- The sorter can reuse or publish a completed staged sort after interruption. An interrupted spill or merge stage may need to run again.
- Complete preprocessing recovery and stronger input-content identity checks remain planned.

### V. Transparent scientific diagnostics — Partial

- Outputs preserve separate spliced, unspliced and ambiguous layers; H5AD and Loom writers have regression coverage.
- Telemetry reports progress, phase timing, sort throughput and Linux process RSS/high-water RSS when available.
- Per-molecule rejection summaries and standardized software-version provenance in output files are not implemented yet.

## Install

Scindere is in initial development and is not being published to crates.io as part of this rebrand. Install Rust 1.89 or newer. On Debian or Ubuntu, the build environment also needs `libhdf5-dev` and `pkg-config`.

```sh
git clone https://github.com/Thokas99/scindere.git
cd scindere
cargo install --locked --path .
scindere --version
```

To build without installing:

```sh
cargo build --locked --release
./target/release/scindere --help
```

The Apptainer definition is in [`apptainer/scindere.def`](apptainer/scindere.def).

## Run

Use the GTF annotation for the genome reference used to align each BAM.
For compatibility with existing workflows, `run10x` keeps the default output directory name `velocyto`; pass `--outputfolder` to choose another location.

### 10x Genomics Cell Ranger

```sh
scindere run outs/possorted_genome_bam.bam genes.gtf \
  --bcfile outs/filtered_feature_bc_matrix/barcodes.tsv.gz \
  --cb-tag CB --ub-tag UB \
  --sampleid sample_10x --outputfolder results/10x
```

### BD Rhapsody WTA

Use the matching `*_Bioproduct.bam` and `*_RSEC_MolsPerCell_MEX.zip`. The filtered MEX archive supplies the called barcodes; use the GTF from the same BD reference archive used for alignment.

```sh
unzip -p sample_RSEC_MolsPerCell_MEX.zip barcodes.tsv.gz > sample_called_barcodes.tsv.gz

scindere run sample_Bioproduct.bam genes.gtf \
  --bcfile sample_called_barcodes.tsv.gz \
  --cb-tag CB --ub-tag MA \
  --sampleid sample_bd --outputfolder results/bd
```

With UMI counting enabled, reads are grouped by the selected cell-barcode and UMI tags. The `--logic` option controls how molecule evidence is assigned to the `spliced`, `unspliced`, `ambiguous` and, for `Discordant10X`, `spanning` layers. These are molecule counts; Scindere does not estimate RNA-velocity vectors or fit downstream velocity models.

H5AD is the default output and preserves `uint32` counts. Use `--output-format loom` or `--output-format both` to select other outputs. Loom’s optional `--dtype uint16` saturates values above 65,535; sparse checkpoints retain exact `uint32` counts.

Useful resource and reporting options:

- `--cell-batch-size N`: maximum raw barcodes per batch; a barcode group is not split.
- `--count-memory-mb N`: estimated budget for each counting-buffer component, not total job memory.
- `--samtools-threads N` and `--samtools-memory N`: sorting thread and memory settings, constrained by detected host, cgroup and Slurm limits.
- `--telemetry-jsonl PATH`: append structured progress, phase, sort and memory events.

Run `scindere run --help` or `scindere --help` for all commands and options.

## Resume and export

Counting writes checkpoints by default under `OUTPUT/SAMPLE.counts`. Resume an interrupted run with its original inputs and counting settings:

```sh
scindere run sample.bam genes.gtf --resume-from results/sample.counts
```

Export a completed checkpoint without recounting:

```sh
scindere write-output results/sample.counts \
  --outputfolder results --output-format h5ad
```

### Existing checkpoint compatibility

The rebrand does not change the checkpoint schema or counting settings. Existing `velocyto-rs-lite` checkpoints using format version 2 remain readable and resumable by Scindere when the original input identities and settings match. The existing version 1 to version 2 migration remains in place. This claim covers the rebrand only; future schema changes must be versioned and documented.

## Origins and acknowledgements

Scindere originated from the [`velocyto-rs` codebase](https://github.com/zqfang/velocyto-rs) developed by Zhuoqing Fang, which is a Rust implementation of the established [`velocyto.py` project](https://github.com/velocyto-team/velocyto.py). The upstream `velocyto-rs` README also acknowledges Johan Henriksson’s original translation work.

The molecule-classification logic and splicing-state methodology are inherited from these upstream projects; they were not developed for Scindere. The bounded counting batches, sparse checkpoints and streamed output present at this baseline were added during the intervening `velocyto-rs-lite` fork and are inherited by Scindere. The initial Scindere rebrand itself does not claim a new classification algorithm.

We acknowledge the original velocyto authors and methodology, Zhuoqing Fang and the `velocyto-rs` implementation, and all contributors whose code is carried forward. The upstream projects and this checkout use the BSD 2-Clause license; the original license text and required notices remain in [`LICENSE`](LICENSE). See the original repositories’ [velocyto.py license](https://github.com/velocyto-team/velocyto.py/blob/master/LICENSE) and [`velocyto-rs` license](https://github.com/zqfang/velocyto-rs/blob/main/LICENSE).

The method is described by La Manno et al., [“RNA velocity of single cells”](https://doi.org/10.1038/s41586-018-0414-6), *Nature* 560, 494–498 (2018). See [`CITATION.cff`](CITATION.cff) for software and paper references.

## License

Scindere is distributed under BSD 2-Clause. Retain the notices and conditions in [`LICENSE`](LICENSE) when redistributing source or binaries.
