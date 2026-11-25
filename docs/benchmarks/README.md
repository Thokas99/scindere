# Benchmark records

Records state the workload, command and measurement scope. Synthetic results do not establish production BAM or whole-job resource use.

- [Compact evidence: 200,000-read synthetic case](results/compact-evidence-200k.md)
- [Earlier dense and checkpoint storage runs](https://github.com/Thokas99/scindere/tree/main/docs/benchmarks/results)

The existing [`count_storage_bench`](https://github.com/Thokas99/scindere/blob/main/examples/count_storage_bench.rs) measures count-storage and export paths. [`compact_evidence_bench`](https://github.com/Thokas99/scindere/blob/main/examples/compact_evidence_bench.rs) compares molecule-evidence counting. Run each binary in a fresh process; report peak RSS separately from the internal count timer.
