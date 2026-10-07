# Presenter Flow — Build Plan and Roadmap

Revision 2 — 7 October 2026. Supersedes the first-version build plan (kept unchanged as the original upload).

Presenter Flow is a free, offline Windows app for preparing and presenting church song lyrics. It follows ProPresenter's workflow and rough panel layout, uses its own branding, and is written entirely in Rust.

Guiding priorities, in order:

1. **Basic functionality that is reliable on Sunday morning.**
2. **Proper Rust conventions.**
3. **Strict separation of code, enforced by the compiler.**

---

## 1. Scope

### 1.1 In v1.0

- Add, edit and delete songs by typing or pasting lyrics.
- A song library, kept separate from ordered playlists (services).
- Click a slide to send it live to a second monitor or projector.
- A preview of the selected slide and a live-output preview, as separate states.
- Text style: font (from the bundled set), size, line height, letter spacing, horizontal alignment, vertical position, text color and solid background color.
- Named templates holding a text style. Applying one copies it into the song.
- Next and previous slide, clear lyrics, blackout.
- Local autosave with backup and corrupt-file recovery.

### 1.2 Not in v1.0 (see the roadmap, §9)

Video and image backgrounds, Bible, stage display, transitions, bilingual layouts, song import, picking installed fonts, drag-and-drop, per-slide style overrides, undo, cloud features and networking.

### 1.3 Constraints

- All application code is Rust. No HTML, JavaScript, Electron or code in other languages. Operating-system APIs and graphics drivers are fine.
- The app runs fully offline and opens no network sockets.
- Target: Windows 10 and 11 on x64.
- Fonts are bundled under the OFL licence: Noto Sans and Noto Sans SC (Simplified Chinese).

---

## 2. Working model: you and the AI agent

### 2.1 Roles

| Role | Who | Responsibilities |
|---|---|---|
| Product owner | You | Set priorities, approve scope changes, accept milestones |
| Tester on real hardware | You | Projector, church laptop, real lyrics, look and feel — things the agent cannot do |
| Project manager and implementer | Kiro (AI agent) | Break down tasks, write code and tests, run local checks, track status, keep the docs current |
| Independent reviewer | A fresh-context Kiro sub-agent (`sage-review`) | Blind code review at every milestone gate |

### 2.2 What the agent can and cannot verify

| The agent can verify | Only you can verify |
|---|---|
| Compiling, `fmt`, `clippy`, unit and integration tests | Output on a real projector or second monitor |
| Parser, storage, live-state and layout logic | Readability at the back of the room, colors and fonts |
| Screenshots of the operator window on this machine | Behavior on the church laptop (DPI, drivers) |
| Code-structure rules (crate dependencies) | Whether the workflow feels right during a service |

Every milestone therefore ends with a **human gate** (§2.5). Agent-only checks never close a milestone on their own.

### 2.3 Unit of work

- **Task**: about one agent session. It has an ID (`T2.3`), one branch (`m2/t2.3-live-snapshot`) and one PR or merge request.
- **Milestone**: a set of tasks that ends in something you can use, plus a human gate.
- **Estimates are in agent sessions (S).** Calendar time is set by how often you can run gates, not by coding speed. The agent writes code quickly; real-hardware testing is the bottleneck.

### 2.4 Definition of Ready and Definition of Done

**Ready** (before the agent starts a task): the goal and acceptance criteria are written, dependencies are merged, and nothing is left for you to decide.

**Done** (before the agent reports a task complete):

- `scripts\check.bat` passes locally: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` (ADR-0007; GitHub CI is not used until M5).
- New behavior has tests in `core` wherever the logic lives there.
- There is no `unwrap()` or `expect()` on user data, files, fonts or displays.
- Public items have doc comments.
- `docs/DECISIONS.md` is updated if a design choice was made.
- The agent's report includes the commands it ran and their results, plus screenshots for any UI change.

### 2.5 Cadence and communication

- At the end of every session, the agent posts a status report with four parts: **Done**, **Evidence** (test output, screenshots), **Next**, **Needs you**.
- Long-running work is recorded in the session work ledger, so it survives context resets.
- **Gate reviews.** At the end of a milestone the agent posts the gate checklist from §7 for you to run. You reply "pass" or list the failures. Failures become bug tasks in the next session.
- **Change control.** New ideas go into the backlog (§9.3). They only enter the current milestone if you swap something else out.
- **Escalations** use one line of background, the exact action needed and the cost of waiting. The agent parks the blocked item and keeps working on others.
- Git rules: work on feature branches and never push directly to `main`. The agent squash-merges its own PR once `scripts\check.bat` passes locally and the report (with screenshots for UI changes) is posted (ADR-0008); you review at the milestone gates and can revert any merge. (One-time exception: the bootstrap commits, see ADR-0005.)
- **Progress tracking:** `docs/STATUS.md` is the checklist for every task and gate. The PR that completes a task also ticks it there.

### 2.6 Project documents (inside the repo)

| File | Purpose |
|---|---|
| `docs/PLAN.md` | This document — architecture, rules and roadmap |
| `docs/DECISIONS.md` | Short decision records (ADRs): context, decision, consequences |
| `docs/STATUS.md` | Current milestone, task board (Todo / Doing / Review / Done), open risks |
| `docs/GATES.md` | Human gate checklists with dated pass/fail results |
| `CHANGELOG.md` | User-visible changes per release |

---

## 3. Architecture

### 3.1 Workspace layout

```
presenter-flow/
├─ Cargo.toml                 # [workspace], shared lints, [profile.release]
├─ Cargo.lock                 # committed
├─ rust-toolchain.toml        # pinned stable toolchain, edition 2021
├─ assets/fonts/              # Noto Sans, Noto Sans SC (+ OFL.txt)
├─ docs/
├─ .github/workflows/ci.yml   # windows-latest: fmt, clippy, test (manual-only until M5, ADR-0007)
└─ crates/
   ├─ core/                   # package: presenter-core — NO egui, NO winit, NO windows
   │  └─ src/
   │     ├─ lib.rs
   │     ├─ ids.rs            # SongId, PlaylistId, EntryId, TemplateId (newtypes over Uuid)
   │     ├─ error.rs          # CoreError (thiserror)
   │     ├─ domain/
   │     │  ├─ song.rs        # Song, invariants
   │     │  ├─ lyrics.rs      # parse_lyrics(&str) -> ParsedLyrics  (pure)
   │     │  ├─ playlist.rs    # Playlist, PlaylistEntry
   │     │  ├─ template.rs    # Template
   │     │  └─ style.rs       # TextStyle, HAlign, VAlign, Rgba, FontFamily
   │     ├─ library/
   │     │  └─ mod.rs         # Library aggregate: all validated song/playlist/template operations
   │     ├─ presentation/
   │     │  └─ live.rs        # LiveState, LiveContent, OutputFrame, navigation, blackout
   │     ├─ layout/
   │     │  ├─ mod.rs         # layout_slide(...) -> SlideLayout (pure geometry + line breaks)
   │     │  └─ measure.rs     # trait TextMeasurer
   │     ├─ settings.rs       # Settings, DisplayRef
   │     └─ storage/
   │        ├─ mod.rs         # trait Repository
   │        ├─ schema.rs      # versioned on-disk DTOs (separate from domain types)
   │        ├─ migration.rs   # vN -> vN+1
   │        └─ json.rs        # JsonRepository: atomic save, backup, recovery
   └─ app/                    # package: presenter-flow (the binary)
      └─ src/
         ├─ main.rs           # logging + eframe::run_native, nothing else
         ├─ app.rs            # PresenterApp: eframe::App impl, frame loop, dispatch
         ├─ state.rs          # AppState: Library + LiveState + UiState
         ├─ actions.rs        # Action enum + apply()
         ├─ autosave.rs       # debounce + save scheduling
         ├─ ui/
         │  ├─ toolbar.rs  library_panel.rs  playlist_panel.rs
         │  ├─ slide_grid.rs  preview_panel.rs  style_panel.rs  status_bar.rs
         │  └─ dialogs/  song_editor.rs  template_dialog.rs  confirm.rs
         ├─ render/
         │  ├─ fonts.rs       # load bundled fonts into egui
         │  ├─ measurer.rs    # EguiMeasurer: impl core::layout::TextMeasurer
         │  └─ slide_painter.rs  # the ONE function that draws a SlideLayout into a rect
         ├─ output/
         │  └─ output_viewport.rs  # second-screen viewport lifecycle
         └─ platform/
            └─ windows.rs     # monitor enumeration (the only allowed `unsafe`, documented)
```

### 3.2 Dependency rules

```
app ──► core            (never the reverse)
app/ui ──► Action       (UI never mutates state directly)
core::library ──► core::domain
core::storage ──► core::domain     (DTO <-> domain conversion lives in storage)
core::layout ──► core::domain::style + TextMeasurer trait (no fonts, no egui)
```

- `core` has no dependency on egui, eframe, winit or the `windows` crate. Cargo enforces this, because those crates are not in `core/Cargo.toml`.
- `#![forbid(unsafe_code)]` in `core`. In `app`, `unsafe` is allowed only in `platform/windows.rs` with a `// SAFETY:` comment.
- Domain types are never serialized directly. `storage::schema` owns the file format, so domain refactors cannot silently break saved files.

### 3.3 State flow (unidirectional)

Each frame:

```
UI panels (&AppState)  ──push──►  Vec<Action>
                                       │
after UI is drawn:   for a in actions { apply(&mut state, a) }  ──► Library / LiveState
                                       │
                         autosave.mark_dirty(kind)  ──► Repository::save (debounced)
```

```rust
// app/src/actions.rs
pub enum Action {
    // Library
    CreateSong { title: String, lyrics: String },
    UpdateSong { id: SongId, title: String, lyrics: String },
    DeleteSong(SongId),                       // only after confirmation dialog
    SetSongStyle { id: SongId, style: TextStyle },
    ApplyTemplate { song: SongId, template: TemplateId },
    SaveTemplate { name: String, style: TextStyle },
    CreatePlaylist { name: String },
    RenamePlaylist { id: PlaylistId, name: String },
    DeletePlaylist(PlaylistId),
    AddToPlaylist { playlist: PlaylistId, song: SongId },
    MoveEntry { playlist: PlaylistId, entry: EntryId, dir: MoveDir },
    RemoveEntry { playlist: PlaylistId, entry: EntryId },
    // Selection (never touches live output)
    SelectSong(SongId),
    SelectSlide(usize),
    // Presentation
    GoLive { song: SongId, slide: usize },
    NextSlide,
    PreviousSlide,
    ClearLyrics,
    ToggleBlackout,
    // Output
    ChooseDisplay(DisplayRef),
    CloseOutput,
}

pub fn apply(state: &mut AppState, action: Action) -> Result<(), AppError>;
```

Errors returned by `apply` go to the status bar as user-readable messages. They never panic.

### 3.4 Core domain types

```rust
pub struct Song {
    pub id: SongId,
    pub title: String,          // non-empty after trim
    pub lyrics_source: String,  // SOURCE OF TRUTH; slides are derived
    pub style: TextStyle,       // copied from a template; independent afterwards
    pub created_at: Timestamp,  // u64 unix seconds via std::time
    pub updated_at: Timestamp,
}

impl Song {
    pub fn slides(&self) -> Vec<Slide>;   // = lyrics::parse_lyrics(&self.lyrics_source).slides
}

pub struct Slide { pub section: Option<String>, pub text: String }

pub struct Playlist { pub id: PlaylistId, pub name: String, pub entries: Vec<PlaylistEntry> }
pub struct PlaylistEntry { pub id: EntryId, pub song: SongId }   // same song may repeat

pub struct Template { pub id: TemplateId, pub name: String, pub style: TextStyle }

pub struct TextStyle {
    pub font: FontFamily,       // enum over bundled fonts in v1
    pub size: f32,              // logical canvas units, 8.0..=400.0
    pub line_height: f32,       // multiplier, 0.8..=3.0
    pub letter_spacing: f32,    // extra canvas units, -10.0..=50.0
    pub h_align: HAlign,        // Left | Center | Right
    pub v_align: VAlign,        // Top | Middle | Bottom
    pub text_color: Rgba,
    pub background: Rgba,
}
```

**Lyrics parsing rules** (`parse_lyrics`, a pure function with table-driven tests):

- Normalize `\r\n` to `\n`. Trim trailing whitespace on each line.
- A slide is a run of non-blank lines. One or more blank lines separate slides.
- A line matching `^\[(.+)\]$` (after trimming) is a section label. It starts a new section, is never shown on screen, and also ends the current slide.
- Line breaks inside a slide are kept. Repeated lines and repeated sections are kept.
- Empty slides are dropped. A song with zero slides is rejected by `Library::create_song`.
- Duplicate titles are allowed.

**Library invariants** (enforced in `library`, tested in `core`):

- Every `PlaylistEntry.song` refers to an existing song.
- `delete_song` returns the playlists it affected (for the confirmation text) and removes all entries for that song in one step.
- Validation failures return `CoreError` variants and never panic.

### 3.5 Live state

```rust
pub struct LiveState {
    content: LiveContent,
    blackout: bool,             // overlay; independent of content
}

pub enum LiveContent {
    Empty,                                   // app start: always
    Slide(LiveSnapshot),
    LyricsCleared { background: Rgba },      // keeps the background, removes text
}

pub struct LiveSnapshot {
    pub song: SongId,
    pub song_title: String,
    pub slide_index: usize,
    pub slide_count: usize,
    pub slide: Slide,
    pub style: TextStyle,
}

pub enum OutputFrame { Black, Background(Rgba), Slide { slide: Slide, style: TextStyle } }

impl LiveState {
    pub fn frame(&self) -> OutputFrame;      // what the projector draws
}
```

Rules:

- `GoLive` builds a new snapshot from the **current** library version of the song. This is the only way to change what is shown, apart from Clear and Blackout.
- `NextSlide` and `PreviousSlide` count as presenting explicitly. They re-snapshot from the current library song at index ±1, clamped to the first and last slide, and never move to another song. If the live song has been deleted, they do nothing and the status bar says why.
- Editing or deleting a song never changes the current snapshot.
- `ClearLyrics` becomes `LyricsCleared` with the snapshot's background. `Next` after a clear presents the following slide.
- `ToggleBlackout` changes only the `blackout` overlay. Leaving blackout shows the content as it was.
- `frame()`: blackout → `Black`. Otherwise `Empty` → `Black`, `LyricsCleared` → `Background`, `Slide` → `Slide`.

### 3.6 Layout and rendering

**One layout result, painted everywhere.** All line breaks are calculated once, at canvas scale (1920 × 1080). Every view then draws the same lines with wrapping turned off, scaled to its rectangle. This guarantees that thumbnails, the previews and the projector break lines identically.

```rust
// core/src/layout/measure.rs
pub trait TextMeasurer {
    /// Width in canvas units of `text` on one line, including letter spacing.
    fn line_width(&self, text: &str, style: &TextStyle) -> f32;
    /// Line advance in canvas units (font metrics × style.line_height).
    fn line_advance(&self, style: &TextStyle) -> f32;
}

// core/src/layout/mod.rs
pub const CANVAS: Size = Size { w: 1920.0, h: 1080.0 };
pub const SAFE_MARGIN: f32 = 96.0;

pub struct SlideLayout {
    pub lines: Vec<PositionedLine>,   // text + origin in canvas units
    pub overflow: bool,               // total height > safe area height
}

pub fn layout_slide(slide: &Slide, style: &TextStyle, m: &dyn TextMeasurer) -> SlideLayout;
```

- Wrapping is greedy by word. A word longer than the line is broken by character, which is also how CJK text without spaces wraps. Explicit line breaks are respected.
- When text overflows, it is still drawn (clipped by the safe area), and the UI shows a warning badge. The font is never shrunk automatically.
- `app/render/measurer.rs` implements `TextMeasurer` with egui's font system (`TextFormat::extra_letter_spacing`, `line_height`). `core` is tested with a fixed-width `FakeMeasurer`.
- `app/render/slide_painter.rs::paint_slide(painter, rect, &OutputFrame, &SlideLayout)` is the **only** drawing code for slides. Thumbnails, the selected preview, the live preview and the output window all call it.
- Layouts are cached per `(song id, updated_at, slide index)` so thumbnails are not recalculated every frame.

### 3.7 Output window and displays

- `platform/windows.rs::list_displays() -> Result<Vec<DisplayInfo>, PlatformError>` uses `EnumDisplayMonitors` and `GetMonitorInfoW`. `DisplayInfo` holds the device name (for example `\\.\DISPLAY2`), the friendly name, the physical rectangle, whether it is the primary display, and the scale factor.
- The output is a borderless egui viewport created with `show_viewport_immediate`, placed at the chosen monitor's rectangle and then made fullscreen. Using an immediate viewport means no `Arc<Mutex<…>>` around the state.
- The display list is refreshed every 2 seconds. If the chosen display disappears, the output closes, the status bar shows a red warning, and output is **never** moved onto the operator screen.
- Closing the output window means `CloseOutput`. The operator app keeps running and no data is lost.
- On first launch, or when the saved `DisplayRef` (device name plus rectangle) no longer matches, the operator is asked to choose a display. Output always starts empty.

### 3.8 Storage

Files live in `%APPDATA%\PresenterFlow\` (found with the `directories` crate):

| File | Content |
|---|---|
| `library.json` | `{ "schema_version": 1, "songs": [...], "playlists": [...], "templates": [...] }` |
| `library.json.bak` | The previous good save |
| `settings.json` | Output display, last-opened playlist (separate file, so corrupt settings never affect the library) |
| `library.corrupt-<unix>.json` | A file that failed to load, set aside. **Never deleted.** |

**Saving** (`JsonRepository::save`):

1. Serialize to `library.json.tmp`, then call `sync_all()`.
2. If `library.json` exists, rename it to `library.json.bak` (replacing the old backup).
3. Rename `library.json.tmp` to `library.json`. `std::fs::rename` replaces atomically on Windows.

**Loading**:

1. Load `library.json` and run migrations up to the current schema version.
2. Validate the IDs and references. Dangling playlist entries are dropped and reported.
3. If loading fails, move the file to `library.corrupt-*.json`, try `.bak`, and tell the user what happened.
4. If both fail, start with an empty library and a clear warning. Bad files are always kept, never overwritten.

**Autosave**: finished operations (create, delete, reorder, apply template) save immediately. Typing in the lyrics editor and changing style values saves 1 second after the last change. Settings are saved on change. Everything is saved when the app closes normally. A save failure shows a persistent red status and is retried on the next change.

### 3.9 Keyboard

These shortcuts work only when no text field has focus. Clickers usually send PageUp and PageDown.

| Key | Action |
|---|---|
| → ↓ Space PageDown | Next slide |
| ← ↑ PageUp | Previous slide |
| C | Clear lyrics |
| B | Toggle blackout |
| Esc | Close the open dialog |

### 3.10 Dependencies

Exact versions are pinned with `=` in task T0.1, and `Cargo.lock` is committed. No async runtime and no database.

| Crate | Used in | Purpose |
|---|---|---|
| `eframe` / `egui` | app | UI and viewports |
| `windows` | app/platform | Monitor enumeration |
| `serde`, `serde_json` | core | Storage DTOs |
| `uuid` (v4, serde) | core | IDs |
| `thiserror` | core, app | Typed errors |
| `directories` | core/storage | AppData path |
| `tracing`, `tracing-subscriber` | app (core uses only `tracing`) | Logging to `%APPDATA%\PresenterFlow\logs` |
| `tempfile` (dev) | core | Storage tests |

---

## 4. User interface

The theme is a dark charcoal background with teal for selection and red for anything live. Panels can be resized and have minimum widths.

| Area | Contents |
|---|---|
| Top toolbar | Add Song, New Playlist, Templates, output display selector, output on/off |
| Upper left | Library search (title, case-insensitive), song list |
| Lower left | Playlists and the ordered entries of the selected playlist (Move Up, Move Down, Remove) |
| Center | Song title, Edit Lyrics, slide thumbnail grid with section labels and overflow badges |
| Upper right | Selected-slide preview (teal frame) and live-output preview (red frame plus LIVE / BLACKOUT / CLEARED badge) |
| Lower right | Template selector, style controls, Save as Template |
| Bottom bar | Previous, Next, Clear Lyrics, Blackout; save status; output status |

Interaction rules:

- Clicking a song selects it. Live output does not change.
- Single-clicking a thumbnail selects it **and** sends it live.
- The preview action (right-click → Preview, or Alt+click) only selects the slide.
- The selection and keyboard focus are always visible.
- Blackout shows a large red banner in the operator window.

---

## 5. Coding conventions

- Standard Rust naming. `rustfmt` uses default settings.
- Workspace lints in the root `Cargo.toml`: `clippy::all = deny`, `unwrap_used = deny` and `expect_used = deny` outside tests, and `missing_docs = warn` for `core`.
- Functions stay small and do one thing. Visibility is `pub(crate)` unless something needs to be public.
- Enums are used for state and actions, never strings or loose bool flags. The one exception is the `blackout` overlay, which really is a single on/off state.
- Fallible operations return `Result`. `CoreError` and `AppError` each carry a user-readable `Display`.
- Domain rules live in `core` only. UI code formats and dispatches, and nothing more.
- Tests sit next to the code (`#[cfg(test)]`) for units. `crates/core/tests/` holds behavior tests: library, live state and storage round trips.
- CI (`windows-latest`: fmt, clippy, test, release build) is manual-only during development and runs automatically on PRs again from M5 (ADR-0007). Until then `scripts\check.bat` runs the same checks locally.

---

## 6. Risk register

| ID | Risk | Likelihood / impact | Mitigation | Owner |
|---|---|---|---|---|
| R1 | Placing a fullscreen egui viewport on a chosen monitor is unreliable | Medium / High | Prototype first (T0.4); fall back to a borderless window sized to the monitor | Agent, then a gate check by you |
| R2 | Letter spacing or line height in egui doesn't match the measurer | Medium / Medium | Single layout path (§3.6); parity test (T0.5) | Agent |
| R3 | Noto Sans SC adds roughly 10–17 MB to the binary or slows startup | High / Low | Load the font from `assets/` next to the exe instead of embedding it; measure startup | Agent |
| R4 | DPI scaling differs between the laptop screen and the projector | Medium / Medium | Size the output from the monitor's physical pixels; gate test at 100%, 125% and 150% | You |
| R5 | No Rust toolchain on this machine | **Closed 2026-10-07** | T0.0 done: Rust 1.99.0 on `D:\dev\rust` | — |
| R6 | The agent loses context between sessions | Medium / Medium | Keep `STATUS.md`, `DECISIONS.md` and the work ledger current; one task per session | Agent |
| R7 | Scope creep | High / Medium | Change control (§2.5); backlog (§9.3) | You |
| R8 | egui breaking changes | Medium / Low | Pin versions exactly; upgrade only as a planned task | Agent |

---

## 7. Milestones for v1.0 (tasks and estimates)

**S** = agent sessions. Ranges are honest uncertainty, not padding.

### M0 — Foundation and risk prototype (3–4 S)

| Task | Work | Acceptance |
|---|---|---|
| T0.0 | **You:** approve installing rustup (stable, MSVC) and VS Build Tools (C++ workload) | `cargo --version` works |
| T0.1 | Workspace, both crates, toolchain file, lints, pinned dependencies, CI, `docs/` skeleton | Empty app opens; CI is green |
| T0.2 | Operator shell with all panels as placeholders, the theme, and `Action`/`apply` wiring | Screenshot matches the §4 layout |
| T0.3 | `platform/windows.rs` monitor list, plus a display selector in the toolbar | All attached monitors are listed with correct rectangles |
| T0.4 | Output viewport on the chosen monitor, showing a hard-coded slide | Fullscreen on display 2 |
| T0.5 | Bundled fonts, `TextMeasurer`, `layout_slide`, `paint_slide`; parity test | Same line breaks at 3 scales; Chinese text renders |

**Gate G0 (you):** run the app with a second screen. Check the slide is fullscreen on the correct screen, the Chinese text renders, and the line height looks right.

### M1 — Songs and storage (3–4 S)

| Task | Work | Acceptance |
|---|---|---|
| T1.1 | Domain types, IDs, `parse_lyrics` with table-driven tests | All parsing rules in §3.4 are covered |
| T1.2 | `Library` song operations and validation | Errors for an empty title or no lyrics; duplicate titles are fine |
| T1.3 | Storage schema v1, `JsonRepository` (atomic save, backup, corrupt-file handling) | Round-trip, corrupt-file and failed-save tests pass |
| T1.4 | Song editor dialog, library panel and search, thumbnails, autosave | Add and edit songs through the UI |

**Gate G1:** add 3 real songs, including one in Chinese. Close and reopen the app. Every slide should be identical. Corrupt `library.json` by hand and confirm recovery from `.bak`.

### M2 — Live operation (2–3 S) — first usable milestone

| Task | Work | Acceptance |
|---|---|---|
| T2.1 | `LiveState` and its rules from §3.5, with full unit tests | All rules are tested |
| T2.2 | Thumbnail click goes live; preview-only selection; both previews; status badges | Selection never changes live output |
| T2.3 | Next, Previous, Clear, Blackout buttons and keyboard shortcuts with focus rules | Shortcuts are ignored while typing |
| T2.4 | Display disconnect and closing the output window | Operator app survives; warning is shown |

**Gate G2:** present a whole song from the laptop to the projector, using a clicker if you have one. Edit a typo while the song is live and confirm the projector doesn't change until you press Next. Unplug the projector and plug it back in.

### M3 — Playlists (1–2 S)

| Task | Work | Acceptance |
|---|---|---|
| T3.1 | `Library` playlist operations and the reference-deletion invariant | Repeated entries work; deleting a song removes its entries |
| T3.2 | Playlist panel, add to playlist, move up/down, remove, delete confirmation listing affected playlists | Same song in 2 playlists with no duplication |

**Gate G3:** build next Sunday's service as a playlist.

### M4 — Styles and templates (2–3 S)

| Task | Work | Acceptance |
|---|---|---|
| T4.1 | Style panel controls, with ranges validated against §3.4 limits | All slides of the song update live in the preview |
| T4.2 | Templates: default "Centered Lyrics", Save as Template, apply by copying | Editing a template doesn't change existing songs |
| T4.3 | Overflow badge and warning text | Long lines show the badge; the font is never shrunk |

**Gate G4:** set up a style that is readable on the real projector and save it as a template.

### M5 — Hardening and release (2–3 S)

| Task | Work | Acceptance |
|---|---|---|
| T5.1 | DPI review, minimum panel sizes, error-message pass | Clean at 100%, 125% and 150% scaling |
| T5.2 | Independent code review (fresh-context sub-agent); fix findings | No high-severity findings left |
| T5.3 | Release build: icon, version info, portable zip (exe, assets, licences) | Runs on a clean Windows machine from the zip |
| T5.4 | `CHANGELOG.md`, short user guide (`docs/USER_GUIDE.md`) | — |

**Gate G5 = v1.0 release criterion:** a full offline rehearsal on the church laptop and projector. Prepare a playlist, restart the app, present every song, clear, blackout, and close the output. There should be no data loss and no unexpected output changes.

---

## 8. Timeline

Assumptions: the agent works in 2–4 sessions a week, and you run a gate within about 2 days of being asked. **Calendar dates move with gate turnaround, not coding.** Week 1 starts the day T0.0 is done.

| Week | Agent work | Your gate |
|---|---|---|
| 1 | M0 (T0.1–T0.5) | **G0**, end of week — the go/no-go on the main risks R1 and R2 |
| 2 | M1 | **G1** |
| 3 | M2 | **G2** — first usable version |
| 4 | M3 + start M4 | **G3** |
| 5 | M4 | **G4** |
| 6 | M5 | **G5** — **v1.0** |

Total: **13–19 agent sessions, about 6 weeks**, with roughly 30–60 minutes of your time per gate. If G0 fails (R1 or R2), the agent stops and proposes an alternative, for example a different text-layout library or a different windowing approach, before continuing. At most 1 week is lost.

---

## 9. Roadmap after v1.0

Each release is planned in detail (tasks and gates as in §7) only when it becomes the next one. Estimates below are rough.

| Release | Theme | Features | Estimate | Key risk or decision |
|---|---|---|---|---|
| **v1.1** | Quality of life | Drag-and-drop reordering; picking installed fonts (`fontdb`); undo/redo for library edits; searching lyrics text; duplicating a song; importing plain `.txt` files; on-screen shortcut help | 4–6 S, about 2 weeks | Low |
| **v1.2** | Backgrounds and transitions | Image backgrounds per template (PNG/JPEG via the `image` crate); fade transition between slides; text shadow and outline for readability over images | 4–5 S, about 2 weeks | Layout and transition interaction |
| **v1.3** | Stage display | Optional third screen or window: current and next slide, clock, section label | 3–4 S, about 1.5 weeks | Needs 3 outputs at once — test hardware early |
| **v2.0** | Rich services | Per-slide style overrides; bilingual layouts (two text areas, for example English and Chinese); announcement and text-only playlist items | 6–8 S, about 3 weeks | Data model change; schema v2 migration |
| **v2.1** | Scripture | Local public-domain Bible text (KJV, WEB, CUV for Chinese) imported from files; passage lookup to slides | 4–6 S, about 2 weeks | Licensing of translations — only public domain or ones you are allowed to use |
| **v2.2** | Video backgrounds | Looping video behind lyrics | 6–10 S | **Decision needed:** pure-Rust video decoding is limited. Either allow the Windows Media Foundation API (OS component, consistent with §1.3) or drop video |
| Later | Ideas | Importing your own ProPresenter files; CCLI usage report; phone remote control (needs a local network server and authentication — security review); NDI output | — | Each needs its own decision record |

### 9.3 Backlog intake

New ideas are added to `docs/STATUS.md` → Backlog, with a one-line value and a rough size. You decide which release they go in. The agent does not pull backlog items into an active milestone without your approval.

---

## 10. Immediate next steps

1. **You:** approve T0.0, installing the Rust toolchain and VS Build Tools on this machine (about 5 GB, can be undone).
2. **Agent:** create the `presenter-flow` git repo with the workspace from §3.1 and start M0.
3. **You:** have a second monitor or projector available at the end of week 1 for G0.
