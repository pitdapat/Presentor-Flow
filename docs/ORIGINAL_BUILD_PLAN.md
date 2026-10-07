# Presenter Flow — first-version build plan

Prepared 7 October 2026. This is a proposed design and implementation plan, not a working application.

## 1. Goal and agreed scope

Build a simple, offline Windows desktop application for preparing and presenting church song lyrics. Keep the familiar presentation workflow and rough panel arrangement of ProPresenter, with original Presenter Flow branding, and iterate after using the first version.

All application logic and UI code will be Rust. Use Rust crates and normal Windows graphics/window APIs; “entirely Rust” means the application has no JavaScript, HTML frontend, Electron, or application code written in another language. It does not mean replacing operating-system components and graphics drivers.

The first version includes:

- Add and edit songs by typing or pasting lyrics.
- A full song library, separate from ordered service playlists.
- Click a slide to present it on a separate monitor or projector.
- Separate selected-slide preview and live-output preview.
- Font, font size, line height, letter spacing, alignment, text position, text color, and solid background color.
- Named, reusable templates containing those slide-style settings.
- Previous/next slide controls, clear lyrics, and blackout.
- Local saving and reopening of songs, playlists, templates, and settings.

Keep video, image backgrounds, Bible integration, cloud accounts, automatic song imports, stage displays, transitions, and dedicated bilingual layouts for later versions. Ordinary Unicode text should be preserved; Chinese font coverage should be checked even without a dedicated bilingual mode.

## 2. Main interface

Use a dark charcoal operator interface with restrained teal selection highlights and red live indicators. Panels should resize, with sensible minimum widths. Begin with a 16:9 slide canvas, defaulting to 1920 × 1080 logical presentation units.

| Area | Contents | Purpose |
|---|---|---|
| Top toolbar | Add Song, New Playlist, Templates, output display selector | Common preparation and display actions |
| Upper left | Library search, All Songs, song list | Browse every saved song |
| Lower left | Playlist list and ordered songs in the chosen playlist | Prepare a service without duplicating library songs |
| Center | Selected song title, Edit Lyrics, slide thumbnails | Navigate a song visually |
| Right, upper | Selected-slide preview and separate live-output preview | Make preparation and live state obvious |
| Right, lower | Template selector and slide-style controls | Format the selected song |
| Bottom | Previous, Next, Clear Lyrics, Blackout; save and output status | Operate the service with clear feedback |

The concept mockup shows slide 1 selected for preview while slide 2 remains live. This is intentional: selected content and live content are separate states.

### Selection and live behavior

- Selecting a song in the library or playlist loads its thumbnails; it does not replace live output.
- Single-clicking a thumbnail takes that slide live and also selects it.
- A small preview action or context-menu action selects a thumbnail without taking it live. Keyboard focus and the selected border must remain visible.
- Editing lyrics or styles affects the working song and its preview. Live output retains a snapshot until a slide is explicitly presented again.
- Clear Lyrics removes live text but keeps the current background.
- Blackout overrides the output with black. Leaving blackout restores the current live snapshot; display a prominent blackout indicator.
- Previous/Next stays within the live song. At the first or last slide it stops; it does not accidentally jump into another song.
- Arrow shortcuts operate presentation only when a text field is not focused. Visible buttons remain available throughout editing.

## 3. Song and playlist workflows

### Add a song

1. Click Add Song.
2. Enter a required title and paste or type lyrics into a plain multiline editor.
3. Separate slides using one or more blank lines. Preserve nonblank line breaks inside each slide.
4. Optionally add a section label on its own line, such as `[Verse 1]` or `[Chorus]`. Labels organize slides and never appear on the projector. A label starts a new section; unlabeled slides continue the current section.
5. Check generated slide previews and any overflow warnings.
6. Choose a template and save the song to the library.

Trim blank leading/trailing lines and ignore empty slides. Preserve repeated lyric lines and repeated sections. Reject empty titles and songs without lyric content; duplicate titles are allowed because songs have distinct IDs.

### Prepare a playlist

1. Create a named playlist, for example Sunday Service.
2. Find songs in the library and use Add to Playlist.
3. Reorder playlist entries using Move Up/Move Down initially; drag-and-drop can be added once the core workflow is stable.
4. Select each playlist song to view its slides.

Playlist entries reference library song IDs rather than storing song copies. The same song can appear more than once in a playlist; each occurrence has its own entry ID. Editing the library song updates its playlist appearances. Removing an entry only removes that occurrence. Deleting a library song requires a confirmation explaining affected playlists and removes its references consistently; it must not disturb an already-live snapshot.

## 4. Text styles and templates

Keep one lyric text area per slide and one active style per song in the first version. This prevents a complex multi-object slide editor from becoming necessary.

| Control | First-version behavior |
|---|---|
| Font | Select supported installed fonts; load a tested default and show fallback warnings |
| Font size | Size in logical presentation units, independent of operator-window scaling |
| Line height | Multiplier applied to the normal line advance, e.g. 1.2 |
| Letter spacing | Additional logical spacing between rendered glyphs; validate shaping and Unicode behavior |
| Alignment | Left, center, or right within the text area |
| Position | Top, middle, or bottom within a fixed safe-margin area |
| Colors | Text color and solid background color |
| Save as Template | Name and save a copy of the current style |

Ship a single default template, Centered Lyrics: white text, black background, centered alignment and position, 64-unit font size, 1.2 line height, zero added letter spacing, and safe margins. Validate these defaults against real projector output.

Applying a template copies its style into the song. Later template edits do not silently reformat existing songs. Style changes apply to every slide in the selected song; per-slide overrides can wait.

Wrap text using the shared layout engine and show an overflow warning when text cannot fit vertically. Do not silently shrink the font. Thumbnails, preview, and projector output must use the same layout result and scale it to their available space.

## 5. Proposed Rust technology

Use `egui` with `eframe` for the desktop UI and native output window. The project documents egui as a Rust GUI, and native eframe supports multiple viewports. This makes it a practical starting point for the operator window plus projector output, subject to a Windows monitor-placement prototype.

- `serde` and `serde_json`: versioned local data files.
- A Rust UUID crate: stable identifiers for songs, slides, templates, and playlist occurrences.
- `thiserror`: typed errors with helpful UI messages.
- `tracing`: diagnostic logging.
- Rust font discovery/loading support: select installed fonts and supply a tested fallback.

Pin compatible dependencies and commit `Cargo.lock` when implementation begins. Avoid adding a database or asynchronous runtime until there is a concrete need. Keep font handling and rendering behind small interfaces so they can be improved without rewriting the library and playlist code.

First investigate text layout, especially custom line height, letter spacing, wrapping, and Chinese glyph coverage. Do not assume a standard text widget provides presentation-quality controls. If needed, implement a dedicated Rust text-layout adapter and render its positioned glyphs consistently across all views.

Documentation checked for this proposal:

- https://github.com/emilk/egui
- https://docs.rs/egui/latest/egui/viewport/index.html

## 6. Code organization

Start with one Cargo package and clear modules. This keeps setup small while separating responsibilities. `main.rs` should only configure logging and launch the application.

| File or directory | Responsibility |
|---|---|
| `Cargo.toml`, `Cargo.lock` | Dependencies and reproducible builds |
| `src/main.rs` | Small launch function |
| `src/lib.rs` | Module exports |
| `src/app/mod.rs`, `state.rs`, `actions.rs` | App coordination, operator state, explicit actions |
| `src/domain/song.rs`, `playlist.rs`, `template.rs`, `style.rs` | Data models and domain invariants |
| `src/services/song_service.rs`, `playlist_service.rs`, `template_service.rs` | Validated user operations |
| `src/ui/main_window.rs`, `toolbar.rs`, `library_panel.rs`, `playlist_panel.rs` | Window composition and navigation panels |
| `src/ui/slide_grid.rs`, `preview_panel.rs`, `style_panel.rs` | Slide navigation and formatting UI |
| `src/ui/song_editor.rs`, `template_dialog.rs`, `output_settings.rs` | Focused editors and dialogs |
| `src/presentation/controller.rs`, `live_state.rs`, `output_window.rs` | Live snapshot, navigation, blackout, display lifecycle |
| `src/rendering/slide_renderer.rs`, `text_layout.rs`, `fonts.rs` | Shared slide drawing and typography |
| `src/storage/repository.rs`, `json_repository.rs`, `migration.rs` | Persistence interface, reliable saving, schema upgrades |
| `src/platform/windows.rs` | Windows-specific display/font integration when required |
| `src/error.rs` | Application error types |
| `tests/` | Behavioral and persistence tests |

UI panels send actions; services validate and change domain data; storage persists it; the presentation controller owns live state. UI files do not directly read/write data files. Domain modules do not depend on egui. One shared renderer owns slide geometry rather than three independent renderers.

### Coding conventions

- Rust naming: `snake_case` functions/modules, `PascalCase` types, `SCREAMING_SNAKE_CASE` constants.
- Small cohesive functions, explicit types at boundaries, and minimal public visibility.
- Use enums for state and actions instead of scattered flags and magic strings.
- Return `Result` for fallible operations. Avoid `unwrap()` or `expect()` on user data, files, fonts, and display discovery.
- Keep application code safe Rust unless a reviewed Windows integration requires a small, documented unsafe boundary.
- Document public interfaces and reasons behind non-obvious behavior.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` in CI.

## 7. Data and saving

Use a versioned JSON document in the Windows per-user application-data directory, with settings separate if helpful. Core records:

- Song: ID, title, ordered slides, copied text style, timestamps.
- Slide: ID, optional section label, lyric text.
- Playlist: ID, name, ordered entries containing entry ID and song ID.
- Template: ID, name, text style.
- Settings: theme, selected output display, canvas ratio, window layout.
- Stored document: schema version and collections above.

Autosave completed operations; debounce ongoing lyric/style edits. Write a temporary file and use a tested Windows-safe replacement procedure. Keep a last-known-good backup, surface save failures, and never overwrite malformed data with an empty library. Save work on normal shutdown; validate IDs and references on load and provide recovery feedback.

Start every launch with output cleared; never automatically resume projecting old lyrics. Display names are not guaranteed stable identifiers, so validate the remembered display and ask the operator to choose again when it is unavailable.

## 8. Build sequence and completion checks

| Phase | Work | Completion check |
|---|---|---|
| 1 — Foundation and risk prototype | Rust project, module boundaries, operator shell, second native output window, typography prototype | A sample lyric slide appears fullscreen on a chosen second display; line height/spacing match preview |
| 2 — Library and songs | Song models, editor/parser, search, JSON saving and recovery | Add/edit a song, restart the app, and recover exactly the saved slides |
| 3 — Playlists | Create, add references, reorder, remove entries | One library song appears in multiple playlists without duplication or accidental deletion |
| 4 — Styles and templates | Controls, shared renderer, overflow feedback, template copy semantics | Save and reapply a template; changing it leaves existing song styles intact |
| 5 — Live operation | Live snapshots, thumbnail clicks, preview-only selection, arrows, clear, blackout | Editing and browsing leave output unchanged; presentation controls behave predictably |
| 6 — Windows release | DPI checks, display disconnect handling, packaging, release build | Complete a rehearsal on an actual laptop and projector, fully offline |

The first usable milestone is phases 1–3 with basic live output from the prototype. The first complete release includes all six phases. Avoid a fixed calendar estimate until the multi-monitor and typography prototype resolves those two main uncertainties.

## 9. Verification before church use

Automated tests should cover lyric splitting/labels, playlist ordering and repeated entries, reference deletion, template copying, live-state isolation, blackout/clear behavior, JSON round trips, corrupt-file recovery, and save-failure handling. Test meaningful behavior rather than every UI widget.

Manually rehearse on Windows with different display scales and output resolutions. Check long lyric lines, missing fonts, Chinese characters, keyboard shortcuts while typing, rapid slide changes, projector disconnect/reconnect, output-window closure, and restart after saving.

If the output display disappears, retain the operator app and show a warning. Do not unexpectedly move fullscreen output onto the operator screen. Closing the output window disables output; it must not terminate the operator app or lose its data.

Release criterion: prepare a small playlist, reopen it, present every song, clear lyrics, use blackout, and complete the rehearsal without data loss or unintended output changes.

## 10. Next action

Review the concept layout and this scope. Then begin phase 1: create the modular Rust project and prove the shared text renderer and second-screen output. Iterate the interface around the resulting working prototype.
