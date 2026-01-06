# Lazarus 🦴

**Offline-first Personal Knowledge Management for the Rest of Us**

[![CI](https://github.com/Ochichan/lazarus/actions/workflows/ci.yml/badge.svg)](https://github.com/Ochichan/lazarus/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/version-0.3.5-blue)](https://github.com/Ochichan/lazarus/releases)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)](https://www.rust-lang.org/)

---

## TL;DR

```bash
# Download and run (that's it)
./lazarus --zim wikipedia.zim

# Open in browser
open http://127.0.0.1:8080
```

**7MB binary** · **10MB RAM** · **111GB Wikipedia instant load** · **Fully offline**

---

## What is Lazarus?

Lazarus is a lightweight, offline-capable personal knowledge management system built for environments where internet connectivity is unreliable, censorship is a concern, or resources are limited.

**The name "Lazarus" comes from the biblical figure who rose from the dead** — symbolizing knowledge that persists and remains accessible even when everything else fails.

### Who is this for?

| User | Use Case |
|------|----------|
| 📚 Students in developing countries | Learning with limited internet |
| 🏥 Aid workers and NGOs | Information access in remote areas |
| 🔒 Journalists and activists | Censorship bypass, privacy |
| 🌍 Researchers | Offline knowledge bases |
| 💻 Low-spec hardware users | Raspberry Pi, old laptops, mesh networks |
| 🤓 Minimalists | Tired of Electron apps eating 500MB RAM |

### Why not Obsidian/Notion/Logseq?

| Feature | Lazarus | Others |
|---------|---------|--------|
| RAM usage (idle) | **~10 MB** | 200-500 MB |
| Binary size | **7 MB** | 150-300 MB |
| 111GB Wikipedia | ✅ 5MB RAM | ❌ Impossible |
| Offline-first | ✅ Always | ⚠️ Limited |
| No account needed | ✅ | ❌ |
| Self-contained binary | ✅ | ❌ |
| Military-grade encryption | ✅ XChaCha20 | ⚠️ Varies |

---

## ✨ Features

### 📝 Note Management
- **Editor.js** block editor (Notion-style)
- Markdown support with live preview
- Tag-based organization
- **Fuzzy search** (typo-tolerant)
- Auto-save with draft recovery
- **Note types**: 📝 Note · 📔 Journal · 📖 Review · 💡 Idea

### 📚 Wikipedia Integration (ZIM Reader)
- Load **entire Wikipedia** (111GB) with **3.6MB RAM**
- Memory-mapped I/O — no loading, instant access
- Fuzzy search across millions of articles < 1 second
- Multiple ZIM files simultaneously (English + Korean + Medical, etc.)

### 🧠 Spaced Repetition System (FSRS)
- **FSRS algorithm** — 20-30% more efficient than SM-2
- Auto-extract flashcards from notes
- Card types: **Basic**, **Cloze**, **Definition**
- Streak tracking + gamification (🔥💎👑)
- Keyboard shortcuts (1-4 ratings)

### 🔀 Split View
- **Wiki + Note editor** side by side
- Select wiki text → add to notes
- Toggle with `Ctrl+E` or 📝+ button

### ⚡ Wiki → Card
- Select text in wiki → popup appears
- **One-click** Basic/Cloze/Definition card creation
- Source URL auto-saved

### 🔐 Security
- **XChaCha20-Poly1305** encryption (same family as Signal, WireGuard)
- **Argon2id** key derivation (Password Hashing Competition winner)
- PIN-based locking (6-32 alphanumeric characters)
- **API authentication middleware** — all APIs protected when locked
- **Concurrent edit locks** — prevent duplicate editing
- **Encrypted backups** — backups encrypted when PIN is set
- **⚠️ No backdoors. No recovery. Forget your PIN = permanent data loss.**

### 📦 Package Sharing (.laz format)
- Export notes as portable packages
- Share via USB in offline environments
- SHA-256 integrity verification
- Drag-and-drop import

### 🌍 Globalization (17 Languages)
- **Base**: 🇺🇸 English, 🇰🇷 Korean
- **Group 1**: 🇸🇦 Arabic (RTL), 🇮🇷 Persian (RTL), 🇰🇪 Swahili, 🇮🇩 Indonesian, 🇮🇳 Hindi, 🇧🇩 Bengali
- **Group 2**: 🇪🇸 Spanish, 🇧🇷 Portuguese, 🇫🇷 French, 🇷🇺 Russian, 🇹🇷 Turkish
- **Group 3**: 🇯🇵 Japanese, 🇨🇳 Chinese Simplified, 🇹🇼 Chinese Traditional, 🇭🇰 Cantonese
- **Full RTL support** for Arabic and Persian (right-to-left layout)
- **Settings page** with language dropdown selector
- Modular i18n architecture (170+ translation keys)

### 💾 Data Safety
- **Write-Ahead Log (WAL)** for crash recovery
- **Rolling backups** (keeps last 3)
- Database compaction
- Zero data loss on power failure

---

## 📊 Performance

| Metric | Value |
|--------|-------|
| Binary size | **7 MB** |
| Idle RAM | **~4 MB** |
| Cold start | **< 100ms** |
| Search (notes) | **< 50ms** |
| Search (111GB Wikipedia) | **< 300ms** |
| Fuzzy tolerance | 1-2 typos |

### How is this possible?

1. **Memory-mapped files (mmap)**: The 111GB ZIM file isn't "loaded" — it's mapped into virtual address space. The OS handles paging.
2. **Zero-copy deserialization (rkyv)**: Notes are read directly from disk without parsing overhead.
3. **Streaming decompression**: Zstd decompresses on-the-fly, never buffering entire articles.
4. **No Electron**: Native Rust + lightweight HTML/CSS/JS frontend.

---

## 🚀 Installation

### Pre-built Binary (Recommended)

```bash
# Download latest release
wget https://github.com/Ochichan/lazarus/releases/latest/download/lazarus-linux-x64

# Make executable
chmod +x lazarus-linux-x64

# Run
./lazarus-linux-x64
```

### Build from Source

```bash
# Clone
git clone https://github.com/Ochichan/lazarus.git
cd lazarus

# Build (release mode for best performance)
cargo build --release

# Binary located at
./target/release/lazarus
```

### With Nix (Flakes)

```bash
nix develop  # Enter dev environment
cargo run    # Run
```

### Requirements

- **OS**: Linux, macOS, BSD (Windows via WSL2)
- **RAM**: 512 MB minimum (yes, really)
- **Storage**: 100 MB for app + your data + ZIM files

---

## 📖 Usage

### Basic

```bash
# Start server (default: http://127.0.0.1:8080)
./lazarus

# Custom port
./lazarus --port 3000

# Allow external connections (for mesh networks)
./lazarus --bind 0.0.0.0
```

### With Wikipedia

Download ZIM files from [Kiwix](https://wiki.kiwix.org/wiki/Content):

```bash
# Single ZIM file
./lazarus --zim /path/to/wikipedia_en_all.zim

# Multiple ZIM files
./lazarus --zim wiki_en.zim --zim wiki_ko.zim --zim medical.zim

# Auto-load from directory
./lazarus --zim-dir /path/to/zims/
```

### Data Location

```bash
# Custom data directory (useful for USB drives)
./lazarus --data /media/usb/lazarus-data
```

### All Options

```
USAGE:
    lazarus [OPTIONS]

OPTIONS:
    -p, --port <PORT>       Port number [default: 8080]
    -b, --bind <ADDR>       Bind address [default: 127.0.0.1]
    -d, --data <PATH>       Data directory [default: ./data]
        --zim <PATH>        ZIM file path (can be repeated)
        --zim-dir <PATH>    Directory containing ZIM files
        --log-level <LVL>   Log level: trace/debug/info/warn/error
    -h, --help              Print help
    -V, --version           Print version
```
---

## 🔐 Security Model

### Encryption Details

| Component | Algorithm | Notes |
|-----------|-----------|-------|
| Cipher | XChaCha20-Poly1305 | AEAD, 256-bit key |
| KDF | Argon2id | Memory-hard, side-channel resistant |
| Nonce | 24 bytes random | Safe for random generation |
| Auth | Poly1305 MAC | Integrity + authenticity |

### What's Protected

- ✅ Note content (when encryption enabled)
- ✅ PIN verification data
- ✅ Backup files (when PIN is set)
- ❌ Note titles (visible for navigation)
- ❌ Tags (visible for organization)
- ❌ Timestamps

### Threat Model

**Protected against:**
- Casual snooping (phone theft, shared computer)
- Mass surveillance (encrypted at rest)
- Data extraction (PIN required)

**NOT protected against:**
- Targeted attacks with physical access + unlimited time
- Rubber-hose cryptanalysis (torture)
- Sophisticated nation-state actors with $10M+ budgets

### ⚠️ WARNING

```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│   THERE IS NO PASSWORD RECOVERY                          │
│                                                          │
│   If you forget your PIN, your encrypted notes are       │
│   PERMANENTLY LOST. This is by design.                   │
│                                                          │
│   No backdoors. No master keys. No recovery emails.      │
│   This is a feature, not a bug.                          │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Web Browser                        │
│                   (Any modern browser)                  │
└────────────────────────┬────────────────────────────────┘
                         │ HTTP
┌────────────────────────┴────────────────────────────────┐
│                     Axum Web Server                     │
│                  (Async, lightweight)                   │
├─────────────────────────────────────────────────────────┤
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │
│  │  Notes  │ │   ZIM   │ │   SRS   │ │ Crypto  │       │
│  │ Handler │ │ Handler │ │ Handler │ │ Handler │       │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘       │
├───────┴──────────┴──────────┴──────────┴───────────────┤
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │   WAL DB    │ │  ZIM Reader │ │  SRS Engine │       │
│  │ (rkyv+zstd) │ │   (mmap)    │ │   (FSRS)    │       │
│  └─────────────┘ └─────────────┘ └─────────────┘       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │   Tantivy   │ │   Backup    │ │  XChaCha20  │       │
│  │   (Search)  │ │  (Rolling)  │ │  (Encrypt)  │       │
│  └─────────────┘ └─────────────┘ └─────────────┘       │
└─────────────────────────────────────────────────────────┘
```

### Tech Stack

| Layer | Technology |
|-------|------------|
| Language | Rust |
| Web Framework | Axum |
| Templates | Askama |
| Frontend | HTMX, Vanilla JS, Editor.js |
| Storage | Custom WAL (rkyv + zstd) |
| Search | Tantivy |
| Encryption | XChaCha20-Poly1305, Argon2id |
| SRS | FSRS algorithm |
| Wikipedia | OpenZIM (mmap) |
| Offline | PWA + Service Worker |

### File Structure

```
data/
├── notes.lazarus      # Note database (WAL format)
├── index/             # Tantivy search index
├── srs.jsonl          # Flashcard data
├── srs_stats.json     # Learning statistics
├── security.json      # PIN configuration
├── backups/           # Rolling backups (last 3)
│   └── *.gz.enc       # Encrypted if PIN set
└── zims/              # ZIM files directory
    └── *.zim
```

---

## ⌨️ Keyboard Shortcuts

### Global

| Key | Action |
|-----|--------|
| `n` | New note |
| `/` | Search |
| `?` | Help |

### Editor

| Key | Action |
|-----|--------|
| `Ctrl+S` | Save |
| `Ctrl+E` | Toggle split view |
| `Ctrl+B` | Bold |
| `Ctrl+I` | Italic |
| `Esc` | Exit mode |

### SRS Review

| Key | Action |
|-----|--------|
| `Space` / `Enter` | Show answer |
| `1` | Again (forgot) |
| `2` | Hard |
| `3` | Good |
| `4` | Easy |

### Wiki

| Key | Action |
|-----|--------|
| `Alt+←` | Back |
| `Alt+→` | Forward |
| `Esc` | Close |

---

## 🗺️ Roadmap

### v0.2 - Stabilization ✅
- [x] GitHub Actions CI/CD
- [x] Pre-built release binaries
- [x] API authentication middleware
- [x] Encrypted backups
- [x] Concurrent edit locks

### v0.3 - Feature Expansion ✅
- [x] 17 languages (EN, KO, AR, FA, SW, ID, HI, BN, ES, PT, FR, RU, TR, JA, ZH-CN, ZH-TW, YUE)
- [x] Full RTL support (Arabic)
- [x] Settings page with language selector
- [x] FSRS personalization (parameter optimization)
- [x] Note linking (`[[note title]]`)
- [x] Graph view (note connections)

### v0.4 - USB Community
- [ ] USB auto-detection
- [ ] Offline bulletin board
- [ ] Anonymous Q&A
- [ ] Package hub

### v0.5 - Advanced
- [ ] Plugin system
- [ ] ARM binary (Raspberry Pi)
- [ ] Android APK
- [ ] Real-time collaboration (WebRTC P2P)

---

## ❓ FAQ

**Q: Why Korean comments in the code?**

A: The developer is Korean. Code works the same regardless of comment language. PRs with English translations welcome.

**Q: Can I use this commercially?**

A: Yes, MIT license. Do whatever you want.

**Q: Is this production-ready?**

A: It's "works on my machine" certified. Battle-tested with Red Team security reviews. Use at your own risk.

**Q: Why no mobile app?**

A: Access via browser on any device. Full PWA support — install to home screen.

**Q: How is a 5-month programmer building this?**

A: Lots of coffee, Claude AI pair programming, and refusing to sleep. Also, Rust's compiler is a great teacher.

---

## 🤝 Contributing

1. Fork it
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -am 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

### Code Style

- Rust: `cargo fmt` before committing
- Lint: `cargo clippy` should pass
- Tests: `cargo test` should pass
- Comments: Korean or English, both fine

---

## 📜 License

MIT License. See [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- [Kiwix](https://kiwix.org) for ZIM format and offline Wikipedia
- [Tantivy](https://github.com/quickwit-oss/tantivy) for blazing-fast search
- [Editor.js](https://editorjs.io) for the block editor
- [open-spaced-repetition](https://github.com/open-spaced-repetition/fsrs4anki) for FSRS algorithm
- Claude AI for pair programming assistance

---

<div align="center">

*"Knowledge should be free, accessible, and private."*

**— The Lazarus Project 🦴**

</div>
