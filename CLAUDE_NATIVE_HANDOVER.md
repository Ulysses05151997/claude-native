# Claude Native Desktop — Handover for Claude Code

## Project Location
`/home/isaulysses/projects/claude-desktop/`

## What This Is
A native Linux Claude client using Rust + wry (WebKitGTK) + tao (windowing) + GTK. 225 lines of Rust. No Electron. No Chromium. Uses the system WebView to load claude.ai. Currently a working proof of concept with critical bugs.

## What Works
- Window renders on Intel HD 3000 integrated graphics (AMD Radeon disabled)
- claude.ai loads and renders fully in WebKitGTK
- Authentication works via email login
- Chat is functional — can read and send messages
- Enter key submits messages
- Ubuntu 24.04, 2011 MacBook Pro 17" (MacBookPro8,3), 16GB RAM

## Architecture (Current)
- Three separate webviews: Chat, Cowork, Code
- Native GTK tab bar across the top switches between them
- Chat and Cowork both load claude.ai independently
- Code loads a static HTML placeholder
- Each webview has its own isolated cookie store (ephemeral)

---

## BUG LIST

### BUG 1 — Resize/Restore Layout Breaks
After maximizing and restoring the window, the webview bounds do not recalculate correctly. The footer becomes oversized and the left sidebar clips. The `WindowEvent::Resized` handler reads dimensions from `fixed.allocation()` but does not properly account for the native tab bar height or the window manager's decorations.

### BUG 2 — Native Tabs Conflict with claude.ai Navigation
claude.ai has its own built-in Chat/Cowork/Code tabs. The native GTK tab bar duplicates this. Clicking the native Cowork or Code tab triggers claude.ai's internal project selection popup because each webview is a separate session navigating to claude.ai independently.

### BUG 3 — Triple Login Required
Each of the three webviews has its own cookie store. User must log in three times. Google OAuth ("Continue with Google") fails in all three — WebKitGTK is not handling the popup/redirect chain.

### BUG 4 — No Persistent Sessions
Cookies are ephemeral (in-memory). Closing the app loses all sessions. User must re-authenticate with email verification every time the app launches.

### BUG 5 — Code Tab Is Empty
The Code tab renders a blank white page. The placeholder HTML from the source exists but is not visible — likely a `set_visible` or bounds issue with the 1x1 initial size.

### BUG 6 — Cowork Is Fake
The Cowork tab is just a second claude.ai webview. There is no VM, no sandbox, no bwrap, no filesystem access. It's identical to Chat.

---

## FIX PLAN — STRICT INSTRUCTIONS

### PHASE 1: Single Webview Architecture (DO THIS FIRST)
**DO:**
- Remove the three-webview architecture entirely
- Use ONE webview loading claude.ai
- Remove the native GTK tab bar — let claude.ai's built-in Chat/Cowork/Code navigation handle tab switching
- This eliminates BUG 2, BUG 3, BUG 5, and BUG 6 in one pass

**DO NOT:**
- Do not try to make three webviews work properly
- Do not try to build real Cowork (VM/sandbox) or real Code (terminal emulator) — those are future phases
- Do not add features — simplify first

### PHASE 2: Persistent Cookie Storage

**DO:**
- Configure wry/WebKitGTK to use a persistent data store
- Store cookies, local storage, and session data in `~/.local/share/claude-native/webdata/`
- On next launch, the user should still be logged in without re-authenticating

**DO NOT:**
- Do not store credentials in plaintext
- Do not create a custom auth system — just persist WebKitGTK's own data store

### PHASE 3: Google OAuth Fix

**DO:**
- Handle the Google OAuth popup/redirect flow inside the webview
- wry needs to handle `new_window` requests — Google OAuth opens a popup- Capture the popup URL, either open it in the same webview or handle the redirect chain
- The user is already logged into Google on the system — the OAuth flow should see their existing Google session via the persistent cookie store from Phase 2

**DO NOT:**
- Do not launch an external browser for login
- Do not bypass or fake the OAuth flow
- Do not intercept or log credentials

### PHASE 4: Resize Fix

**DO:**
- Fix the `WindowEvent::Resized` handler to properly size the webview to fill the entire content area below the window title bar
- With the native tab bar removed (Phase 1), the webview should fill the entire window
- Handle maximize, restore, and manual resize correctly
- Test on both X11 and Wayland (XWayland)

**DO NOT:**
- Do not hardcode pixel values
- Do not assume a specific screen resolution

---

## DO NOT DO ANY OF THE FOLLOWING (FUTURE PHASES — NOT NOW)

- Do not add MCP client functionality
- Do not add terminal emulator integration
- Do not attempt real Cowork sandbox/VM functionality- Do not add system tray integration
- Do not add global hotkeys
- Do not refactor to Tauri framework — stay with raw tao/wry/gtk
- Do not add build system complexity (no npm, no webpack, no bundler)
- Do not change the dependency stack — tao, wry, gtk, urlencoding, open

---

## SUCCESS CRITERIA

When done, this app should:
1. Open a single window that loads claude.ai
2. Let claude.ai's own navigation handle Chat/Cowork/Code switching
3. Remember the user's session between app restarts
4. Allow login via "Continue with Google" without errors
5. Resize cleanly at any window size including maximize/restore
6. Be under 300 lines of Rust

---

## BUILD AND TEST

```bash
cd /home/isaulysses/projects/claude-desktop/src-tauri
cargo build 2>&1
./target/debug/claude-desktop
```

## BEFORE MAKING ANY CHANGES

```bash
cd /home/isaulysses/projects/claude-desktop
cp -r src-tauri/src src-tauri/src.bak
```

## GitHub
Owner: Ulysses05151997
This will be published as a public repository. Write clean code with comments.