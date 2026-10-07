@echo off
rem Builds (if needed) and opens Presenter Flow.
rem   run.bat           debug build: compiles fastest, fine for checking changes
rem   run.bat release   release build: smoother, use for rehearsals
setlocal

rem Always work from the repo root, wherever this file is started from.
cd /d "%~dp0.."

rem Rust is installed on D: (PLAN T0.0). Add it to PATH if this window
rem was opened before the install.
where cargo >nul 2>nul
if errorlevel 1 set "PATH=D:\dev\rust\cargo\bin;%PATH%"
where cargo >nul 2>nul
if errorlevel 1 (
    echo Rust ^(cargo^) was not found. Expected it in D:\dev\rust\cargo\bin.
    pause
    exit /b 1
)

set "PROFILE_FLAG="
if /i "%~1"=="release" set "PROFILE_FLAG=--release"

echo Building and starting Presenter Flow %PROFILE_FLAG% ...
cargo run %PROFILE_FLAG% -p presenter-flow
if errorlevel 1 (
    echo.
    echo Presenter Flow failed to build or exited with an error. See the messages above.
    pause
    exit /b 1
)
endlocal
