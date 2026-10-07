# Human gates

Run by the product owner on real hardware. Record each attempt; failures become bug tasks.

Result format: `YYYY-MM-DD — PASS` or `YYYY-MM-DD — FAIL: <what failed>`.

## G0 — Second screen and text rendering

- [ ] Output opens fullscreen on the chosen (second) screen, not the operator screen
- [ ] Chinese text renders (no boxes or missing glyphs)
- [ ] Line height and letter spacing look correct
- [ ] Line breaks match between preview and projector

Results:

## G1 — Songs and storage

- [ ] Add 3 real songs, one in Chinese
- [ ] Close and reopen: every slide identical
- [ ] Hand-corrupt `library.json`: app recovers from `.bak` and tells you

Results:

## G2 — Live operation

- [ ] Present a whole song (with a clicker if available)
- [ ] Fix a typo while live: projector unchanged until Next
- [ ] Clear and Blackout behave as expected
- [ ] Unplug and replug the projector: operator app survives, warning shown

Results:

## G3 — Playlists

- [ ] Build next Sunday's service as a playlist
- [ ] Same song in two places works; deleting a song shows affected playlists

Results:

## G4 — Styles and templates

- [ ] A style readable at the back of the room, saved as a template
- [ ] Editing the template does not change existing songs

Results:

## G5 — v1.0 release

- [ ] Full offline rehearsal on the church laptop and projector
- [ ] Prepare playlist → restart app → present every song → clear → blackout → close output
- [ ] No data loss, no unexpected output changes

Results:
