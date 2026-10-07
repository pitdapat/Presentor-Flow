@echo off
rem Local quality gate: the same checks CI runs (PLAN 2.4, ADR-0007).
rem Run before every commit that completes a task.
rem   check.bat          fmt, clippy, tests
rem   check.bat release  also builds the release exe
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

echo [1/4] Format
cargo fmt --all --check || goto :failed
echo [2/4] Clippy
cargo clippy --workspace --all-targets --locked -- -D warnings || goto :failed
echo [3/4] Tests
cargo test --workspace --locked || goto :failed
if /i "%~1"=="release" (
    echo [4/4] Release build
    cargo build --workspace --release --locked || goto :failed
) else (
    echo [4/4] Release build skipped ^(pass "release" to include it^)
)

echo.
echo All checks passed.
endlocal
exit /b 0

:failed
echo.
echo Checks FAILED. See the messages above.
endlocal
exit /b 1
