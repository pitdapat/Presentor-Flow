@echo off
rem Local quality gate (PLAN 2.4, ADR-0007).
rem   check.bat          QUICK (default): format, lint, core tests.
rem                      Lint is a check-only build (no code generated), so it
rem                      catches every compile error fast. Core tests need no UI
rem                      libraries, so they build in seconds.
rem   check.bat full     adds the app tests (builds egui/wgpu: slow).
rem                      Run before merging a PR that touches crates\app.
rem   check.bat release  full + the release exe. Run before a release.
setlocal

cd /d "%~dp0.."

where cargo >nul 2>nul
if errorlevel 1 set "PATH=D:\dev\rust\cargo\bin;%PATH%"
where cargo >nul 2>nul
if errorlevel 1 (
    echo Rust ^(cargo^) was not found. Expected it in D:\dev\rust\cargo\bin.
    pause
    exit /b 1
)

set "MODE=%~1"
if "%MODE%"=="" set "MODE=quick"

echo [format]
cargo fmt --all --check || goto :failed
echo [lint]
cargo clippy --workspace --all-targets --locked -- -D warnings || goto :failed

if /i "%MODE%"=="quick" (
    echo [tests: core]
    cargo test -p presenter-core --locked || goto :failed
) else (
    echo [tests: all]
    cargo test --workspace --locked || goto :failed
)

if /i "%MODE%"=="release" (
    echo [release build]
    cargo build --workspace --release --locked || goto :failed
)

echo.
echo All %MODE% checks passed.
endlocal
exit /b 0

:failed
echo.
echo Checks FAILED. See the messages above.
endlocal
exit /b 1
