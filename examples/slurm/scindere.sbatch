#!/usr/bin/env bash
#SBATCH --job-name=scindere
#SBATCH --cpus-per-task=8
#SBATCH --mem=64G
#SBATCH --time=12:00:00
#SBATCH --output=scindere-%j.log

set -eo pipefail

if [[ -z "$BAM" || -z "$GTF" || -z "$BARCODES" ]]; then
  printf 'Set BAM, GTF and BARCODES with sbatch --export.\n' >&2
  exit 64
fi

out="results/scindere-$SLURM_JOB_ID"
scindere run "$BAM" "$GTF" --bcfile "$BARCODES" --sampleid "scindere-$SLURM_JOB_ID" --outputfolder "$out" --cell-batch-size 128 --count-memory-mb 8192 --samtools-threads "$SLURM_CPUS_PER_TASK" --samtools-memory 1024 --telemetry-jsonl "scindere-$SLURM_JOB_ID.jsonl"
