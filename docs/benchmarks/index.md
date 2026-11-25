# Benchmarks

## Compact molecule evidence: 200,000 synthetic reads

Measured on 2026-11-25 with Linux 7.0.0-44-generic, an AMD Ryzen AI 7 450 and
Rust 1.99.0. Each measurement used a fresh process. The 539,461-byte BAM
contains 200,000 identical reads for one cell and one UMI, mapped to one gene
and transcript with two exons. Both versions returned one count.

| Path | Count time, median of 5 | Process wall time, median of 5 | Peak RSS, median of 5 |
|---|---:|---:|---:|
| Map-based baseline (`84ca23c`) | 0.121 s | 0.15 s | 97,612 KiB |
| Compact evidence (`e8d3e4f`) | 0.112 s | 0.14 s | 97,684 KiB |

The count timer covers `count_stream`; `/usr/bin/time` covers BAM setup and
counting. The 9 ms difference is specific to this small fixture. The 72 KiB RSS
difference is within run-to-run noise and shows no measurable memory change.
No allocation counts were collected. This synthetic result does not establish
production performance or whole-job memory use; the roadmap remains Partial.

### Reproduce

Generate the fixture once, then count it five times in separate processes:

```sh
cargo build --locked --release --all-features --example compact_evidence_bench
target/release/examples/compact_evidence_bench generate /tmp/high-umi.bam /tmp/genes.gtf 200000
/usr/bin/time -f 'wall_s=%e peak_rss_kib=%M' target/release/examples/compact_evidence_bench count /tmp/high-umi.bam /tmp/genes.gtf 200000
```

The program checks that each run counts one molecule. For the baseline, copy
the example into a checkout of `84ca23c` and build it there.

Earlier [dense-storage and checkpoint measurements](https://github.com/Thokas99/scindere/tree/main/docs/benchmarks/results)
record separate workloads and should not be compared directly with this case.
