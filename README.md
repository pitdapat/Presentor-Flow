# Presenter Flow

A free, offline Windows app for preparing and presenting church song lyrics, written in Rust with egui. It follows ProPresenter's workflow but has its own branding and shares no code or assets with it.

**Status:** pre-alpha, milestone M0. See [docs/STATUS.md](docs/STATUS.md) for the live task board.

## Documents

| File | Purpose |
|---|---|
| [docs/PLAN.md](docs/PLAN.md) | Architecture, rules, milestones and roadmap |
| [docs/STATUS.md](docs/STATUS.md) | Task checklist per milestone, risks, backlog, session log |
| [docs/GATES.md](docs/GATES.md) | Real-hardware acceptance checklists and results |
| [docs/DECISIONS.md](docs/DECISIONS.md) | Design decision records |
| [docs/ORIGINAL_BUILD_PLAN.md](docs/ORIGINAL_BUILD_PLAN.md) | The first-version plan, kept for reference |
| [CHANGELOG.md](CHANGELOG.md) | User-visible changes |

## Layout

```
crates/core   presenter-core  — domain, library, live state, layout, storage (no UI/OS deps)
crates/app    presenter-flow  — egui operator app, output window, Windows monitor code
assets/fonts  bundled fonts (added in T0.5)
scripts       helper scripts (run.bat, run-release.bat), see scripts/README.md
```

## Run

Double-click `scripts\run.bat` to build (if needed) and open the app, or `scripts\run-release.bat` for the smoother release build.

## Build

Requires the Rust toolchain pinned in `rust-toolchain.toml` and the MSVC build tools.

```powershell
cargo run -p presenter-flow          # run the app
cargo test --workspace               # all tests
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

## Workflow

One task = one branch (`m<N>/t<N.M>-<slug>`) = one PR. Tick the task in `docs/STATUS.md` in the same PR that completes it.
