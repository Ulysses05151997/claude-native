# Claude Native — Phase 5 & 6: Claude Code + Cowork (Hard Mode)

## REALITY CHECK
These are reverse engineering projects. No public documentation exists for how
the Electron app integrates Claude Code or how Cowork's sandbox works internally.
Everything must be observed, documented, then rebuilt. This is weeks-to-months,
not days. Break it into reconnaissance first, implementation second.

---

## PHASE 5: CLAUDE CODE INTEGRATION (HARD MODE)

### What the Electron app actually does
The Code tab is NOT just a terminal running `claude`. It is a web UI that:
- Spawns Claude Code as a child process
- Communicates via a local protocol (likely JSON-RPC or custom IPC)
- Renders diffs inline with syntax highlighting
- Shows file previews and lets you approve/reject changes
- Manages multiple sessions in a sidebar
- Streams Claude's responses in real-time
- Has its own permission model (approve/deny tool calls)

### RECON PHASE (DO THIS FIRST — NO CODE)
1. Run the Electron app with the Code tab open
2. Find the Claude Code process: `ps aux | grep claude`
3. Identify the IPC mechanism:   - Check for Unix sockets: `ls /tmp/ | grep claude`
   - Check for named pipes: `find /tmp -name '*claude*' 2>/dev/null`
   - Check stdio: `ls -la /proc/$(pgrep -f "claude-code")/fd/`
   - Strace the spawned process: `strace -f -e trace=network,read,write -p PID`
4. Capture the protocol:
   - Use strace or `socat` to intercept communication between Electron and Claude Code
   - Document message format (JSON-RPC? Custom protocol? HTTP?)
   - Identify: how sessions start, how prompts are sent, how responses stream back
   - Identify: how tool calls are proposed and approved/denied
   - Identify: how diffs are transmitted and rendered
5. Examine the Electron app's source:
   - `asar extract` the app.asar from the installed Electron app
   - Find the Code tab's JavaScript — it contains the client-side protocol implementation
   - This is the rosetta stone — the actual client code that talks to Claude Code
6. Document everything in `docs/claude-code-protocol.md` before writing any Rust

### IMPLEMENTATION PHASES

#### 5A: Process Management
- Spawn Claude Code as a child process from Rust
- Establish IPC channel (match whatever protocol was discovered in recon)
- Handle process lifecycle: start, stop, crash recovery
- Store session state

#### 5B: Message Protocol
- Implement the client side of the IPC protocol in Rust
- Send prompts, receive streaming responses
- Handle tool call approval/denial flow

#### 5C: UI — Diff Rendering
- Render diffs inline (this is the hardest UI piece)
- Syntax highlighting (tree-sitter or syntect crate)
- Approve/reject controls per change
#### 5D: UI — Session Management
- Multiple sessions in a sidebar
- Session switching without losing state
- Session history and resume

#### 5E: File Preview & Terminal
- Embedded file viewer with syntax highlighting
- Terminal output rendering for command results
- File tree navigation

### SHORTCUT WORTH CONSIDERING
Claude Code has an open-source SDK and CLI. The CLI already handles all the
protocol complexity. Instead of reimplementing the protocol from scratch, the
Rust app could:
1. Spawn `claude` CLI in a PTY (pseudo-terminal)
2. Parse the terminal output (ANSI codes, structured output)
3. Render it in a custom GTK widget
This is less elegant but ships faster. The polished web UI can come later.

---

## PHASE 6: COWORK INTEGRATION (HARD MODE)

### What Cowork actually does
Cowork is NOT claude.ai in a tab. It is:
- A sandboxed execution environment using bubblewrap (bwrap) on Linux
- Controlled filesystem mounts — specific directories exposed, rest isolated
- Its own lifecycle daemon (cowork-vm-service.js on Linux)
- Persistent state between invocations
- MCP server integration inside the sandbox
- Tool execution (bash, file operations) inside the sandbox
- Communication back to the parent app via a defined protocol

### RECON PHASE (DO THIS FIRST — NO CODE)
1. Open Cowork in the Electron app and run a task that touches the filesystem
2. Find the sandbox process:   - `ps aux | grep bwrap`
   - `ps aux | grep cowork`
   - `pstree -p $(pgrep -f claude) | head -40`
3. Map the sandbox mounts:
   - `cat /proc/$(pgrep -f bwrap)/mountinfo`
   - Which host directories are exposed inside the sandbox?
   - Which are read-only vs read-write?
   - Where does persistent state live between sessions?
4. Find the lifecycle daemon:
   - aaddrick's project notes reference `cowork-vm-service.js`
   - Where does it run? How is it spawned? What ports does it listen on?
   - `ss -tlnp | grep cowork` or `netstat -tlnp | grep node`
5. Capture the communication protocol:
   - How does the Electron app talk to the sandbox?
   - HTTP? WebSocket? Unix socket? stdio?
   - What messages start a session, execute a command, return results?
6. Extract from Electron source:
   - `asar extract` — find the Cowork client code
   - Document the bwrap invocation arguments
   - Document the mount configuration
   - Document the session protocol
7. Document everything in `docs/cowork-sandbox-protocol.md`

### IMPLEMENTATION PHASES

#### 6A: Bubblewrap Sandbox
- Spawn bwrap from Rust with correct mount configuration
- Match the mount table discovered in recon
- Handle sandbox lifecycle: create, persist, destroy
- Test: can you run `ls` inside the sandbox and get output back?

#### 6B: Communication Protocol
- Implement the protocol between the native app and the sandbox
- Handle command execution requests and results
- Stream output back to the UI

#### 6C: MCP Inside Sandbox
- Launch MCP servers inside the sandbox
- Bridge MCP communication to/from the Anthropic API
- This is what makes Cowork actually useful vs just a container
#### 6D: Session Persistence
- State survives between Cowork invocations
- Files created in one session available in the next
- Match whatever persistence model the Electron app uses

#### 6E: UI Integration
- Cowork tab in the native app launches the sandbox
- Tool output rendered in the webview or custom widget
- File browser for sandbox contents

---

## RECOMMENDED ORDER

1. Phase 5 Recon — observe and document Claude Code integration (1-2 sessions)
2. Phase 6 Recon — observe and document Cowork sandbox (1-2 sessions)
3. Phase 5A-5B — Claude Code process + protocol (the core)
4. Phase 5 Shortcut — if protocol is too complex, fall back to PTY + CLI
5. Phase 6A-6B — Cowork sandbox + communication (the core)
6. UI polish last — diffs, file previews, session management

## THE HONEST ASSESSMENT

Phase 5 Shortcut (CLI in a PTY) can ship in a week.
Phase 5 Hard Mode (full protocol reimplementation) is 4-8 weeks.
Phase 6 (Cowork) is 4-12 weeks depending on how complex the sandbox protocol is.

The recon phases cost nothing but time and produce documentation that is
valuable to the Linux community regardless of whether the implementation
ships. Publishing `claude-code-protocol.md` and `cowork-sandbox-protocol.md`
on GitHub would be significant contributions on their own.

## DO NOT
- Do not start coding before recon is complete
- Do not guess at protocols — observe and document
- Do not try to build 5 and 6 simultaneously
- Do not skip the asar extraction — the Electron source is the answer key
