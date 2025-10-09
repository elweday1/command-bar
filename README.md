# Dossier

![Dossier-Cover](./dossier-cover.jpg)
A modern, extensible command launcher built with Tauri 2 and Svelte 5. Dossier provides a fast, keyboard-driven interface to search and execute commands, launch applications, and interact with various plugins.

## Features

- **Plugin System**: Extensible architecture with support for custom plugins
- **Global Shortcuts**: Quick access from anywhere on your system
- **Modern UI**: Built with Svelte 5 and shadcn-svelte components
- **Cross-Platform**: Runs on Windows, macOS, and Linux
- **Fast Search**: Instant results with fuzzy matching
- **Customizable**: Settings window for configuration

## Development

### Start development server

```bash
npm run tauri dev
```

### Build for production

```bash
npm run tauri build
```

### Code quality

```bash
npm run lint    # Check code style
npm run format  # Format code
npm run check   # Type checking
```

## Usage

- Press the global shortcut to open Dossier
- Type to search for commands, applications, or use plugin prefixes
- Use arrow keys to navigate results
- Press Enter to execute the selected action
- Press Escape to close

## Plugin Development

Dossier supports custom plugins. Check the `src/lib/plugins.ts` file for the plugin interface and examples.

## Tech Stack

- **Frontend**: Svelte 5, TypeScript, Tailwind CSS 4
- **Backend**: Rust, Tauri 2
- **UI Components**: shadcn-svelte
- **Build Tool**: Vite

## License

MIT License - see [LICENSE](LICENSE) file for details.
