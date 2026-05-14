@echo off
setlocal

echo =========================================
echo Git Commit + Push Helper
echo =========================================

where git >nul 2>nul
if errorlevel 1 (
  echo [ERROR] git command was not found in PATH.
  exit /b 1
)

git rev-parse --is-inside-work-tree >nul 2>nul
if errorlevel 1 (
  echo [ERROR] This folder is not a git repository.
  exit /b 1
)

git add .
if errorlevel 1 (
  echo [ERROR] git add failed.
  exit /b 1
)

set /p COMMIT_MSG=Commit message: 
if "%COMMIT_MSG%"=="" (
  echo [ERROR] Commit message is required.
  exit /b 1
)

git commit -m "%COMMIT_MSG%"
if errorlevel 1 (
  echo [INFO] Commit was not created. (No changes or commit error)
)

set /p REMOTE_NAME=Remote name [origin] : 
if "%REMOTE_NAME%"=="" set "REMOTE_NAME=origin"

set "CURRENT_BRANCH="
for /f "delims=" %%B in ('git branch --show-current') do set "CURRENT_BRANCH=%%B"

if "%CURRENT_BRANCH%"=="" (
  set /p BRANCH_NAME=Branch to push: 
) else (
  set /p BRANCH_NAME=Branch to push [%CURRENT_BRANCH%] : 
)
if "%BRANCH_NAME%"=="" set "BRANCH_NAME=%CURRENT_BRANCH%"

if "%BRANCH_NAME%"=="" (
  echo [ERROR] Branch name is required.
  exit /b 1
)

set /p USE_UPSTREAM=Use -u (set upstream)? [Y/n] : 
if "%USE_UPSTREAM%"=="" set "USE_UPSTREAM=Y"

if /I "%USE_UPSTREAM%"=="Y" (
  git push -u "%REMOTE_NAME%" "%BRANCH_NAME%"
) else (
  git push "%REMOTE_NAME%" "%BRANCH_NAME%"
)

if errorlevel 1 (
  echo [ERROR] git push failed.
  exit /b 1
)

echo [OK] Commit/push flow finished.
exit /b 0
