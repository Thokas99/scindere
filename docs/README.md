<div class="landing-hero">
  <img class="landing-mark" src="assets/scindere-mark.svg" alt="" width="104" height="66">
  <p class="landing-eyebrow">SCINDERE DOCUMENTATION</p>
  <h1>Scindere</h1>
  <p class="landing-tagline">Resource-aware RNA splicing quantification for single-cell transcriptomics.</p>
  <p class="landing-intro">Scindere produces spliced, unspliced and ambiguous molecule counts from single-cell and single-nucleus sequencing alignments. It carries out velocyto-derived molecule classification with bounded barcode batches, resumable sparse checkpoints and streamed H5AD or Loom output.</p>
  <div class="landing-actions">
    <a class="landing-button landing-button-primary" href="installation.html">Install Scindere</a>
    <a class="landing-button landing-button-secondary" href="usage.html">Start a run</a>
  </div>
</div>

## From alignments to count matrices

<p class="workflow-intro">The workflow groups alignments by barcode, classifies molecule evidence, then writes the three count layers to an analysis-ready output.</p>

<ol class="workflow-grid" aria-label="Scindere quantification workflow">
  <li class="workflow-step">
    <span class="workflow-number">01</span>
    <h3>Inputs</h3>
    <p>BAM alignments, a genome-matched GTF and cell barcodes.</p>
  </li>
  <li class="workflow-step">
    <span class="workflow-number">02</span>
    <h3>Classify molecules</h3>
    <p>Process alignments and assign molecule evidence to splicing states.</p>
  </li>
  <li class="workflow-step">
    <span class="workflow-number">03</span>
    <h3>Count in batches</h3>
    <p>Use bounded dense scratch buffers and persist sparse count checkpoints.</p>
  </li>
  <li class="workflow-step">
    <span class="workflow-number">04</span>
    <h3>Write outputs</h3>
    <p>Stream count matrices to H5AD or Loom for downstream analysis.</p>
  </li>
</ol>

<div class="count-layers" role="group" aria-label="Output molecule count layers">
  <div class="count-layer"><span class="layer-symbol layer-spliced" aria-hidden="true">●</span><strong>Spliced</strong></div>
  <div class="count-layer"><span class="layer-symbol layer-unspliced" aria-hidden="true">◆</span><strong>Unspliced</strong></div>
  <div class="count-layer"><span class="layer-symbol layer-ambiguous" aria-hidden="true">■</span><strong>Ambiguous</strong></div>
</div>

## Explore the guides

<div class="guide-grid">
  <a class="guide-card" href="installation.html"><strong>Installation</strong><span>Build Scindere and check system requirements.</span></a>
  <a class="guide-card" href="usage.html"><strong>Usage</strong><span>Run generic BAM, 10x, BD Rhapsody and other workflows.</span></a>
  <a class="guide-card" href="configuration.html"><strong>Configuration</strong><span>Set counting buffers, checkpoints and output options.</span></a>
  <a class="guide-card" href="output.html"><strong>Output and checkpoints</strong><span>Understand matrix orientation, layers and recovery.</span></a>
</div>

Scindere stores counts sparsely between batches, while using bounded dense scratch arrays internally. Barcode counting is sequential in the current implementation; parallel barcode counting and global memory accounting are planned. See the [architecture overview](architecture/overview.html) and [development roadmap](development/roadmap.html) for implementation details and status.

## Origins and acknowledgements

Scindere builds on the velocyto methodology, the `velocyto-rs` implementation by Zhuoqing Fang, and translation contributions by Johan Henriksson. Its current counting and file-format behavior inherits work from these projects and `velocyto-rs-lite`; Scindere does not introduce a new molecule-classification algorithm. Read the [full project provenance](provenance.html) and [citation information](https://github.com/Thokas99/scindere/blob/main/CITATION.cff).
