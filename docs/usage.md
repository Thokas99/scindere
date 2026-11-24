# Usage

Run `scindere --help` or a subcommand's `--help` flag to inspect the installed CLI.

## Generic BAM input

Supply one or more position-sorted BAM files, a genome-matched GTF, and optionally a barcode whitelist:

```sh
scindere run lane1.bam lane2.bam genes.gtf --bcfile barcodes.tsv --cb-tag CB --ub-tag UB --sampleid sample --outputfolder results
```

If cell or UMI tags are omitted, Scindere uses its tag-detection behavior. Use `--sample-tag` when a multi-sample BAM carries a sample identity tag; the sample is included in the cell identity.

## 10x Genomics Cell Ranger

Give `run10x` the Cell Ranger sample folder containing `outs/` and the matching GTF. It locates the position-sorted BAM and filtered barcodes.

```sh
scindere run10x /data/cellranger/sample genes.gtf --output-format h5ad
```

The default output folder is `<sample-folder>/velocyto` for compatibility. Use the generic `run` command when a custom output folder is required.

## BD Rhapsody WTA

Use the matching Bioproduct BAM, RSEC molecule MEX archive, and GTF from the aligned reference. The example script extracts the filtered barcode member and passes the BD barcode and UMI tags.

[BD Rhapsody example script](../examples/bd-rhapsody/run.sh).

For a multi-sample BAM, add `--sample-tag ST` if the BAM carries sample identities in `ST`.

## Other supported workflows

- `run-dropest` accepts DropEst-preprocessed BAM data; `dropest-bc-correct` corrects barcodes from a two-column mapping.
- `run-smartseq2` counts one position-sorted BAM per cell without UMIs.
- See [the 10x example](../examples/10x/run.sh) and [Slurm example](../examples/slurm/scindere.sbatch) for runnable scripts. Submit the Slurm script with:

```sh
sbatch --export=ALL,BAM=/data/sample.bam,GTF=/ref/genes.gtf,BARCODES=/data/barcodes.tsv examples/slurm/scindere.sbatch
```

## Resume and export

Counting writes resumable checkpoints by default. Resume with the same inputs and counting options used for the original run:

```sh
scindere run sample.bam genes.gtf --bcfile barcodes.tsv --cb-tag CB --ub-tag UB --sampleid sample --outputfolder results --resume-from results/sample.counts
```

A completed checkpoint can be exported without reopening the BAM or GTF:

```sh
scindere write-output results/sample.counts --outputfolder results --output-format h5ad
```

See [configuration](configuration.md) for resource controls and [output](output.md) for count layers and checkpoint compatibility.
