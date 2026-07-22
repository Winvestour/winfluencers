# Winfluencers Desktop

The native desktop shell for [Winfluencers](https://www.winvestour.com/winfluencers) — Winvestour's influencer program app. Built with [Tauri v2](https://tauri.app): a small, native window (~2 MB) that opens the live Winfluencers web app.

**Download the latest release:** see the [Releases](https://github.com/Winvestour/winfluencers/releases) page for Windows and Linux builds.

## What this is

This shell contains **no application logic** — it's a thin native window around `https://www.winvestour.com`. All of Winfluencers' actual functionality (applying to campaigns, connecting social accounts, earnings tracking, payouts) lives on the web and is identical across platforms; this repo only ships the native wrapper (window chrome, tray behavior, auto-sizing) so Winfluencers installs and feels like a real desktop app.

- Branded title bar (frameless window, rose, drag-to-move)
- Single-instance (opening a second time focuses the existing window)
- Window size/position remembered between launches
- No telemetry, no bundled secrets, no local data storage beyond what the browser session already does

## Building locally

Prerequisites: [Rust](https://rustup.rs) (stable, MSVC toolchain on Windows), [Node.js](https://nodejs.org) 20+, and platform build tools ([Tauri prerequisites](https://tauri.app/start/prerequisites/)).

```bash
npm install
npm run tauri build
```

Output installers land in `src-tauri/target/release/bundle/`.

## Supported platforms

| Platform | Format | Status |
|---|---|---|
| Windows 10/11 | `.exe` (NSIS), `.msi` | ✅ |
| Linux | `.deb`, `.rpm`, `.AppImage` | ✅ (built via CI) |
| macOS | — | Not planned |

## License

MIT — see [LICENSE](LICENSE).
