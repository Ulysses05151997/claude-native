# Claude Native Desktop — Project Memory

## What This Is
A native Linux Claude Desktop client. No Electron. No Chromium. Uses Rust + wry (WebKitGTK) + tao + GTK.
Built by Ulysses Isa (GitHub: Ulysses05151997). Part of the IdleCloud project ecosystem.

## Owner
Ulysses Isa — JD, former Senate Chief of Staff. Linux on a 2011 MacBook Pro 17" (MacBookPro8,3),
Ubuntu 24.04, Intel HD 3000 (AMD Radeon disabled), 16GB RAM. Not a hobbyist — this ships to GitHub.

## Related Repositories (same author)
- `ncs2-boot` — Intel NCS2 USB boot tool in C, reverse-engineered from deprecated OpenVINO source
- `broadcom-wl-scan-fix` — DMA fix for Broadcom wl driver on Linux kernel 6.17+

## Project Location
`/home/isaulysses/projects/claude-desktop/`

## Architecture
- `src-tauri/src/main.rs` — entire application
- Dependencies: tao 0.34, wry 0.54, gtk 0.18, urlencoding 2, open 5
- NO Tauri framework — uses the underlying libraries directly
- Builds with `cargo build` from `src-tauri/`

## Current State (May 2026)
Working proof of concept. Chat works. Known bugs documented in CLAUDE_NATIVE_HANDOVER.md.
## Rules for Claude Code

### DO
- Read CLAUDE_NATIVE_HANDOVER.md before starting any work
- Back up files before modifying them
- Build and test after every change — `cargo build` from src-tauri/
- Write clean, commented Rust code — this will be public on GitHub
- Update this file when you complete a phase or make architectural decisions
- Commit working states with descriptive messages

### DO NOT
- Do not add Tauri framework — stay with raw tao/wry/gtk
- Do not add MCP, terminal emulation, or Cowork VM — those are future phases
- Do not add npm, webpack, or any JS build tooling
- Do not create multiple webviews — single webview architecture only
- Do not overengineer — the whole app should be under 300 lines of Rust
- Do not lose work — commit to git after every successful phase

### CRITICAL
If you complete work and do not commit it, it will be lost. Ulysses has already
lost project history from prior Claude Code sessions that did not save state.
After every successful build and test, run:
```
git add -A && git commit -m "descriptive message"
```

## Build
```bash
cd /home/isaulysses/projects/claude-desktop/src-tauri
cargo build 2>&1
./target/debug/claude-desktop
```

## Session Log
<!-- Claude Code: append a one-line summary after each session -->
