# Scindere GitHub Pages Documentation Site Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Publish Scindere's existing technical documentation as a searchable mdBook website at `https://thokas99.github.io/scindere/`.

**Architecture:** `book.toml` at the repository root will build the existing `docs/` Markdown tree into a static mdBook. A dedicated GitHub Actions workflow will build on pull requests and deploy the generated Pages artifact from `main`; GitHub Pages will use the `/scindere/` project path.

**Tech Stack:** mdBook 0.5.4, Markdown, a small local CSS theme, GitHub Actions, GitHub Pages.

**Spec:** `docs/superpowers/specs/2026-11-24-github-pages-documentation-site-design.md`

## Global Constraints

- Keep Markdown sources and scientific wording, attribution, license notices, and checkpoint compatibility details.
- No output site files are committed to a branch; Actions deploys the generated artifact.
- Pull requests validate the build but never deploy.
- The project site uses `/scindere/` as its base path.
- Keep the existing root README as the project landing page and link it to the docs site.
- Do not change Rust code, Cargo package metadata, counting behavior, Cargo publication status, or release tags.
- Retain root `ROADMAP.md` and `CONTRIBUTING.md` as brief GitHub-facing links to their site pages.

## Review Focus

- **Project-site base path:** nested pages and theme assets must resolve below `/scindere/`; Task 2 checks generated HTML and the deployed site.
- **Moved root guides:** root roadmap/contribution pointers and site links must resolve after migration; Task 1 builds all chapters and checks rendered links.
- **Pages-only files and attribution:** root-only files such as `CITATION.cff` and `LICENSE` must not become broken relative site links; Task 1 checks all rendered links and keeps GitHub links where appropriate.
- **Light/dark and narrow screens:** existing mark, colors, readable contrast, and navigation must remain usable; Task 2 checks generated pages in both themes and a mobile-width browser view.
- **Deployment boundary:** pull requests must never publish; only `main` may deploy; Task 3 checks the workflow condition and the GitHub environment branch policy.

---

### Task 1: Build the documentation book and navigation

**Files:**

- Create: `book.toml`
- Create: `docs/SUMMARY.md`
- Modify: `docs/README.md`, `README.md`
- Create: `docs/development/roadmap.md`, `docs/development/contributing.md`
- Modify: `ROADMAP.md`, `CONTRIBUTING.md`
- Modify as needed for links: `docs/architecture/overview.md`, `docs/benchmarks/README.md`, `docs/configuration.md`, `docs/development/checkpoints.md`, `docs/development/release.md`, `docs/installation.md`, `docs/output.md`, `docs/provenance.md`, `docs/troubleshooting.md`, `docs/usage.md`

**Interfaces:**

- The book source directory is `docs/`; `docs/SUMMARY.md` is the sole navigation source.
- `book.toml` sets `site-url = "/scindere/"` and the generated directory to `target/mdbook/`.
- The book home is `docs/README.md`; root `README.md` remains the GitHub landing page.
- The site navigation groups start-here guides, user guides, architecture/development, and project provenance/benchmarks.

- [x] **Install the pinned local build tool**

Run: `cargo install mdbook --locked --version 0.5.4 --root /tmp/scindere-mdbook`

Expected: `/tmp/scindere-mdbook/bin/mdbook --version` reports `mdbook v0.5.4`; no Cargo project dependency or lockfile changes.

- [x] **Add the mdBook configuration and navigation**

Create `book.toml` and `docs/SUMMARY.md`. Set the book title to `Scindere Documentation`, source to `docs`, output to `target/mdbook`, and the GitHub Pages site URL to `/scindere/`. List every public documentation chapter exactly once; exclude `docs/superpowers/specs/` and `docs/superpowers/plans/` from the public navigation.

- [x] **Move roadmap and contribution content into the book source**

Move the existing full content to `docs/development/roadmap.md` and `docs/development/contributing.md`. Replace root `ROADMAP.md` and `CONTRIBUTING.md` with short links to the corresponding site pages while retaining their full content in the site.

- [x] **Make the docs home and root README point to the site**

Turn `docs/README.md` into a concise entry page with links into the book. In root `README.md`, replace direct links to individual Markdown guides with one documentation-site link to `https://thokas99.github.io/scindere/`.

- [x] **Repair links for the new book paths**

Update local links affected by the roadmap/contribution moves. For root-only metadata such as `CITATION.cff` and `LICENSE`, link to their GitHub `main` files or link to an existing site page; do not leave relative links that escape the book source.

- [x] **Build and inspect the complete book**

Run: `/tmp/scindere-mdbook/bin/mdbook build --dest-dir target/mdbook`

Expected: build succeeds; `target/mdbook/index.html` and every chapter in `docs/SUMMARY.md` exist; generated chapter and asset links remain document-relative under the `/scindere/` project path, and `404.html` uses that base path; no public page links to a missing local chapter or a planning document.

- [x] **Commit the independently reviewable docs migration**

Commit message: `docs: organize Scindere guides as an mdBook`

### Task 2: Apply Scindere's visual identity

**Files:**

- Create: `docs/theme/scindere.css`
- Modify: `book.toml`, `docs/README.md`

**Interfaces:**

- Reuse `docs/assets/scindere-mark.svg`; do not create a second logo or add a frontend dependency.
- Use mdBook's built-in theme controls and search. The local CSS only supplies Scindere's restrained teal/violet accents and spacing.

- [x] **Add the mark and small CSS override**

Show the existing mark on the documentation home with descriptive alt text. Add the CSS file and reference it with `additional-css` in `book.toml`; retain mdBook's light/dark selector and search controls.

- [x] **Rebuild and inspect both themes**

Run: `/tmp/scindere-mdbook/bin/mdbook build --dest-dir target/mdbook`

Expected: build succeeds, the rendered home includes the logo and theme stylesheet, and the sidebar/search appear. Inspect light and dark themes at desktop and narrow mobile widths; confirm text contrast and navigation remain readable.

- [x] **Commit the branded book**

Commit message: `docs: apply Scindere theme to documentation site`

### Task 3: Deploy the book through GitHub Pages

**Files:**

- Create: `.github/workflows/docs.yml`

**Interfaces:**

- Use `actions/checkout@v7`, `actions/configure-pages@v5`, `actions/upload-pages-artifact@v4`, and `actions/deploy-pages@v4`.
- Build with pinned mdBook `0.5.4`; send the generated `target/mdbook/` contents as the Pages artifact.
- Deploy only when the event is a push or manual dispatch on `refs/heads/main`; pull requests build only.
- Deployment uses environment `github-pages`, with the environment branch policy restricted to `main`.

- [x] **Add a separate docs workflow**

Trigger on pull requests, pushes to `main`, and manual dispatch, scoped to book/docs/README/roadmap/contribution/workflow paths. Give the build job `contents: read`. Give Pages write and OIDC token permissions only to the deployment job. Make the deployment job depend on a successful build and guard it with `github.ref == 'refs/heads/main'` plus a non-PR event check.

- [x] **Check workflow structure locally**

Run: `git diff --check` and parse `.github/workflows/docs.yml` as YAML. Inspect that the deploy job has `needs: build`, the exact `main` guard, minimal Pages/OIDC permissions, and the GitHub Pages environment URL output.

- [x] **Enable Pages and restrict its environment**

Use the authenticated GitHub CLI to create/update `Thokas99/scindere` Pages with `build_type: workflow`. Configure environment `github-pages` with a custom deployment branch policy containing only `main`. Read the Pages configuration and branch policy back to confirm both settings. If GitHub denies the authenticated settings change, leave the workflow complete and report the exact maintainer setting needed.

- [x] **Push to main and verify the hosted site**

Commit message: `ci: publish Scindere documentation with GitHub Pages`. Push the reviewed commits to `main` as a fast-forward. Follow the docs workflow until build and deployment succeed; then verify `https://thokas99.github.io/scindere/` serves the book, search, images, nested page links, and `/scindere/` asset paths.

## Final Self-Review

- Existing user/developer guides, architecture, roadmap, benchmarks, provenance, and checkpoint details each have a navigation entry.
- The root README points to the hosted site; root roadmap and contributor files remain available as links.
- Rust build/test workflows and Cargo metadata are untouched.
- Pull requests cannot deploy, and the Pages environment accepts only `main`.
- No generated HTML is tracked, and no Cargo package, release, or tag is published or changed.
