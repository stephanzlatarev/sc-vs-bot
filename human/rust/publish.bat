@echo off
setlocal

set SIGNTOOL="C:\Program Files (x86)\Windows Kits\10\bin\10.0.28000.0\x86\signtool.exe"
set BIN=target\release\rust.exe
set EXE=target\sc-vs-bot.exe

echo [1/4] Building release binary...
cargo build --release
if errorlevel 1 goto :fail

if not exist "%BIN%" (
  echo ERROR: Expected output not found: %BIN%
  goto :fail
)

echo [2/4] Copying binary to %EXE%...
copy /Y "%BIN%" "%EXE%" >nul
if errorlevel 1 goto :fail

echo [3/4] Signing executable...
%SIGNTOOL% sign /fd SHA256 /f sc-vs-bot.pfx /p "Abcd1234" "%EXE%"
if errorlevel 1 goto :fail

echo Publish finished: %EXE%
goto :ok

:fail
echo Publish failed.
exit /b 1

:ok
exit /b 0
