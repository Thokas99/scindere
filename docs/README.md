<div class="landing-hero">
  <img class="landing-mark" src="assets/scindere-mark.svg" alt="Scindere branching transcript mark" width="88" height="56">
  <h1>Scindere</h1>
  <p class="landing-tagline">Resource-aware molecule quantification for RNA velocity workflows.</p>
  <p>Count spliced, unspliced and ambiguous molecules from BAM alignments and a matching GTF. Scindere does not infer velocity; its classification follows velocyto.</p>
  <div class="landing-actions">
    <a class="landing-button landing-button-primary" href="installation.html">Installation</a>
    <a class="landing-button landing-button-secondary" href="https://github.com/Thokas99/scindere">GitHub</a>
  </div>
</div>

## Core capabilities

- Sequential counting in bounded, complete-barcode batches with resumable sparse checkpoints.
- H5AD and Loom output for generic BAM, 10x, BD Rhapsody, DropEst and Smart-seq2 workflows.
- Resource estimates and run telemetry. `--count-memory-mb` estimates counting components, not process-wide RSS.

## Quick start

```sh
scindere run sample.bam genes.gtf --bcfile barcodes.tsv --cb-tag CB --ub-tag UB \
  --sampleid sample --outputfolder results
```
