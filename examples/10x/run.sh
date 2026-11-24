#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  printf 'Usage: %s CELLRANGER_SAMPLE_DIR GENES.gtf\n' "$0" >&2
  exit 64
fi

scindere run10x "$1" "$2" --output-format h5ad
