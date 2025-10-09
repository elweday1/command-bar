@echo off
if "%~1"=="" (
    echo Usage: %0 ^<svg-file^>
    exit /b 1
)

set SVG_FILE=%~1
set ICONS_DIR=src-tauri\icons

if not exist "%SVG_FILE%" (
    echo Error: SVG file '%SVG_FILE%' not found
    exit /b 1
)

if not exist "%ICONS_DIR%" mkdir "%ICONS_DIR%"

magick convert "%SVG_FILE%" -resize 32x32 "%ICONS_DIR%\32x32.png"
magick convert "%SVG_FILE%" -resize 128x128 "%ICONS_DIR%\128x128.png"
magick convert "%SVG_FILE%" -resize 256x256 "%ICONS_DIR%\128x128@2x.png"
magick convert "%SVG_FILE%" -resize 512x512 "%ICONS_DIR%\icon.png"
magick convert "%SVG_FILE%" -resize 30x30 "%ICONS_DIR%\Square30x30Logo.png"
magick convert "%SVG_FILE%" -resize 44x44 "%ICONS_DIR%\Square44x44Logo.png"
magick convert "%SVG_FILE%" -resize 71x71 "%ICONS_DIR%\Square71x71Logo.png"
magick convert "%SVG_FILE%" -resize 89x89 "%ICONS_DIR%\Square89x89Logo.png"
magick convert "%SVG_FILE%" -resize 107x107 "%ICONS_DIR%\Square107x107Logo.png"
magick convert "%SVG_FILE%" -resize 142x142 "%ICONS_DIR%\Square142x142Logo.png"
magick convert "%SVG_FILE%" -resize 150x150 "%ICONS_DIR%\Square150x150Logo.png"
magick convert "%SVG_FILE%" -resize 284x284 "%ICONS_DIR%\Square284x284Logo.png"
magick convert "%SVG_FILE%" -resize 310x310 "%ICONS_DIR%\Square310x310Logo.png"
magick convert "%SVG_FILE%" -resize 50x50 "%ICONS_DIR%\StoreLogo.png"
magick convert "%SVG_FILE%" -resize 256x256 -define icon:auto-resize=256,128,64,48,32,16 "%ICONS_DIR%\icon.ico"
magick convert "%SVG_FILE%" -resize 1024x1024 "%ICONS_DIR%\icon.icns"

echo Icons generated successfully in %ICONS_DIR%