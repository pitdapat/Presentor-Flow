# scripts

Helper scripts for running and working on Presenter Flow on Windows. Each
script works from the repo root, wherever it is started from.

| Script | What it does |
|---|---|
| `run.bat` | Builds the debug version if anything changed, then opens the app. Use it to check changes. |
| `run-release.bat` | Same, using the faster release build. Use it for rehearsals. |
| `check.bat` | Quick local gate: format, lint (check-only build) and core tests. Run after every change. |
| `check.bat full` | Adds the app tests. Must pass before merging a PR that changes `crates\app`. |
| `check.bat release` | Full checks plus the release build. Run before a release. |

The first build takes a minute or two; later runs start in seconds.
