# Provenance and citations

Scindere quantifies molecules for RNA velocity workflows. It does not infer
velocity or introduce a new molecule-classification method.

## Scientific method

The spliced, unspliced and ambiguous molecule states follow velocyto, described
by La Manno et al. in [“RNA velocity of single cells”](https://doi.org/10.1038/s41586-018-0414-6).
The [velocyto.py project](https://github.com/velocyto-team/velocyto.py) was
developed by the paper's authors and is distributed under the BSD 2-Clause
License.

## Software and engineering lineage

Scindere derives from [velocyto-rs](https://github.com/zqfang/velocyto-rs),
Zhuoqing Fang's Rust implementation. Its upstream README credits Johan
Henriksson for original translation contributions. The intervening
`velocyto-rs-lite` project carried forward Rust workflows and engineering now
present in Scindere, including barcode and UMI handling, BAM workflows, H5AD
output, bounded sequential counting, sparse checkpoints, resume, streamed
output and resource telemetry.

Those resource and output features are software engineering contributions;
the classification methodology remains inherited from velocyto. Scindere
retains upstream notices and BSD 2-Clause terms.

## Development history

- **May–June 2026:** Rust counting commands, barcode/UMI controls, sample tags,
  noodles BAM access and AnnData output were developed.
- **October 2026:** counting gained sparse batch checkpoints, bounded batches
  and recovery.
- **November 2026:** output exactness, sort progress, resource telemetry and the
  Scindere project identity were refined.

The repository history and [development archive](superpowers/archive/index.md)
record earlier implementation plans. The [roadmap](development/roadmap.md)
holds current status and validation gates.

## Citations and licensing

Cite La Manno et al. for the scientific method and use the software references
in [`CITATION.cff`](https://github.com/Thokas99/scindere/blob/main/CITATION.cff).
Preserve the required notices when redistributing derived source or binaries;
see the [Scindere license](https://github.com/Thokas99/scindere/blob/main/LICENSE),
[velocyto.py license](https://github.com/velocyto-team/velocyto.py/blob/master/LICENSE)
and [velocyto-rs license](https://github.com/zqfang/velocyto-rs/blob/main/LICENSE).
