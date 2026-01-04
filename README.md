# Lazarus

**Offline-first Personal Knowledge Management for the Rest of Us**

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)

---

## What is Lazarus?

Lazarus is a lightweight, offline-capable personal knowledge management system built for environments where internet connectivity is unreliable, censorship is a concern, or resources are limited.

**The name "Lazarus" comes from the biblical figure who rose from the dead** — symbolizing knowledge that persists and remains accessible even when everything else fails.

### Who is this for?

- 📚 **Students in developing countries** with limited internet access
- 🏥 **Aid workers and NGOs** operating in remote areas
- 🔒 **Journalists and activists** in censorship-heavy regions
- 🌍 **Researchers** who need offline access to vast knowledge bases
- 💻 **Low-spec hardware users** (Raspberry Pi, old laptops, mesh networks)
- 🤓 **Minimalists** who are tired of Electron apps eating 500MB of RAM

### Why not Obsidian/Notion/Logseq?

| Feature | Lazarus | Others |
|---------|---------|--------|
| RAM usage (idle) | **3.6 MB** | 200-500 MB |
| Binary size | **6 MB** | 150-300 MB |
| 111GB Wikipedia | ✅ Instant | ❌ Impossible |
| Offline-first | ✅ Always | ⚠️ Limited |
| No account needed | ✅ | ❌ |
| Self-contained binary | ✅ | ❌ |
| Military-grade encryption | ✅ XChaCha20 | ⚠️ Varies |

---

## Features

### 📝 Note Management
- Block-based editor (Editor.js)
- Markdown support with live preview
- Tag-based organization
- Full-text fuzzy search (typo-tolerant)
- Auto-save with draft recovery
- Split-view for multitasking

### 📚 Wikipedia Integration (ZIM Reader)
- Load **entire Wikipedia** (111GB) with **3.6MB RAM**
- Memory-mapped I/O (mmap) — no loading, instant access
- Fuzzy search across millions of articles in < 1 second
- Multiple ZIM files simultaneously (English + Korean + Medical, etc.)
- Web UI for browsing and searching

### 🧠 Spaced Repetition System (SRS)
- SM-2 algorithm for optimal retention
- Auto-extract flashcards from notes
- Card types: Basic, Cloze, Definition
- Daily streak tracking with gamification
- Keyboard-driven review (1-4 for ratings)

### 🔐 Security
- **XChaCha20-Poly1305** encryption (same family as used by Signal, WireGuard)
- **Argon2id** key derivation (winner of Password Hashing Competition)
- Per-note encryption — encrypt only what matters
- PIN-based locking (4-8 digits)
- **No backdoors. No recovery. If you forget your PIN, your data is gone.**

### 📦 Package Sharing (.laz format)
- Export notes as portable packages
- Share via USB in offline environments
- SHA-256 integrity verification
- Drag-and-drop import
- Curriculum support for educational content

### 🌍 Internationalization
- English and Korean UI
- Easy to add more languages
- RTL support ready

### 💾 Data Safety
- Write-Ahead Log (WAL) for crash recovery
- Automatic rolling backups (keeps last 3)
- Database compaction
- Zero data loss on power failure

---

## Performance

Tested on: Intel i5, 8GB RAM, NVMe SSD

| Metric | Value |
|--------|-------|
| Binary size | 6.2 MB |
| Idle RAM (no ZIM) | ~10 MB |
| Idle RAM (111GB ZIM loaded) | **3.6 MB** |
| Cold start | < 100ms |
| Search (notes) | < 50ms |
| Search (111GB Wikipedia) | < 300ms |
| Fuzzy search tolerance | 1-2 typos |

### How is this possible?

1. **Memory-mapped files (mmap)**: The 111GB ZIM file isn't "loaded" — it's mapped into virtual address space. The OS handles paging.
2. **Zero-copy deserialization (rkyv)**: Notes are read directly from disk without parsing overhead.
3. **Streaming decompression**: Zstd decompresses on-the-fly, never buffering entire articles.
4. **No Electron**: Native Rust + lightweight HTML/CSS/JS frontend.

---

## Installation

### Pre-built Binary (Recommended)

```bash
# Download latest release
wget https://github.com/yourname/lazarus/releases/latest/download/lazarus-linux-x64

# Make executable
chmod +x lazarus-linux-x64

# Run
./lazarus-linux-x64
```

### Build from Source

```bash
# Clone
git clone https://github.com/yourname/lazarus.git
cd lazarus

# Build (release mode for best performance)
cargo build --release

# Binary located at
./target/release/lazarus
```

### Requirements

- **OS**: Linux, macOS, BSD (Windows via WSL2)
- **RAM**: 512 MB minimum (yes, really)
- **Storage**: 100 MB for app + your data + ZIM files

---

## Usage

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

## Deployment Scenarios

### Scenario 1: Mesh Network (Refugee Camp / Disaster Zone)

```
┌─────────────────────────────────────────────┐
│  Mesh Router (Raspberry Pi / OpenWRT)       │
│  ┌─────────────────────────────────────┐    │
│  │  Lazarus (6 MB binary)              │    │
│  │  + Wikipedia ZIM (on SD card)       │    │
│  │  RAM usage: ~10 MB                  │    │
│  └─────────────────────────────────────┘    │
│                    │                         │
│           Local WiFi Mesh                   │
│                    │                         │
│    ┌───────┐  ┌───────┐  ┌───────┐         │
│    │Phone 1│  │Phone 2│  │Laptop │         │
│    └───────┘  └───────┘  └───────┘         │
└─────────────────────────────────────────────┘
```

### Scenario 2: School in Rural Area

```
┌─────────────────────────────────────────────┐
│  Old Laptop (Server)                        │
│  ┌─────────────────────────────────────┐    │
│  │  Lazarus + Curriculum ZIMs          │    │
│  │  + .laz packages (textbooks)        │    │
│  └─────────────────────────────────────┘    │
│                    │                         │
│              Local WiFi                      │
│                    │                         │
│    ┌───────┐  ┌───────┐  ┌───────┐         │
│    │Student│  │Student│  │Teacher│         │
│    │Tablet │  │ Phone │  │Laptop │         │
│    └───────┘  └───────┘  └───────┘         │
│                                             │
│  📦 Share .laz packages via USB            │
│  🧠 Students use SRS for exam prep          │
│  🔒 Personal notes encrypted with PIN       │
└─────────────────────────────────────────────┘
```

---

## Security Model

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

## Technical Details

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      Web Browser                        │
│                    (Any modern browser)                 │
└────────────────────────┬────────────────────────────────┘
                         │ HTTP
┌────────────────────────┴────────────────────────────────┐
│                     Axum Web Server                     │
│                   (Async, lightweight)                  │
├─────────────────────────────────────────────────────────┤
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │
│  │  Notes  │ │   ZIM   │ │   SRS   │ │ Crypto  │       │
│  │ Handler │ │ Handler │ │ Handler │ │ Handler │       │
│  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘       │
├───────┴──────────┴──────────┴──────────┴───────────────┤
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │   WAL DB    │ │  ZIM Reader │ │  SRS Engine │       │
│  │ (rkyv+zstd) │ │   (mmap)    │ │   (SM-2)    │       │
│  └─────────────┘ └─────────────┘ └─────────────┘       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐       │
│  │   Tantivy   │ │   Backup    │ │  XChaCha20  │       │
│  │   (Search)  │ │  (Rolling)  │ │  (Encrypt)  │       │
│  └─────────────┘ └─────────────┘ └─────────────┘       │
└─────────────────────────────────────────────────────────┘
```

### Dependencies

Core:
- `axum` - Web framework
- `tokio` - Async runtime
- `tantivy` - Full-text search
- `rkyv` - Zero-copy serialization
- `zstd` - Compression
- `chacha20poly1305` - Encryption
- `argon2` - Key derivation

Frontend:
- Vanilla HTML/CSS/JS
- HTMX for interactivity
- Editor.js for block editing

### File Structure

```
data/
├── notes.lazarus      # Note database (WAL format)
├── index/             # Tantivy search index
├── srs.jsonl          # Flashcard data
├── srs_stats.json     # Learning statistics
├── security.json      # PIN configuration
├── backups/           # Rolling backups (last 3)
│   └── *.gz
└── zims/              # ZIM files directory
    └── *.zim
```

---

## Keyboard Shortcuts

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

---

## FAQ

**Q: Why Korean comments in the code?**

A: The original developer is Korean. Code works the same regardless of comment language. PRs with English translations welcome.

**Q: Can I use this commercially?**

A: Yes, MIT license. Do whatever you want.

**Q: Will you add feature X?**

A: Maybe. Open an issue. Or better, submit a PR.

**Q: Is this production-ready?**

A: It's "works on my machine" certified. Use at your own risk.

**Q: Why no mobile app?**

A: Access via browser on any device. PWA support planned.

---

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -am 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

### Code Style

- Rust: `cargo fmt` before committing
- Comments: Korean or English, both fine
- Tests: Would be nice to have some

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

- [Kiwix](https://kiwix.org) for ZIM format and offline Wikipedia
- [Tantivy](https://github.com/quickwit-oss/tantivy) for blazing-fast search
- [Editor.js](https://editorjs.io) for the block editor
- Claude (AI) for pair programming assistance

*"Knowledge should be free, accessible, and private."*

— The Lazarus Project
