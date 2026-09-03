<div align="center">
  <img src="assets/icon.png" width="96" alt="">
</div>

# Winfluencers Desktop

The native desktop shell for [Winfluencers](https://www.winvestour.com/winfluencers) — Winvestour's influencer program app. Built with [Tauri v2](https://tauri.app): a small, native window (~2 MB) that opens the live Winfluencers web app.

**Download the latest release:** see the [Releases](https://github.com/Winvestour/winfluencers/releases) page for Windows and Linux builds.

<div align="center">

<a href="https://github.com/Winvestour/winfluencers/releases/latest"><img src="https://img.shields.io/github/v/release/Winvestour/winfluencers?style=for-the-badge&color=E11D48&label=latest" alt="Latest release"></a>
<a href="https://github.com/Winvestour/winfluencers/releases"><img src="https://img.shields.io/github/downloads/Winvestour/winfluencers/total?style=for-the-badge&color=E11D48" alt="Downloads"></a>
<img src="https://img.shields.io/badge/platforms-Windows_%C2%B7_Linux-E11D48?style=for-the-badge" alt="Windows · Linux">
<a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-E11D48?style=for-the-badge" alt="MIT license"></a>

<a href="https://github.com/Winvestour/winfluencers/releases/latest"><img src="https://img.shields.io/badge/Download_for_Windows-.exe_%C2%B7_.msi-E11D48?style=for-the-badge&logo=windows&logoColor=white" alt="Download for Windows" height="36"></a>&nbsp;
<a href="https://github.com/Winvestour/winfluencers/releases/latest"><img src="https://img.shields.io/badge/Download_for_Linux-.deb_%C2%B7_.rpm_%C2%B7_.AppImage-E11D48?style=for-the-badge&logo=linux&logoColor=white" alt="Download for Linux" height="36"></a>&nbsp;
<a href="https://play.google.com/store/apps/details?id=com.winvestour.influencer"><img src="https://img.shields.io/badge/Google_Play-Android-414141?style=for-the-badge&logo=googleplay&logoColor=white" alt="Get it on Google Play" height="36"></a>

<a href="https://www.winvestour.com/winfluencers"><b>Website</b></a> · <a href="https://www.winvestour.com/register"><b>Create a free account</b></a> · <a href="https://github.com/Winvestour"><b>All Winvestour apps</b></a>

<img src="assets/hero.webp" alt="" width="760">

<sub>Earn commission on sales made with your own coupon code and match with brands.</sub>

</div>


## What this is

This shell contains **no application logic** — it's a thin native window around `https://www.winvestour.com`. All of Winfluencers' actual functionality (applying to campaigns, connecting social accounts, earnings tracking, payouts) lives on the web and is identical across platforms; this repo only ships the native wrapper (window chrome, tray behavior, auto-sizing) so Winfluencers installs and feels like a real desktop app.

- Branded title bar (frameless window, rose, drag-to-move)
- Single-instance (opening a second time focuses the existing window)
- Window size/position remembered between launches
- No telemetry, no bundled secrets, no local data storage beyond what the browser session already does

## Screenshots

<div align="center">
<img src="assets/phone-1.webp" alt="Winfluencers screenshot 1" width="190">&nbsp;
<img src="assets/phone-2.webp" alt="Winfluencers screenshot 2" width="190">&nbsp;
<img src="assets/phone-3.webp" alt="Winfluencers screenshot 3" width="190">&nbsp;
<img src="assets/phone-4.webp" alt="Winfluencers screenshot 4" width="190">
<br><br>
<a href="https://www.winvestour.com/winfluencers/screenshots">See all screenshots →</a>
</div>

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
