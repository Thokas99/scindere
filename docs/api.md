# Rust API

[Open the generated Rust API reference](https://thokas99.github.io/scindere/api/scindere/index.html).

The reference is generated from the public crate with `cargo doc`. Start with [`ExInCounter`](https://thokas99.github.io/scindere/api/scindere/counter/struct.ExInCounter.html) for collecting `count()` and batch-oriented `count_stream()`, [`Logic`](https://thokas99.github.io/scindere/api/scindere/logic/trait.Logic.html) for classification, and [`Checkpoint`](https://thokas99.github.io/scindere/api/scindere/count_checkpoint/struct.Checkpoint.html) for durable count batches.

The internal compact molecule-evidence types are private. Existing public `Molitem`, `SegmentMatch` and `Logic::count` interfaces remain available.
