# Decision records

Short ADRs: context, decision, consequences. Newest last. A decision is changed by adding a new record that supersedes the old one.

## ADR-0001 — Two-crate workspace for compile-time separation

- **Date:** 2026-10-07 · **Status:** Accepted
- **Context:** The original plan relied on discipline to keep egui out of domain code.
- **Decision:** `presenter-core` (no UI/OS crates) and `presenter-flow` (egui app). `crates/core/tests/architecture.rs` fails if a forbidden crate is added to core's manifest.
- **Consequences:** Logic is testable without a window; a stray `use egui::…` in core fails to compile.

## ADR-0002 — Unidirectional state flow (actions + apply)

- **Date:** 2026-10-07 · **Status:** Accepted
- **Context:** egui panels mutating state directly leads to borrow fights and logic in the UI.
- **Decision:** Panels take `&AppState` and push `Action`s; `actions::apply` is the only writer.
- **Consequences:** Actions are unit-testable; panels stay presentation-only.

## ADR-0003 — Lyrics source text is the source of truth

- **Date:** 2026-10-07 · **Status:** Accepted
- **Decision:** Songs store `lyrics_source`; slides are derived by the pure `parse_lyrics`.
- **Consequences:** Exact edit round-trip; no stored slide IDs in v1.

## ADR-0004 — Pin egui/eframe 0.36.2 and use its new panel/App API

- **Date:** 2026-10-07 · **Status:** Accepted
- **Context:** egui 0.36 replaced `TopBottomPanel`/`SidePanel` with `egui::Panel` and `eframe::App::update` with `App::ui(&mut Ui, …)`.
- **Decision:** Use the 0.36 API. All dependencies are pinned with `=` in the root `Cargo.toml`; upgrades are planned tasks.
- **Consequences:** Online examples for older egui need translating.

## ADR-0005 — Bootstrap commits pushed directly to `main`

- **Date:** 2026-10-07 · **Status:** Accepted (one-time exception)
- **Context:** The GitHub repo was empty, and the product owner asked for the plan and skeleton to land on `main`.
- **Decision:** The initial commit and the skeleton commit go straight to `main`. From T0.2 onwards every task uses a feature branch and a PR (PLAN §2.3).
- **Consequences:** The skeleton was not reviewed through a PR; it is covered by the T5.2 independent review.

## ADR-0006 — Skeleton placeholders use `todo!("Tx.y: …")`

- **Date:** 2026-10-07 · **Status:** Accepted (temporary)
- **Decision:** Unimplemented functions panic with the task ID that implements them, so `grep 'todo!("T'` lists outstanding work. The app crate carries `#![expect(dead_code)]` during the skeleton phase; `expect` warns once nothing is dead, forcing its removal.
- **Consequences:** No `todo!` may remain on a code path reachable from the UI at a milestone gate. All are gone by T5.2.


## ADR-0007 - Local checks instead of GitHub CI until release prep

- **Date:** 2026-10-07 · **Status:** Accepted
- **Context:** The product owner wants to conserve GitHub Actions usage during development.
- **Decision:** `.github/workflows/ci.yml` runs only when started by hand (`workflow_dispatch`). The Definition of Done is met by running `scripts\check.bat` locally (fmt, clippy `-D warnings`, tests; `release` adds the release build), and the agent's report includes its output. Automatic CI on PRs is switched back on during M5 (release prep).
- **Consequences:** Checks run on the developer machine only, so a "works on my machine" problem (e.g. an uncommitted file) can slip through. Mitigation: run `check.bat` from a clean working tree before reporting a task done, and run CI once by hand at each milestone gate if needed.
