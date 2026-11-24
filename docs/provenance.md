# Provenance and project history

Scindere has an independent project identity and carries forward code and scientific behavior from earlier projects. It does not claim to have developed the velocyto molecule-classification method.

## Scientific and software lineage

The spliced, unspliced and ambiguous molecule states follow the established velocyto methodology described by La Manno et al. in [“RNA velocity of single cells”](https://doi.org/10.1038/s41586-018-0414-6). The original [velocyto.py project](https://github.com/velocyto-team/velocyto.py) was developed by the velocyto authors and is distributed under BSD 2-Clause terms.

Scindere originated from [velocyto-rs](https://github.com/zqfang/velocyto-rs), the Rust implementation developed by Zhuoqing Fang. Its upstream README credits Johan Henriksson (@mahogny) for the original software translation work. The Rust code and interfaces carried into Scindere retain these inherited contributions and notices.

The intervening `velocyto-rs-lite` fork added and maintained behavior now carried by Scindere, including:

- BAM processing through the noodles Rust crates, with barcode and UMI tag controls.
- Generic BAM, Cell Ranger 10x, DropEst and Smart-seq2 command paths, plus sample-tag handling.
- H5AD output alongside Loom output, with regression tests for exact count layers.
- Bounded barcode counting, sparse checkpoint shards, checkpoint resume and streamed H5AD/Loom output.
- BAM sort progress, structured execution telemetry and resource-aware sort settings.
- Documentation and compatibility work for BD Rhapsody and HPC use.

These inherited engineering features are distinct from the upstream molecule-classification methodology.

## Development history

The preserved repository history records the following major stages:

- **May 2026:** the Rust project was initialized and the translated counting commands and barcode/UMI options were developed.
- **June 2026:** sample-tag handling and AnnData output were added; BAM access moved to the noodles crates.
- **October 2026:** counting gained sparse batch checkpoints, bounded memory behavior and recoverable progress.
- **November 2026:** sparse count exactness and export efficiency were refined; Cargo packaging, BD Rhapsody guidance and BAM sort progress were prepared.
- **Scindere 0.1.0:** established an independent package, library, executable, documentation and repository identity while preserving the inherited count logic, checkpoint format and output behavior.

This timeline summarizes the inherited codebase and does not assign upstream work to Scindere.

## Attribution and licensing

The original BSD 2-Clause license and copyright notices remain in the [Scindere license](https://github.com/Thokas99/scindere/blob/main/LICENSE). The [velocyto.py license](https://github.com/velocyto-team/velocyto.py/blob/master/LICENSE) and [velocyto-rs license](https://github.com/zqfang/velocyto-rs/blob/main/LICENSE) are acknowledged; preserve all required notices when redistributing derived source or binaries.

Cite La Manno et al. for the scientific method and see [CITATION.cff](https://github.com/Thokas99/scindere/blob/main/CITATION.cff) for the software and paper references.
