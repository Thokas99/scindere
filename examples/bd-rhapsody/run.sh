#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 3 ]]; then
  printf 'Usage: %s BIOPRODUCT.bam RSEC_MEX.zip GENES.gtf\n' "$0" >&2
  exit 64
fi

bam=$1
mex=$2
gtf=$3
tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

unzip -p "$mex" barcodes.tsv.gz > "$tmpdir/barcodes.tsv.gz"
scindere run "$bam" "$gtf" --bcfile "$tmpdir/barcodes.tsv.gz" --cb-tag CB --ub-tag MA --outputfolder results
