# Scindere documentation site design

**Status:** Draft for user approval
**Decision:** mdBook built and deployed with GitHub Pages Actions

## Intent

Give Scindere users one public, navigable documentation site instead of asking them to discover individual Markdown pages through the repository README. Keep Markdown as the reviewable source format; the site is its published presentation. The proposed project URL is `https://thokas99.github.io/scindere/`.

## Current state

- The root README links to separate guides under `docs/`.
- `docs/README.md` is a link index, and the guides already contain installation, usage, configuration, output, troubleshooting, architecture, benchmarks, provenance, and development information.
- `ROADMAP.md` and `CONTRIBUTING.md` remain at the repository root.
- The repository is public, uses `main`, and has Rust CI but no docs build or deployment workflow.
- GitHub's Pages API returns 404, so Pages is not enabled yet.

## Goals

- Publish a single branded, responsive docs site with sidebar navigation and search.
- Preserve Markdown sources and scientific wording, attribution, license notices, and checkpoint compatibility details.
- Give users a clear path from installation through usage, output interpretation, and troubleshooting.
- Include architecture, roadmap, provenance, benchmarks, and contribution material in the site navigation.
- Build the site for pull requests and deploy it from `main` through GitHub Actions.
- Keep the existing concise root README as the GitHub project landing page and point its documentation section to the site.

## Non-goals

- Rewriting the Rust application, CLI, package metadata, or scientific counting behavior.
- Replacing Markdown with hand-maintained HTML or deleting documentation sources.
- Adding a custom domain, analytics, comments, or a new documentation service.
- Publishing a Cargo package or creating a release.

## Approaches considered

1. **GitHub Pages with Jekyll:** the most direct Pages setup and a familiar Markdown workflow. Its basic theme and navigation/search fit a technical manual less well without additional theme work.
2. **mdBook with GitHub Pages Actions — selected:** Rust-native, Markdown-based, with a generated sidebar and built-in search. It requires a book navigation file, a pinned mdBook build tool, and a small Pages workflow. This matches Scindere's Rust toolchain while keeping the source and deployment simple.
3. **MkDocs Material with GitHub Pages Actions:** the strongest out-of-the-box docs presentation and navigation. It adds a Python toolchain and theme maintenance to a Rust repository, so it is more than this site needs initially.

GitHub Pages supports custom Actions workflows for static site generators. mdBook supports Markdown books, navigation, search, and theme customization. References: [GitHub Pages custom workflows](https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages), [mdBook documentation](https://rust-lang.github.io/mdBook/), and [mdBook continuous integration](https://rust-lang.github.io/mdBook/continuous-integration.html).

## Proposed site structure

Use `book.toml` at the repository root and the existing `docs/` tree as the book source. Add a navigation summary that groups the existing pages into:

- **Start here:** documentation home, installation, usage.
- **Guides:** configuration, output and checkpoints, troubleshooting.
- **Architecture and development:** architecture, checkpoint internals, roadmap, contributing, release process.
- **Project:** benchmark records and provenance.

Use `docs/README.md` as the site home content, expanding its plain index into a short route through the documentation. Keep the current root README as the repository landing page, replacing its links to individual guide files with a prominent docs-site link.

Move the canonical roadmap and contributor guide content into the book source so the site can render them as normal pages. Retain root `ROADMAP.md` and `CONTRIBUTING.md` as brief links to those pages for GitHub conventions. Keep agent instructions, changelog, citation metadata, Cargo files, and the license in their current locations; link to relevant root files where needed.

Use the existing Scindere mark and restrained teal/violet colors. Keep mdBook's light/dark theme controls and search; use a small local CSS override only if needed to carry the existing visual identity. No JavaScript framework or frontend dependency is proposed.

## Build and deployment

- Add a dedicated docs workflow, separate from Rust CI.
- Build on documentation pull requests and on pushes to `main`; allow a manual run.
- Grant read-only content access for builds. Grant `pages: write` and `id-token: write` only to the deployment job.
- Upload the generated book with GitHub's Pages artifact action and deploy with its Pages action. Pull requests validate the build but never deploy.
- Pin mdBook to a known release and set the site base path to `/scindere/` so assets and navigation work on a project site.
- Configure repository Pages source to **GitHub Actions** and use the `github-pages` deployment environment, restricted to `main`.
- Treat the request to set up the site as authorization to make these Pages settings through the authenticated GitHub session. If GitHub denies the settings update, leave the workflow ready and report the exact maintainer action needed.
- No output site files are committed to a branch; Actions deploys the generated artifact.

## Validation

- Build the book locally and verify all chapters, images, and intra-site links resolve.
- Confirm the built home, sidebar, search, light/dark rendering, and the GitHub Pages project base path.
- Confirm pull requests run the docs build without deployment and a push to `main` deploys successfully.
- Smoke-test the published URL on desktop and mobile widths; follow the install, usage, output, and provenance navigation paths.
- Re-run the existing Rust CI checks only if implementation changes touch Rust or the Rust workflow.

## Calls left to implementation

- Select the exact mdBook patch release and the smallest reliable CI installation method.
- Set page titles and navigation labels consistently with the existing Scindere brand.
- Keep the existing scientific content intact except for navigation and links needed to render it as a site.

## Approval gate

This document describes the proposed site and deployment. No site scaffolding, workflow, Pages setting, or documentation move should be implemented until the user approves this design. After approval, prepare an implementation plan before changing the repository.
