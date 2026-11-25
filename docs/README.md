<div class="home-hero">
  <div class="home-brand">
    <img class="home-mark" src="assets/scindere-mark.svg" alt="Scindere branching transcript mark" width="120" height="75">
    <div class="home-brand-copy">
      <p class="home-kicker">Single-cell transcriptomics</p>
      <h1>Scindere</h1>
    </div>
  </div>
  <div class="home-introduction">
    <p class="home-tagline">Resource-aware RNA splicing quantification for single-cell transcriptomics.</p>
    <p>Count spliced, unspliced and ambiguous molecules from sequencing alignments and a matching GTF. Scindere supports RNA velocity analysis; it does not infer velocity. Molecule classification follows the velocyto methodology.</p>
    <div class="home-actions">
      <a class="home-button home-button-primary" href="installation.html">Install Scindere</a>
      <a class="home-button home-button-secondary" href="quick-start.html">Run a first sample</a>
    </div>
  </div>
</div>

## From alignments to count matrices

<p class="home-section-intro">A run moves from tagged alignments to three molecule-count layers, with committed batches available for recovery.</p>

<ol class="home-workflow" aria-label="Scindere quantification workflow">
  <li><h3>Prepare inputs</h3><p>BAM alignments, a genome-matched GTF and cell barcodes.</p></li>
  <li><h3>Classify molecules</h3><p>Group by barcode and apply velocyto-derived splicing logic.</p></li>
  <li><h3>Count and checkpoint</h3><p>Keep whole barcode groups in bounded batches; store committed counts sparsely.</p></li>
  <li><h3>Write matrices</h3><p>Stream H5AD or Loom output with spliced, unspliced and ambiguous layers.</p></li>
</ol>

## Designed around practical count runs

<div class="home-feature-grid">
  <article class="home-feature">
    <h3>Bounded counting</h3>
    <p>Configure barcode batch size and a counting-component memory estimate. Dense arrays remain bounded scratch; the estimate is not a process-wide RSS cap.</p>
  </article>
  <article class="home-feature">
    <h3>Recoverable progress</h3>
    <p>Durable sparse checkpoint shards support validated resume. Completed runs can be exported without reopening the source BAM or GTF.</p>
  </article>
  <article class="home-feature">
    <h3>Research-ready outputs</h3>
    <p>Use H5AD or Loom with separate count layers, and run generic BAM, 10x, BD Rhapsody, DropEst or Smart-seq2 workflows.</p>
  </article>
</div>

<dl class="home-facts">
  <div><dt>Sequencing workflows</dt><dd>Generic BAM · 10x · BD Rhapsody · DropEst · Smart-seq2</dd></div>
  <div><dt>Output formats</dt><dd>H5AD by default · Loom · both</dd></div>
  <div><dt>Research computing</dt><dd>Slurm and Apptainer examples, plus structured run telemetry</dd></div>
</dl>

## Install and run

Scindere requires Rust 1.89 or newer and system HDF5 development libraries.
The [installation guide](installation.html) lists platform packages.

~~~sh
cargo install --locked \
  --git \
  https://github.com/Thokas99/scindere \
  --branch main
~~~

For a generic BAM with cell barcodes in `CB` and UMIs in `UB`:

~~~sh
scindere run sample.bam genes.gtf \
  --bcfile barcodes.tsv \
  --cb-tag CB --ub-tag UB \
  --sampleid sample \
  --outputfolder results
~~~

The default output is `results/sample.h5ad`; the `results/sample.counts`
checkpoint can be resumed or exported later. Read [Quick Start](quick-start.html)
for the full command flow and [Supported Workflows](supported-workflows.html)
for platform-specific inputs and HPC examples.

## Benchmark evidence

Median of five fresh-process runs on a synthetic 200,000-read fixture:

| Counting path | Count time | Peak RSS |
|---|---:|---:|
| Map-based baseline | 0.121 s | 97,612 KiB |
| Compact molecule evidence | 0.112 s | 97,684 KiB |

The 9 ms timing difference is specific to this small fixture; the RSS
difference is within run-to-run noise. These measurements do not establish
production performance or whole-job memory use. The [benchmark page](benchmarks/index.html)
documents the setup and earlier storage measurements.

## Explore the documentation

Start with [installation](installation.html), [configuration](configuration.html),
[output and recovery](output.html) or [troubleshooting](troubleshooting.html).
The [architecture overview](architecture/overview.html), [benchmarks](benchmarks/index.html)
and [Rust API](api.html) provide implementation and measurement details.

## Citation and credits

Scindere carries forward Rust implementation and engineering work from
velocyto-rs and velocyto-rs-lite. The splicing classification method is inherited
from velocyto; Scindere does not introduce a new classifier. Read
[Citation & Credits](citation.html) for scientific citations, software lineage
and license notices.
