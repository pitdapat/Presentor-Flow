# Presenter Flow — Status

Last updated: 2026-10-07 · Current milestone: **M0 — Foundation and risk prototype**

How to use this file: tick a box only when the task meets the Definition of Done (PLAN §2.4). Gates are ticked only by the product owner. Each task links to its PR once one exists.

Legend: `[x]` done · `[~]` in review / partially done · `[ ]` todo

## Roadmap at a glance

| Milestone | Target week | Status | Gate |
|---|---|---|---|
| M0 Foundation and risk prototype | 1 | 🟡 In progress | G0 ☐ |
| M1 Songs and storage | 2 | ⚪ Not started | G1 ☐ |
| M2 Live operation (first usable) | 3 | ⚪ Not started | G2 ☐ |
| M3 Playlists | 4 | ⚪ Not started | G3 ☐ |
| M4 Styles and templates | 5 | ⚪ Not started | G4 ☐ |
| M5 Hardening and release — **v1.0** | 6 | ⚪ Not started | G5 ☐ |

## M0 — Foundation and risk prototype (3–4 S)

- [x] **T0.0** Rust toolchain installed (rustup 1.99.0 on `D:\dev\rust`; MSVC 14.44 + Windows SDK reused from existing VS Build Tools)
- [x] **T0.1** Workspace, both crates, toolchain file, lints, pinned dependencies, CI, `docs/` skeleton — CI green on `main` (run 37633412168)
- [x] **T0.2** Operator shell: all panels as placeholders, theme, `Action`/`apply` wiring — [PR #1](https://github.com/pitdapat/Presentor-Flow/pull/1); screenshot `docs/screenshots/t0.2-operator-shell.png`
- [x] **T0.3** `platform/windows.rs` monitor list + toolbar display selector — `EnumDisplayMonitors` + DPI; numbered "Display 1, 2…"; saved choice restored on restart
- [~] **T0.4** Output viewport on the chosen monitor — test window on this screen verified; borderless fullscreen on a second monitor built (ADR-0009) but **needs G0**
- [x] **T0.5** Bundled fonts (loaded from `assets/fonts`), `EguiMeasurer`, `layout_slide`, `paint_slide`, parity test at 3 scales
- [ ] **G0** (you) Second screen: correct monitor, fullscreen, Chinese renders, line height looks right

## M1 — Songs and storage (3–4 S)

- [x] **T1.1** Domain types, IDs, `parse_lyrics` with table-driven tests (17 cases)
- [x] **T1.2** `Library` song operations and validation
- [x] **T1.3** Storage schema v1, `JsonRepository` (atomic save, `.bak`, corrupt files set aside, never deleted)
- [~] **T1.4** Song editor dialog, library panel + search, thumbnails, autosave, sample songs on first run — built; dialog opens and validates; add/edit not yet clicked through end to end
- [ ] **G1** (you) 3 real songs incl. Chinese survive restart; hand-corrupted file recovers from `.bak`

## M2 — Live operation (2–3 S) — first usable

- [x] **T2.1** `LiveState` rules (PLAN §3.5) with full unit tests
- [x] **T2.2** Click thumbnail → live; preview-only selection (right-click / Alt+click); both previews; badges — verified in the running app
- [~] **T2.3** Next / Previous / Clear / Blackout buttons + shortcuts with focus rules — built and unit-tested; keyboard/clicker not tried on hardware
- [~] **T2.4** Display disconnect and output-window close — closing verified (operator keeps running); disconnect covered by unit test only
- [ ] **G2** (you) Present a full song on the projector; live edit doesn't change output until Next; unplug/replug

## M3 — Playlists (1–2 S)

- [~] **T3.1** `Library` playlist operations + reference-deletion invariant — deleting a song already removes its playlist entries; playlist operations not started
- [ ] **T3.2** Playlist panel, add, move up/down, remove, delete confirmation
- [ ] **G3** (you) Build next Sunday's service as a playlist

## M4 — Styles and templates (2–3 S)

- [~] **T4.1** Style panel controls with validated ranges — range validation done in core; no style panel yet
- [ ] **T4.2** Templates: default "Centered Lyrics", Save as Template, apply by copy
- [~] **T4.3** Overflow badge and warning — "⚠ too long" badge on thumbnails done; warning in the style panel comes with T4.1
- [ ] **G4** (you) Readable style on the real projector, saved as a template

## M5 — Hardening and release (2–3 S)

- [ ] **T5.0** Switch GitHub CI back on for PRs and `main` (ADR-0007); first green run
- [ ] **T5.1** DPI review (100/125/150 %), min panel sizes, error-message pass
- [ ] **T5.2** Independent fresh-context code review; fix findings; remove skeleton `expect(dead_code)`
- [ ] **T5.3** Release build: icon, version info, portable zip
- [ ] **T5.4** `CHANGELOG.md`, `docs/USER_GUIDE.md`
- [ ] **G5** (you) Full offline rehearsal on church laptop + projector — **v1.0**

## Open risks

| ID | Risk | State |
|---|---|---|
| R1 | Fullscreen viewport on chosen monitor unreliable | Open — built with `with_monitor` (ADR-0009); confirm at G0 |
| R2 | Measurer vs. renderer mismatch | Mitigated — one `text_job` for both; parity test at 3 scales passes |
| R3 | Noto Sans SC size / startup time | Mitigated — 8.3 MB subset font loaded from `assets/fonts`, not embedded; startup ~2 s in debug |
| R4 | DPI differences laptop vs projector | Open — G0/T5.1 |
| R5 | No Rust toolchain | **Closed** 2026-10-07 |
| R6 | Agent context loss between sessions | Mitigated — this file + DECISIONS.md |
| R7 | Scope creep | Mitigated — backlog below |
| R8 | egui breaking changes | Observed — 0.36 renamed panels/App API (ADR-0004); versions pinned |

## Backlog (needs your approval before entering a milestone)

| Idea | Value | Size | Proposed release |
|---|---|---|---|
| _(empty)_ | | | |

## Session log

| Date | Session | Done | Next |
|---|---|---|---|
| 2026-10-07 | S1 | T0.0; T0.1 skeleton (workspace, crates, CI, docs); initial push to `main` | Confirm CI green → T0.2 on branch `m0/t0.2-operator-shell` |
| 2026-10-07 | S2 | T0.1 CI green; T0.2 operator shell (theme, PLAN §4 layout, split columns, live badge, blackout banner) → PR; `scripts\run.bat`; CI switched to manual-only, local `scripts\check.bat` (ADR-0007); PR #1 merged; agent auto-merges after local checks (ADR-0008) | T0.3 monitor list on `m0/t0.3-monitor-list` |
| 2026-10-07 | S3 | Basic functionality in one PR at your request (ADR-0009): T0.3, T0.5, T1.1–T1.3, T2.1, T2.2 done; T0.4, T1.4, T2.3, T2.4 built, partly verified; sample songs; 54 tests | You: run G0 + try adding a real song → then finish T1.4/T2.3/T2.4 checks, start M3 playlists |
