# Mouzi 🧹🐁

> **Your downloads, tamed.**

Mouzi is a silent, elegant file organizer that lives in your system tray and keeps your Downloads folder (and any other folder) automatically tidy. It runs quietly in the background, monitors selected folders, and moves, renames, or sorts files based on customizable rules.

[![Windows](https://img.shields.io/badge/Windows-10%2F11-blue?logo=windows)](https://mouzi.cc)
[![Tauri](https://img.shields.io/badge/Built%20with-Tauri-FFC131?logo=tauri)](https://tauri.app)
[![Rust](https://img.shields.io/badge/Backend-Rust-000000?logo=rust)](https://www.rust-lang.org)
[![React](https://img.shields.io/badge/Frontend-React-61DAFB?logo=react)](https://react.dev)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## 📸 Screenshots
<img width="640" height="257" alt="ezgif-39399e26ded336f6" src="https://github.com/user-attachments/assets/950b065d-ed29-4e55-9dd2-eaac188fba5d" />
<img width="490" height="372" alt="Zrzut ekranu 2026-05-09 184947" src="https://github.com/user-attachments/assets/c969b726-69fc-41da-9d2c-55051a107274" />


---

## ✨ Features

### 🔇 Silent by Default
- Runs 24/7 in the background with minimal resource usage (~5 MB RAM)
- Automatically organizes new files as they arrive
- Shows a subtle Windows toast notification with the count of organized files
- Silent autostart with Windows

### 📁 Smart Rules Engine
- **Images** (`.jpg`, `.png`, `.gif`, `.webp`...) → `Downloads/Images/`
- **Documents** (`.pdf`, `.docx`, `.xlsx`...) → `Downloads/Documents/`
- **Archives** (`.zip`, `.rar`, `.7z`...) → `Downloads/Archives/`
- **Installers** (`.exe`, `.msi`...) → `Downloads/Installers/`
- **Music** / **Video** → dedicated folders
- **Catch-all** rule for everything else

### 🛠️ Fully Customizable
- Create your own rules with extensions, regex patterns, and destination folders
- Use dynamic placeholders in paths: `{year}`, `{month}`, `{day}`, `{extension}`, `{filename}`
- Reorder rules by priority - first match wins

### 🚫 Ignore Rules (.mouziignore)
- Per-folder ignore patterns — like `.gitignore` for your files
- Set up via Settings UI or write a `.mouziignore` file manually
- Supports wildcards (`*.tmp`), exact names (`.DS_Store`), and folders (`node_modules/`)

### 📜 History & Undo
- Every action is logged locally in SQLite
- Undo any single move with one click
- Clear history anytime

### 🌍 Multi-language
Auto-detects your Windows system language. Supported:
- 🇬🇧 English
- 🇵🇱 Polish
- 🇮🇹 Italian
- 🇩🇪 German
- 🇫🇷 French
- 🇷🇺 Russian

*(Falls back to English if system language is not supported)*

### 🕶️ Dark Mode
- Follows system theme, or force Light / Dark mode from settings

### 🔒 Privacy First
- **100% offline** - zero cloud, zero file name uploads
- **No telemetry** by default
- **System files ignored** - `desktop.ini`, `Thumbs.db`, `.DS_Store`, and other OS hidden files are never touched
- **Portable version available** - run without installing, leaves no trace in the registry
- All data stored locally in your user profile folder

---

## 📥 Download

| Installer | Size | Best For |
|-----------|------|----------|
| [`Mouzi_0.1.0_x64-setup.exe`](https://mouzi.cc/download) | ~3.3 MB | Regular users (auto-installer) |
| [`Mouzi_0.1.0_x64_en-US.msi`](https://mouzi.cc/download) | ~4.7 MB | Enterprise / Active Directory |
| [`Mouzi_0.1.0_x64-portable.exe`](https://mouzi.cc/download) | ~14 MB | Power users (no install) |

**SHA-256 Checksums**

```
Mouzi_0.1.0_x64-setup.exe: e8c1bdecdb14eba23f53955a8270d1681e2026a49169ac994820cf8cc6e8bc28
Mouzi_0.1.0_x64_en-US.msi: a5c33c23bf93a5a10bd6534161a058dbd35fbc85b4fa2e5a11f0ddfd6bebe2d3
Mouzi_0.1.0_x64-portable.exe: 677649351d02076f2dc16b1487606f4173af699cb46535f8692eacd5dcc1c50b
```

> ⚠️ **Windows 10/11 only.** Requires the [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (pre-installed on most systems).

---

## 🚀 Quick Start

1. **Download & install** Mouzi using the installer above.
2. Mouzi starts automatically and places an icon in your system tray (📂).
3. **Left-click** the tray icon to open the popup - see recent actions, stats, and clean manually.
4. **Right-click** the tray icon for the menu: `Clean Now`, `Settings`, `Quit`.
5. Drop a file into your `Downloads` folder and watch it disappear into the right subfolder within 2 seconds.

---

## ⚙️ How Rules Work

Rules are evaluated top-to-bottom. The first rule that matches a file wins.

| Condition | Example Match |
|-----------|---------------|
| Extensions | `jpg`, `png`, `gif` |
| Regex pattern | `.*faktura.*` matches `faktura_2025.pdf` |

**Destination path placeholders:**
```
Downloads/Documents/{year}/{month}/
→ Downloads/Documents/2026/05/
```

---

## 📐 Architecture

```
+---------------------------------------------+
|  Frontend (React 19 + TypeScript + Tailwind) |
|  +- Popup window (300x420, frameless)        |
|  +- Settings window (900x650)                |
+---------------------------------------------+
|  Tauri 2.x Bridge                            |
+---------------------------------------------+
|  Backend (Rust)                              |
|  +- File Watcher (notify crate)              |
|  +- Rules Engine                             |
|  +- SQLite Database (rusqlite)               |
|  +- System Tray & Notifications              |
+---------------------------------------------+
```

---

## 🛠️ Development

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) 20+
- Windows SDK / MSVC (Visual Studio Build Tools)

### Setup

```bash
# Clone the repo
git clone https://github.com/yourusername/mouzi.git
cd mouzi

# Install frontend dependencies
npm install

# Run in development mode (hot-reload for both frontend & Rust)
npm run tauri dev
```

### Build from Source

```bash
# Production build (MSI + NSIS installer)
npm run tauri build
```

Output will be in `src-tauri/target/release/bundle/`.

---

## 📋 Roadmap

- [x] MVP with default rules
- [x] Multi-language support
- [x] Dark mode
- [x] History & undo
- [x] Start with Windows (registry Run key)
- [x] Custom folders with local rules
- [x] System files ignored (desktop.ini, Thumbs.db, etc.)
- [x] Check for updates button
- [x] `.mouziignore` - per-folder ignore patterns (like `.gitignore`)
- [x] Portable version (single .exe, no installer)
- [x] Browser temp files ignored (`.crdownload`, `.part`, `.tmp`)
- [x] Grace period option - delay moving files by X minutes (so browser download links stay valid)
- [x] File lock check - skip files currently in use by another process
- [x] Single-instance guard - prevent duplicate processes & tray icons
- [x] First-run popup visibility - show window on initial launch
- [x] Clickable toast - open destination folder from popup notification
- [x] Skip 0KB placeholder files during download
- [ ] Scheduled clean mode - run once/2/3/4 times per day instead of real-time
- [ ] Batch group selected files - select multiple files, one click to group into a folder
- [ ] Export/import rules as JSON (backup + sharing)
- [ ] Suggest mode (modal confirmation per file)
- [ ] Local AI tagging (ONNX runtime for content classification)
- [ ] Rule learning from user manual moves
- [ ] macOS & Linux ports

---

## ☕ Support

If Mouzi saves you time and keeps your Downloads folder sane, consider supporting its development:

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/hsr)

Or visit the project homepage: **[mouzi.cc](https://mouzi.cc)**

---
## See Also

### [Ordir](https://github.com/landnthrn/ordir)

Order folders any way you want inside Windows File Explorer, and add custom thumbnails.

---
## 📄 License

Mouzi is released under the [MIT License](LICENSE).

---

## 🙏 Acknowledgements

Built with [Tauri](https://tauri.app), [React](https://react.dev), [Tailwind CSS](https://tailwindcss.com), and [Rust](https://www.rust-lang.org).

---

<p align="center">
  <sub>Made with ❤️ for people who download too much stuff.</sub>
</p>
