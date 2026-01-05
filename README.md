# Lazarus 🦴

**Offline-first Personal Knowledge Management for the disconnected world.**

A lightweight PKM system designed for students in developing countries, refugee camps, disaster zones, and censored regions — anywhere internet access is unreliable or unavailable.

## Why Lazarus?

> "Knowledge should be free, accessible, and private."

- 📚 **111GB Wikipedia** loads with just **3.6MB RAM**
- 🔒 **Military-grade encryption** (XChaCha20-Poly1305 + Argon2id)
- 📴 **100% offline** — no internet required
- 💾 **6.2MB binary** — fits on any USB drive
- 🔄 **USB knowledge sharing** via .laz packages

## Features

### 📝 Note-taking
- Block-based editor (Notion-style)
- Full-text search with fuzzy matching
- Auto-save with WAL (Write-Ahead Log)
- Tags and organization

### 📖 Wikipedia Reader
- Read ZIM files (offline Wikipedia)
- Memory-mapped I/O for huge files
- Split-view: Wiki + Notes side by side
- Create flashcards from any text

### 🧠 Spaced Repetition (FSRS)
- Modern FSRS algorithm (better than Anki's SM-2)
- Card types: Basic, Cloze, Definition
- Auto-extract cards from notes
- Streak tracking & gamification

### 🔐 Security
- PIN lock with Argon2id key derivation
- XChaCha20-Poly1305 encryption
- Encrypted backups
- API authentication middleware

### 📦 Knowledge Sharing
- Export/import .laz packages
- Share curricula via USB
- Checksum verification
- Works in air-gapped environments

## Quick Start
```bash
# Download (Linux)
curl -LO https://github.com/username/lazarus/releases/latest/download/lazarus
chmod +x lazarus

# Run
./lazarus

# Open browser
# http://localhost:8080
```

## Usage

### Adding Wikipedia

1. Download a ZIM file from [Kiwix](https://wiki.kiwix.org/wiki/Content_in_all_languages)
2. Place it in `~/.lazarus/zims/` or add via UI
3. Browse offline Wikipedia!

### Creating Flashcards

1. Select text in Wikipedia or Notes
2. Click popup → choose card type
3. Review in SRS section

### Sharing Knowledge
```bash
# Export notes + cards as .laz package
# UI: Settings → Export Package

# Import on another device
# UI: Settings → Import Package
```

## Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| Web Framework | Axum + Tokio |
| Storage | Custom WAL + rkyv + zstd |
| Search | Tantivy |
| Encryption | XChaCha20-Poly1305 + Argon2id |
| Templates | Askama |
| Editor | Editor.js |

## Performance

| Metric | Value |
|--------|-------|
| Binary size | 6.2 MB |
| Idle RAM | ~10 MB |
| 111GB ZIM loaded | 3.6 MB RAM |
| Cold start | < 100ms |

## Building from Source
```bash
# Requirements: Rust 1.70+
git clone https://github.com/username/lazarus
cd lazarus
cargo build --release

# Binary at target/release/lazarus
```

## Configuration

Data stored in `~/.lazarus/`:
```
~/.lazarus/
├── notes.lazarus    # Note database
├── index/           # Search index
├── srs.jsonl        # Flashcards
├── backups/         # Auto backups
├── security.json    # PIN config
└── zims/            # ZIM files
```

## Roadmap

- [x] Core PKM (notes, search, tags)
- [x] ZIM Wikipedia reader
- [x] FSRS spaced repetition
- [x] Encryption & PIN lock
- [x] .laz package format
- [x] i18n (English, Korean)
- [ ] More languages (Arabic, French, Spanish)
- [ ] PWA support
- [ ] USB community board
- [ ] Graph view

## Contributing

PRs welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License. See [LICENSE](LICENSE).

## Acknowledgments

- [Kiwix](https://kiwix.org) for ZIM format
- [FSRS](https://github.com/open-spaced-repetition/fsrs4anki) algorithm
- Everyone working to make knowledge accessible

---

*Built for the 2.9 billion people without reliable internet access.*

🦴 **Lazarus** — Knowledge rises from the ashes.
