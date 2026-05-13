# Claude Native Desktop for Linux

A native Linux client for [claude.ai](https://claude.ai) in 117 lines of Rust. No Electron. No Chromium. Uses the system WebKitGTK webview.

## Why

The official Claude Desktop app ships as an Electron wrapper — a full Chromium browser bundled inside an application just to render a chat interface. On older or resource-constrained Linux hardware, this means hundreds of megabytes of RAM consumed before you type a single message.

This project replaces that with a native GTK window and the system WebKitGTK renderer. Same claude.ai interface. Fraction of the resources.

## Features

- **Native GTK window** with system WebKitGTK webview — no bundled browser engine
- **Google OAuth login** via popup window with shared cookie context
- **Persistent sessions** — cookies stored at `~/.local/share/claude-native/webdata/`, login survives app restarts
- **Auto-resize** — GTK handles layout natively, works with maximize/restore/manual resize
- **117 lines of Rust** — the entire application

## Requirements

- Linux with GTK 3 and WebKitGTK 4.1+
- Rust toolchain (rustup.rs)
- System packages:
```bash
# Ubuntu/Debian
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev build-essential

# Fedora
sudo dnf install gtk3-devel webkit2gtk4.1-devel gcc
```

## Build

```bash
cd src-tauri
cargo build --release
```

## Run

```bash
./src-tauri/target/release/claude-desktop
```

Or for development:

```bash
cd src-tauri
cargo run
```

## Desktop Entry

To add to your application launcher:

```bash
cat > ~/.local/share/applications/claude-native.desktop << 'EOF'
[Desktop Entry]
Name=Claude Native
Comment=Native Linux Claude client — no Electron
Exec=/path/to/claude-desktop/src-tauri/target/release/claude-desktop
Icon=applications-internet
Type=Application
Categories=Network;Chat;
EOF
```

## How It Works

The app creates a single GTK window with a WebKitGTK webview pointed at claude.ai. All navigation — Chat, Cowork, Code — is handled by claude.ai's own built-in UI. Google OAuth is handled via a popup window that shares the same web process and cookies as the main view.
Session data persists at `~/.local/share/claude-native/webdata/` so you stay logged in between app restarts.

## What This Is Not

This is a lightweight native wrapper for claude.ai. It does **not** include:

- MCP (Model Context Protocol) server integration
- Terminal emulator / Claude Code integration
- Cowork VM sandbox
- System tray or global hotkeys

Those are future goals. Today it replaces Electron with 117 lines of Rust and gives you the same claude.ai experience with native performance.

## Tested On

- Ubuntu 24.04
- MacBook Pro 17" (2011) — Intel HD 3000, 16GB RAM
- X11 and Wayland (XWayland)

## License

MIT

## Author

Ulysses Isa — [github.com/Ulysses05151997](https://github.com/Ulysses05151997)
