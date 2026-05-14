@echo off
setlocal EnableExtensions EnableDelayedExpansion

cd /d "%~dp0"

echo =========================================
echo Sandspiel Run Helper
echo =========================================

echo [STEP] Checking npm...
where npm >nul 2>nul
if errorlevel 1 (
  echo [ERROR] npm was not found. Install Node.js first.
  pause
  exit /b 1
)

set "NEED_INSTALL=0"
if not exist "node_modules" set "NEED_INSTALL=1"
if not exist "node_modules\react-firebaseui" set "NEED_INSTALL=1"

if "%NEED_INSTALL%"=="1" (
  echo [STEP] Installing npm dependencies with --legacy-peer-deps...
  npm install --legacy-peer-deps
  if errorlevel 1 (
    echo [ERROR] npm install failed.
    pause
    exit /b 1
  )
) else (
  echo [STEP] npm dependencies already installed.
)

set "CARGO_EXE="
where cargo >nul 2>nul
if errorlevel 1 (
  if exist "%USERPROFILE%\.cargo\bin\cargo.exe" (
    set "CARGO_EXE=%USERPROFILE%\.cargo\bin\cargo.exe"
  )
) else (
  set "CARGO_EXE=cargo"
)

if "%CARGO_EXE%"=="" (
  echo [STEP] Rust toolchain not found. Trying to install via winget...
  where winget >nul 2>nul
  if errorlevel 1 (
    echo [ERROR] winget not found. Install Rust manually: https://rustup.rs/
    pause
    exit /b 1
  )

  winget install -e --id Rustlang.Rustup --accept-package-agreements --accept-source-agreements
  if errorlevel 1 (
    echo [ERROR] Rust installation failed.
    pause
    exit /b 1
  )

  if exist "%USERPROFILE%\.cargo\bin\cargo.exe" (
    set "CARGO_EXE=%USERPROFILE%\.cargo\bin\cargo.exe"
  ) else (
    echo [ERROR] cargo.exe not found after install. Restart terminal and rerun.
    pause
    exit /b 1
  )
)

if exist "crate\Cargo.toml" (
  echo [STEP] Checking wasm-pack...
  set "WASM_PACK_EXE="
  where wasm-pack >nul 2>nul
  if errorlevel 1 (
    if exist "%USERPROFILE%\.cargo\bin\wasm-pack.exe" (
      set "WASM_PACK_EXE=%USERPROFILE%\.cargo\bin\wasm-pack.exe"
    )
  ) else (
    set "WASM_PACK_EXE=wasm-pack"
  )

  if "!WASM_PACK_EXE!"=="" (
    echo [STEP] wasm-pack not found. Installing with cargo...
    "%CARGO_EXE%" install wasm-pack
    if errorlevel 1 (
      echo [ERROR] wasm-pack installation failed.
      pause
      exit /b 1
    )

    if exist "%USERPROFILE%\.cargo\bin\wasm-pack.exe" (
      set "WASM_PACK_EXE=%USERPROFILE%\.cargo\bin\wasm-pack.exe"
    ) else (
      where wasm-pack >nul 2>nul
      if not errorlevel 1 set "WASM_PACK_EXE=wasm-pack"
    )
  )

  if "!WASM_PACK_EXE!"=="" (
    echo [ERROR] wasm-pack still not found after install.
    pause
    exit /b 1
  ) else (
    echo [STEP] Building wasm package...
    pushd crate
    "!WASM_PACK_EXE!" build
    if errorlevel 1 (
      popd
      echo [ERROR] wasm-pack build failed.
      pause
      exit /b 1
    )
    popd
  )
)

echo [STEP] Starting Sandspiel dev server...
echo [INFO] Opening http://localhost:8080
start "" "http://localhost:8080"
npm run start

echo [INFO] Dev server stopped.
exit /b %errorlevel%
