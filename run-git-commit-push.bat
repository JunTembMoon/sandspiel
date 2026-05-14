@echo off
setlocal

cd /d "%~dp0"
call "%~dp0git-commit-push.bat"

if errorlevel 1 (
  echo.
  echo [FAIL] Commit/push failed. Press any key to close.
  pause >nul
  exit /b 1
)

echo.
echo [DONE] Commit/push completed. Press any key to close.
pause >nul
exit /b 0
