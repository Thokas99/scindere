# Benchmark records

The JSON records in [results](results/) are preserved measurements from the pre-Scindere codebase. They include synthetic dense-versus-checkpoint storage runs and Linux process-level resource samples. They are not production BAM benchmarks and do not establish Slurm or whole-job peak memory.

The benchmark example is [count_storage_bench](../../examples/count_storage_bench.rs). Run each mode in a fresh process. The retained reports include command, wall time, peak child-process RSS and sampling caveats. Keep raw records unchanged; new measurements should include input dimensions, machine/runtime context, exact command and measurement scope.
