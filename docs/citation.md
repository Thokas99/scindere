# Citation & Credits

## Scientific method

Scindere quantifies molecules for RNA velocity analysis; it does not infer
velocity or introduce a new molecule-classification method. Its spliced,
unspliced and ambiguous states follow velocyto, described by La Manno et al. in
[“RNA velocity of single cells”](https://doi.org/10.1038/s41586-018-0414-6),
*Nature* 560, 494–498 (2018).

## Software lineage

Scindere derives from [velocyto-rs](https://github.com/zqfang/velocyto-rs),
Zhuoqing Fang's Rust implementation. Its upstream README credits Johan
Henriksson for original translation contributions. The intervening
`velocyto-rs-lite` project carried forward Rust workflows and engineering now
present in Scindere, including barcode and UMI handling, BAM workflows, H5AD
output, bounded sequential counting, sparse checkpoints, resume, streamed
output and resource telemetry. These are software engineering contributions;
the molecule-classification methodology remains inherited from velocyto.

Please cite La Manno et al. for the scientific method and use the software
references in [`CITATION.cff`](https://github.com/Thokas99/scindere/blob/main/CITATION.cff).

## Licensing and notices

Scindere is distributed under the [BSD 2-Clause License](https://github.com/Thokas99/scindere/blob/main/LICENSE).
Preserve required upstream notices when redistributing derived source or
binaries. See the [velocyto.py license](https://github.com/velocyto-team/velocyto.py/blob/master/LICENSE)
and [velocyto-rs license](https://github.com/zqfang/velocyto-rs/blob/main/LICENSE).
