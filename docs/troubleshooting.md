# Troubleshooting

## HDF5 headers or library not found

Install the HDF5 development package and pkg-config on the build host. The executable links to the system HDF5 library, so the matching runtime library must also be present on execution hosts. See [installation](installation.md).

## A count batch exceeds its budget

The command keeps complete barcode groups together. Reduce `--cell-batch-size` if several groups are being counted in each batch. If one barcode group alone exceeds the limit, increase `--count-memory-mb`; Scindere does not split that group.

## Resume rejects a checkpoint

Resume requires matching input identities and counting settings. Check that the BAM, GTF, whitelist, tags, logic and other counting options are the same as the original run. Current input identity uses path, size and modification time rather than a content hash.

## Output is missing

Check the command's `--output-format` selection and output folder. `run10x` writes under the Cell Ranger sample folder's `velocyto` directory by default. `write-output` requires a completed checkpoint.
