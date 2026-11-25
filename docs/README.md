<div class="landing-hero">
  <img class="landing-mark" src="assets/scindere-mark.svg" alt="" width="104" height="66">
  <p class="landing-eyebrow">SCINDERE DOCUMENTATION</p>
  <h1>Scindere</h1>
  <p class="landing-tagline">RNA splicing quantification for single-cell data.</p>
  <p class="landing-intro">Count spliced, unspliced and ambiguous molecules from BAM alignments with a matching GTF and cell barcodes.</p>
  <div class="landing-actions">
    <a class="landing-button landing-button-primary" href="installation.html">Install</a>
    <a class="landing-button landing-button-secondary" href="usage.html">Run Scindere</a>
  </div>
</div>

## Count layers

Scindere applies velocyto-derived molecule classification and writes H5AD or Loom output. Count batches use dense scratch arrays; sparse checkpoints and streamed output keep completed batches small.

<div class="count-layers" role="group" aria-label="Output molecule count layers">
  <div class="count-layer"><span class="layer-symbol layer-spliced" aria-hidden="true">●</span><strong>Spliced</strong></div>
  <div class="count-layer"><span class="layer-symbol layer-unspliced" aria-hidden="true">◆</span><strong>Unspliced</strong></div>
  <div class="count-layer"><span class="layer-symbol layer-ambiguous" aria-hidden="true">■</span><strong>Ambiguous</strong></div>
</div>

## Guides

[Installation](installation.md) · [Usage](usage.md) · [Configuration](configuration.md) · [Output and checkpoints](output.md) · [Troubleshooting](troubleshooting.md)

## Development

[Development overview](development/index.md) · [Roadmap](development/roadmap.md) · [Rust API](api.md)

Scindere carries forward the velocyto classification method and prior Rust implementation work. See [provenance and citations](provenance.md).
