# Plugin Build Scripts

## Node.js Build System (Recommended)

### Install dependencies (first time only):
```bash
npm install
```

### Build all plugins once:
```bash
npm run build
```

### Watch for changes and auto-rebuild:
```bash
npm run watch
```

## Legacy Scripts

### Windows (build.bat):
```cmd
build.bat
build.bat --watch
```

### Linux/macOS (build.sh):
```bash
./build.sh
./build.sh --watch
```

## Output Directory

All built plugin libraries (.dll, .so, .dylib) are copied to:
`~/.config/dossier/plugins/.build/`

## File Watching

The watch mode monitors all `.rs` files in plugin directories and automatically rebuilds when changes are detected.

### Requirements for optimal watching:
- **Linux**: `inotifywait` (install with `sudo apt install inotify-tools`)
- **macOS**: `fswatch` (install with `brew install fswatch`)
- **Windows**: Built-in polling (no additional tools needed)

If the optimal tools aren't available, the scripts fall back to polling mode.