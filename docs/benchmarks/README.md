# Benchmark records

The JSON records in [`../benchmark-results/`](../benchmark-results/) are preserved measurements from the pre-Scindere codebase. They include synthetic dense-versus-checkpoint storage runs and process-level Linux resource samples. They are not production BAM benchmarks and do not establish Slurm or whole-job peak memory.

The synthetic benchmark example is `count_storage_bench`; run each mode in a fresh process. The retained reports include command, wall time, peak child-process RSS and sampling caveats. Keep raw records unchanged when updating project branding so their original command provenance remains intact. Add new measurements with the input dimensions, machine/runtime context, exact command and measurement scope; do not infer biological performance from synthetic matrices.
