# Bundled fonts

- `NotoSans-Regular.ttf` — Noto Sans (Latin, Greek, Cyrillic)
- `NotoSansSC-Regular.otf` — Noto Sans SC subset (Simplified Chinese), used as the fallback for CJK characters
- `OFL.txt` — SIL Open Font License 1.1, which covers both fonts

Source: the Noto fonts project ([notofonts](https://github.com/notofonts)). Both may be bundled and redistributed under the OFL; they must not be sold on their own.

The fonts are loaded from this folder next to the executable at startup rather than embedded in the binary (risk R3, ADR-0009). A release zip must include `assets\fonts\` beside `presenter-flow.exe`.
