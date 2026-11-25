# Compact evidence benchmark: 200,000 reads

Measured 2026-11-25 on Linux 7.0.0-44-generic, AMD Ryzen AI 7 450, Rust 1.99.0, release build. Each run used a fresh process. The 539,461-byte BAM contains 200,000 identical alignments for one cell and one UMI, mapped to one gene and transcript with two exons. Both versions returned one count.

| Path | Internal count time, median of 5 | Process wall time, median of 5 | Peak RSS, median of 5 |
|---|---:|---:|---:|
| Existing map-based path (`84ca23c`) | 0.121 s | 0.15 s | 97,612 KiB |
| Compact path (`e8d3e4f`) | 0.112 s | 0.14 s | 97,684 KiB |

The internal timer covers `count_stream`; `/usr/bin/time` covers BAM setup and counting. The compact path was 9 ms faster in this small synthetic case. The 72 KiB RSS difference is within run-to-run noise and shows no measurable memory change. This benchmark does not count allocator calls or represent production BAMs; the roadmap item remains Partial pending constrained and representative runs.

## Reproduce

Generate the fixture once, then build each version with the same example and run it five times in a fresh process:

```sh
cargo build --locked --release --all-features --example compact_evidence_bench
target/release/examples/compact_evidence_bench generate /tmp/high-umi.bam /tmp/genes.gtf 200000
/usr/bin/time -f 'wall_s=%e peak_rss_kib=%M' target/release/examples/compact_evidence_bench count /tmp/high-umi.bam /tmp/genes.gtf 200000
```

For the baseline, copy the example into a checkout of `84ca23c` and build it with `cargo build --locked --release --all-features --example compact_evidence_bench`. The program checks that each run counts one molecule.
