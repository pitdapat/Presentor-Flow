# Changelog

User-visible changes per release. Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Project skeleton: operator window with placeholder panels and a working Blackout toggle.
- Operator shell (T0.2): dark charcoal theme with teal selection and red live colors, the full PLAN §4 layout with resizable split columns, preview and live-output frames with a LIVE / BLACKOUT / CLEARED badge, and a red blackout banner. Buttons for later tasks are shown disabled with the task that enables them.
- Basic functionality (ADR-0009):
  - Songs: Add Song / Edit Lyrics dialog (blank line = new slide, `[Chorus]` names a section), delete with confirmation, title search, slide thumbnails with a "too long" warning.
  - Library is saved automatically to `%APPDATA%\PresenterFlow\library.json`, with a backup of the previous save; a damaged file is set aside and the backup restored.
  - Presenting: click a slide to send it live, right-click or Alt+click to preview only; Previous / Next / Clear Lyrics / Blackout buttons and keys (arrows, Space, PageUp/PageDown, C, B).
  - Output: choose a display in the toolbar and switch Output on. A second monitor gets fullscreen output; this screen gets a test window. Esc or closing the output window turns output off; unplugging the display closes it with a warning.
  - Noto Sans + Noto Sans SC fonts, so Chinese displays correctly; long Chinese lines wrap.
  - Three sample songs on first launch.
