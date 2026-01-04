# Lazarus

**Offline-first Personal Knowledge Management**

![Version](https://img.shields.io/badge/version-0.1.0--beta-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.75+-orange)
![Tests](https://img.shields.io/badge/tests-25%20passing-brightgreen)

---

## What is Lazarus?

Lazarus is a lightweight, offline-capable personal knowledge management system. A standalone binary that can handle a full Wikipedia dump (111GB ZIM) without choking.

**The name "Lazarus" comes from the biblical figure who rose from the dead** — symbolizing knowledge that persists and remains accessible even when everything else fails.

## Features

### 📝 Note Management
- Block-based editor (Editor.js)
- Tag-based organization
- Full-text fuzzy search (typo-tolerant)
- Auto-save with draft recovery
- Split-view for multitasking

### 📚 Wikipedia Integration (ZIM Reader)
- Load **entire Wikipedia** (111GB) via memory-mapped I/O
- Fuzzy search across millions of articles
- Multiple ZIM files simultaneously
- Web UI for browsing and searching

### 🧠 Spaced Repetition System (SRS)
- SM-2 algorithm for optimal retention
- Auto-extract flashcards from notes
- Card types: Basic, Cloze, Definition
- Daily streak tracking with gamification
- Keyboard-driven review (1-4 for ratings)

### 🔐 Security
- **XChaCha20-Poly1305** encryption
- **Argon2id** key derivation
- Per-note encryption
- PIN-based locking (6-32 alphanumeric characters)

### 📦 Package Sharing (.laz format)
- Export notes as portable packages
- Share via USB in offline environments
- SHA-256 integrity verification
- Drag-and-drop import

### 🌍 Internationalization
- English and Korean UI
- Easy to add more languages

### 💾 Data Safety
- Write-Ahead Log (WAL) for crash recovery
- Automatic rolling backups (keeps last 3)
- Database compaction

---

## Installation

### Build from Source
```bash
git clone https://github.com/Ochichan/lazarus.git
cd lazarus
cargo build --release
./target/release/lazarus
```

### Requirements

- **OS**: Linux, macOS, BSD (Windows via WSL2)
- **RAM**: 512 MB minimum
- **Storage**: 100 MB for app + your data + ZIM files

---

## Usage
```bash
# Start server (default: http://127.0.0.1:8080)
./lazarus

# Custom port
./lazarus --port 3000

# With Wikipedia ZIM file
./lazarus --zim /path/to/wikipedia.zim

# Custom data directory
./lazarus --data /path/to/data
```

### All Options
```
OPTIONS:
    -p, --port <PORT>       Port number [default: 8080]
    -b, --bind <ADDR>       Bind address [default: 127.0.0.1]
    -d, --data <PATH>       Data directory [default: ./data]
        --zim <PATH>        ZIM file path (can be repeated)
        --zim-dir <PATH>    Directory containing ZIM files
    -h, --help              Print help
    -V, --version           Print version
```

---

## Security

### Encryption Details

| Component | Algorithm |
|-----------|-----------|
| Cipher | XChaCha20-Poly1305 (AEAD, 256-bit) |
| KDF | Argon2id (memory-hard) |
| Nonce | 24 bytes random |

### ⚠️ WARNING
```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│   THERE IS NO PASSWORD RECOVERY                          │
│                                                          │
│   If you forget your PIN, your encrypted notes are       │
│   PERMANENTLY LOST. This is by design.                   │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### Known Limitations

| Issue | Risk | Mitigation |
|-------|------|------------|
| PIN entropy | 6-char alphanumeric = ~31 bits | Use longer PINs. Argon2 slows brute-force |
| No rate limiting | DoS when exposed | Use reverse proxy for public deployment |
| Single-user design | Concurrent edit conflicts | Last write wins |

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

### SRS Review

| Key | Action |
|-----|--------|
| `Space` | Show answer |
| `1-4` | Rate card |

---

## Technical Details

### How is this possible?

1. **Memory-mapped files (mmap)**: The 111GB ZIM file isn't "loaded" — it's mapped into virtual address space
2. **Zero-copy deserialization (rkyv)**: Notes are read directly from disk without parsing overhead
3. **Streaming decompression**: Zstd decompresses on-the-fly
4. **No Electron**: Native Rust + lightweight HTML/CSS/JS frontend

### File Structure
```
data/
├── notes.lazarus      # Note database (WAL format)
├── index/             # Tantivy search index
├── srs.jsonl          # Flashcard data
├── srs_stats.json     # Learning statistics
├── security.json      # PIN configuration
├── backups/           # Rolling backups
└── zims/              # ZIM files directory
```

### Test Coverage
```
25 tests passing:
- crypto: 6 tests (encrypt/decrypt, key derivation)
- db: 4 tests (WAL, engine, notes)
- srs: 10 tests (cards, reviews, streaks)
- error: 2 tests
- zim: 1 test
```

---

## FAQ

**Q: Why Korean comments in the code?**

A: The developer is Korean. PRs with English translations welcome.

**Q: Is this production-ready?**

A: Beta. Use at your own risk. Backup your data.

**Q: Can I use this commercially?**

A: Yes, MIT license.

---

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Run tests (`cargo test`)
4. Commit your changes
5. Open a Pull Request

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

- [Kiwix](https://kiwix.org) for ZIM format
- [Tantivy](https://github.com/quickwit-oss/tantivy) for search
- [Editor.js](https://editorjs.io) for the block editor
- Claude (AI) for pair programming

---

*Built with AI assistance. Knowledge should be free, accessible, and private.*
