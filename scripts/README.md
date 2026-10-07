# scripts

Helper scripts for running and working on Presenter Flow on Windows. Each
script works from the repo root, wherever it is started from.

| Script | What it does |
|---|---|
| `run.bat` | Builds the debug version if anything changed, then opens the app. Use it to check changes. |
| `run-release.bat` | Same, using the faster release build. Use it for rehearsals. |
| `check.bat` | Local quality gate: format, lint and tests (`check.bat release` adds the release build). Must pass before a task is done. |

The first build takes a minute or two; later runs start in seconds.
