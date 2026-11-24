# Scindere development notes

- Preserve molecule-classification behavior and the existing `run`/`write-output` argument interface unless a separate change explicitly requires otherwise.
- Keep sparse checkpoint storage distinct from bounded dense scratch matrices in documentation and code comments.
- Do not describe `--count-memory-mb` as a process-wide RSS cap; it budgets counting components.
- Preserve upstream copyright notices, BSD 2-Clause terms and attribution in `LICENSE` and redistributed artifacts.
- The rebrand keeps checkpoint format version 2 unchanged. Any future incompatible checkpoint change needs a versioned migration and explicit compatibility documentation.
- Before a release, run the validation commands in `CONTRIBUTING.md` and review `ROADMAP.md` for claims that need updating.
