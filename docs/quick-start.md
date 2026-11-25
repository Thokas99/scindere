# Quick Start

This example counts molecules from a position-sorted BAM with a cell-barcode
tag, a UMI tag and a genome-matched GTF. For build requirements, see
[Installation](installation.md).

## Run a sample

With cell barcodes in `CB` and UMIs in `UB`:

~~~sh
scindere run sample.bam genes.gtf \
  --bcfile barcodes.tsv \
  --cb-tag CB --ub-tag UB \
  --sampleid sample \
  --outputfolder results
~~~

Scindere writes `results/sample.h5ad` by default and retains a resumable
`results/sample.counts` checkpoint. The H5AD matrix has cells as rows and genes
as columns, with spliced, unspliced and ambiguous counts in separate layers.
Choose Loom with `--output-format loom` or write both formats with
`--output-format both`.

## Resume or export

Resume an interrupted run using the same inputs and counting options:

~~~sh
scindere run sample.bam genes.gtf --bcfile barcodes.tsv --cb-tag CB --ub-tag UB \
  --sampleid sample --outputfolder results --resume-from results/sample.counts
~~~

Export a completed checkpoint without reopening the BAM, GTF or barcode file:

~~~sh
scindere write-output results/sample.counts --outputfolder results --output-format h5ad
~~~

For other input layouts, see [Supported Workflows](supported-workflows.md).
For memory estimates and sorting settings, see [Configuration](configuration.md).
Checkpoint details and compatibility are in [Output & Recovery](output.md).
