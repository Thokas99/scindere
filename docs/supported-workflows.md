# Supported Workflows

Scindere reads position-sorted BAM alignments and a GTF from the same genome
reference. The generic `run` command accepts one or more BAM files; specialized
subcommands locate or interpret platform-specific inputs.

| Workflow | Command | Main inputs |
|---|---|---|
| Generic BAM | `run` | BAM file(s), GTF and optional barcode whitelist |
| 10x Genomics | `run10x` | Cell Ranger sample folder and GTF |
| BD Rhapsody WTA | `run` | Bioproduct BAM, RSEC MEX archive and GTF |
| DropEst | `run-dropest` | DropEst-preprocessed BAM and GTF |
| Smart-seq2 | `run-smartseq2` | One position-sorted BAM per cell; no UMI counting |

## Generic BAM

Provide one or more position-sorted BAM files, the matching GTF and optionally
a barcode whitelist:

~~~sh
scindere run lane1.bam lane2.bam genes.gtf \
  --bcfile barcodes.tsv \
  --cb-tag CB --ub-tag UB \
  --sampleid sample --outputfolder results
~~~

If cell-barcode or UMI tags are omitted, Scindere uses its tag-detection
behavior. Use `--sample-tag` when a multi-sample BAM carries sample identity;
the sample is included in the cell identity. See [Quick Start](quick-start.md)
for expected output and recovery examples.

## 10x Genomics Cell Ranger

Pass the Cell Ranger sample folder containing `outs/` and its matching GTF.
Scindere locates the position-sorted BAM and filtered barcode file:

~~~sh
scindere run10x /data/cellranger/sample \
  genes.gtf --output-format h5ad
~~~

The default output folder is `<sample-folder>/velocyto` for compatibility.
Use the generic `run` command when you need to choose a custom output folder.
See the [10x example](https://github.com/Thokas99/scindere/blob/main/examples/10x/run.sh).

## BD Rhapsody WTA

Use the Bioproduct BAM, RSEC molecule MEX archive and GTF from the aligned
reference. The [BD Rhapsody example](https://github.com/Thokas99/scindere/blob/main/examples/bd-rhapsody/run.sh)
extracts the filtered barcode member and uses the BD barcode and UMI tags
`CB` and `MA`. For a multi-sample BAM with sample identities in `ST`, add
`--sample-tag ST` to the generic command.

## DropEst and Smart-seq2

`run-dropest` accepts DropEst-preprocessed BAM data. The separate
`dropest-bc-correct` command corrects barcodes from a two-column mapping.
`run-smartseq2` counts one position-sorted BAM per cell without UMIs.
Use `scindere --help` and each subcommand's `--help` for the current options.

## Slurm and Apptainer

The [Slurm example](https://github.com/Thokas99/scindere/blob/main/examples/slurm/scindere.sbatch)
sets scheduler resources, count batch controls and JSONL telemetry. Supply its
input paths with:

~~~sh
sbatch --export=ALL,\
BAM=/data/sample.bam,\
GTF=/ref/genes.gtf,\
BARCODES=/data/barcodes.tsv \
examples/slurm/scindere.sbatch
~~~

The repository also includes an [Apptainer definition](https://github.com/Thokas99/scindere/blob/main/apptainer/scindere.def)
for a Linux image with HDF5 runtime libraries. The `--count-memory-mb` setting
estimates counting components, not total job RSS; annotation, sorting, output
and runtime overhead need additional memory. See [Configuration](configuration.md).
