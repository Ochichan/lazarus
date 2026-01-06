# Lazarus 🦴

**Offline-first Personal Knowledge Management for the Rest of Us**

[![CI](https://github.com/Ochichan/lazarus/actions/workflows/ci.yml/badge.svg)](https://github.com/Ochichan/lazarus/actions/workflows/ci.yml)
[![Benchmark](https://github.com/Ochichan/lazarus/actions/workflows/benchmark.yml/badge.svg)](https://github.com/Ochichan/lazarus/actions/workflows/benchmark.yml)
[![Version](https://img.shields.io/badge/version-0.3.5-blue)](https://github.com/Ochichan/lazarus/releases)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange)](https://www.rust-lang.org/)
![Binary](https://img.shields.io/badge/Binary-7.4MB-blue)
![RAM](https://img.shields.io/badge/RAM-5MB-brightgreen)
![Efficiency](https://img.shields.io/badge/Efficiency-Insane-red)

---

## TL;DR

```bash
# Download and run (that's it)
./lazarus --zim wikipedia.zim

# Open in browser
open http://127.0.0.1:8080
```

**7.4MB binary** · **5MB RAM** · **4ms cold start** · **115GB Wikipedia** · **17 languages** · **Fully offline**

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

| Feature | Lazarus | Electron Apps |
|---------|---------|---------------|
| Cold start | **4 ms** | 2-5 seconds |
| RAM usage (idle) | **5 MB** | 200-500 MB |
| Binary size | **7.4 MB** | 150-300 MB |
| 115GB Wikipedia | ✅ **+1MB RAM** | ❌ **Impossible** |
| Multiple ZIM files | ✅ Simultaneous | ❌ N/A |
| Offline-first | ✅ Always | ⚠️ Limited |
| No account needed | ✅ | ❌ |
| Military-grade encryption | ✅ XChaCha20 | ⚠️ Varies |

> **Why is 115GB impossible for Electron?**
> V8 JavaScript engine has a ~4GB heap limit. `fs.readFile()` loads entire files into memory.
> Even with streaming, ZIM format requires random access which is impractical.
> **Electron literally cannot do this. Period.**

---

## 📸 Proof: Real Measurements

> *"115GB with 5MB RAM? 4ms startup? You're lying."*

We're not. Here's the actual measurements:

### Cold Start

```bash
$ time ./lazarus --help
________________________________________________________
Executed in    4.28 millis    fish           external
   usr time    4.17 millis    0.00 micros    4.17 millis
   sys time    4.73 millis  564.00 micros    4.16 millis
```

**4.28 milliseconds.** Electron apps show loading spinners for 3 seconds.

### Memory Usage (pmap -x)

```bash
# Idle - No ZIM loaded
$ pmap -x $(pgrep lazarus) | tail -n 1
total kB         2655596   11656    4064
#                ↑ Virtual  ↑ RSS   ↑ Dirty (4MB)

# After loading 115GB ZIM (Wikipedia + TED Talks)
$ pmap -x $(pgrep lazarus) | tail -n 1
total kB         119768512   12852    5132
#                ↑ 114GB      ↑ RSS   ↑ Dirty (5MB)
```

### The Numbers

| State | Virtual | RSS | Dirty (Real Usage) |
|-------|---------|-----|-------------------|
| Idle (no ZIM) | 2.6 GB | 11.6 MB | **4 MB** |
| 115GB ZIM loaded | 114 GB | 12.8 MB | **5 MB** |
| **Difference** | +111 GB | +1.2 MB | **+1 MB** |

### 🤯 Let That Sink In

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   Loading 115GB increased memory usage by 1MB.          │
│                                                         │
│   ONE. MEGABYTE.                                        │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### During Heavy Usage

| State | Dirty | RSS | Search Speed |
|-------|-------|-----|--------------|
| Heavy search + browse | **35 MB** | 380 MB | **< 300ms** |

> **Note**: RSS includes OS page cache — automatically reclaimed when needed.

### Verify It Yourself

```bash
# Cold start
time ./lazarus --help

# Memory usage
./lazarus --zim your-115gb-file.zim &
pmap -x $(pgrep lazarus) | tail -n 1
```

---

## 🤔 How Is This Possible?

| Technique | What It Does |
|-----------|--------------|
| **mmap** | Maps file to virtual address space. OS pages in 4KB chunks *only when accessed*. 115GB stays on disk. |
| **Zero-copy (rkyv)** | Deserialize without copying. Read directly from memory-mapped region. |
| **Streaming Zstd** | Decompress on-the-fly. Never buffer entire articles. |
| **No Electron** | Native binary. No JavaScript VM. No Chromium eating 300MB. |

**TL;DR**: We don't "load" the file. We map it. The OS handles the rest.

---

## ✨ Features

### 📝 Note Management
- **Editor.js** block editor (Notion-style)
- Markdown support with live preview
- Tag-based organization
- **Fuzzy search** (typo-tolerant)
- Auto-save with draft recovery
- **Note types**: 📝 Note · 📔 Journal · 📖 Review · 💡 Idea
- **Note linking**: `[[note title]]` wiki-style links
- **Graph view**: Visualize connections with D3.js

### 📚 Wikipedia Integration (ZIM Reader)
- Load **entire Wikipedia** (115GB) with **+1MB RAM**
- **Multiple ZIM files simultaneously** — English + Korean + Medical + TED Talks, all at once!
- Memory-mapped I/O — no loading, instant access
- Fuzzy search across 6M+ articles in < 300ms
- Cross-ZIM unified search

### 🧠 Spaced Repetition System (FSRS)
- **FSRS algorithm** — 20-30% more efficient than SM-2
- Auto-extract flashcards from notes
- Card types: **Basic**, **Cloze**, **Definition**
- Streak tracking + gamification (🔥💎👑)
- Keyboard shortcuts (1-4 ratings)

### 🔀 Split View
- **Wiki + Note editor** side by side
- Select wiki text → add to notes
- Toggle with `Ctrl+E`

### ⚡ Wiki → Card
- Select text in wiki → popup appears
- **One-click** flashcard creation
- Source URL auto-saved

### 🔐 Security
- **XChaCha20-Poly1305** encryption (same as Signal, WireGuard)
- **Argon2id** key derivation (Password Hashing Competition winner)
- PIN-based locking (6-32 characters)
- **API authentication middleware**
- **Concurrent edit locks**
- **Encrypted backups**
- **⚠️ No backdoors. No recovery. Forget PIN = data gone forever.**

### 📦 Package Sharing (.laz format)
- Export notes as portable packages
- Share via USB in offline environments
- SHA-256 integrity verification
- Drag-and-drop import

### 🌍 Internationalization (17 Languages)

| Tier | Languages |
|------|-----------|
| **Base** | 🇺🇸 English, 🇰🇷 Korean |
| **    ** | 🇸🇦 Arabic (RTL), 🇮🇷 Persian (RTL), 🇰🇪 Swahili, 🇮🇩 Indonesian, 🇮🇳 Hindi, 🇧🇩 Bengali |
| **    ** | 🇪🇸 Spanish, 🇧🇷 Portuguese, 🇫🇷 French, 🇷🇺 Russian, 🇹🇷 Turkish |
| **    ** | 🇯🇵 Japanese, 🇨🇳 简体中文, 🇹🇼 繁體中文, 🇭🇰 廣東話 |

- **Full RTL support** for Arabic and Persian
- **170+ translation keys** per language

### 💾 Data Safety
- **Write-Ahead Log (WAL)** for crash recovery
- **Rolling backups** (keeps last 3)
- Database compaction
- Zero data loss on power failure

---

## 📊 Performance

| Metric | Value | Notes |
|--------|-------|-------|
| Binary size | **7.4 MB** | Single executable |
| Cold start | **< 5 ms** | Measured: 4.28ms |
| Idle RAM | **4 MB** | Dirty memory |
| +115GB ZIM | **+1 MB** | Yes, one megabyte |
| Heavy usage | **35 MB** | Search + browse |
| Note search | **< 50ms** | Tantivy |
| Wiki search | **< 300ms** | 6M+ articles |

### Comparison

| App | Cold Start | Idle RAM | Load 115GB |
|-----|------------|----------|------------|
| **Lazarus** | **4 ms** | **4 MB** | ✅ **+1 MB** |
| Obsidian | ~3 sec | ~300 MB | ❌ Crash |
| Notion | ~4 sec | ~400 MB | ❌ Impossible |
| VS Code | ~3 sec | ~500 MB | ❌ Impossible |

---

## 🚀 Installation

### Pre-built Binary

```bash
wget https://github.com/Ochichan/lazarus/releases/latest/download/lazarus-linux-x64
chmod +x lazarus-linux-x64
./lazarus-linux-x64
```

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
- **Storage**: 100 MB + your ZIM files

---

## 📖 Usage

```bash
# Basic
./lazarus

# Custom port
./lazarus --port 3000

# With Wikipedia
./lazarus --zim /path/to/wikipedia.zim

# Multiple ZIM files simultaneously!
./lazarus --zim wiki_en.zim --zim wiki_ko.zim --zim ted_talks.zim --zim medical.zim

# Load all ZIMs from a directory
./lazarus --zim-dir /path/to/zims/

# USB drive mode
./lazarus --data /media/usb/lazarus-data
```

Download ZIM files from [Kiwix](https://wiki.kiwix.org/wiki/Content).

### Example: Multi-ZIM Setup

```bash
# English Wikipedia (111GB) + Korean Wikipedia (2GB) + TED Talks (4GB) + Medical (8GB)
./lazarus \
  --zim wikipedia_en_all_maxi.zim \
  --zim wikipedia_ko_all.zim \
  --zim ted_mul_life.zim \
  --zim wikimed_en.zim

# Total: ~125GB of content
# RAM usage: Still ~5MB 🤯
```

---

## 🔐 Security Model

| Component | Algorithm |
|-----------|-----------|
| Cipher | XChaCha20-Poly1305 (256-bit) |
| KDF | Argon2id |
| Nonce | 24 bytes random |
| Auth | Poly1305 MAC |

### ⚠️ WARNING

```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│   THERE IS NO PASSWORD RECOVERY                          │
│                                                          │
│   Forget your PIN = PERMANENT DATA LOSS                  │
│   No backdoors. No master keys. By design.               │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## 🏗️ Architecture

```
Browser ←HTTP→ Axum Server
                    │
    ┌───────┬───────┼───────┬───────┐
    │       │       │       │       │
  Notes    ZIM    SRS   Crypto  Search
    │       │       │       │       │
  WAL+    mmap   FSRS  XChaCha  Tantivy
  rkyv    (multi)      Argon2id
```

### Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 🦀 |
| Web | Axum + Askama |
| Frontend | HTMX + Vanilla JS + Editor.js |
| Storage | Custom WAL (rkyv + zstd) |
| Search | Tantivy |
| Crypto | XChaCha20-Poly1305 + Argon2id |
| SRS | FSRS algorithm |
| Wikipedia | OpenZIM (mmap, multi-file) |
| i18n | 17 languages, RTL support |

---

## 🗺️ Roadmap

### v0.3 - Current ✅
- [x] 17 languages with RTL
- [x] Note linking + Graph view
- [x] Memory benchmark CI
- [x] Multi-ZIM simultaneous loading

### v0.4 - USB Community
- [ ] USB auto-detection
- [ ] Offline bulletin board
- [ ] Package hub

### v0.5 - Advanced
- [ ] Plugin system
- [ ] ARM binary (Raspberry Pi)
- [ ] Android APK

---

## ❓ FAQ

**Q: Is the "+1MB for 115GB" claim real?**

A: Yes. Run `pmap -x $(pgrep lazarus) | tail -n 1` before and after loading a ZIM. Check the "Dirty" column.

**Q: Can I load multiple ZIM files?**

A: Yes! Use `--zim` multiple times or `--zim-dir`. Load Wikipedia + TED + Medical encyclopedia all at once. Still ~5MB RAM.

**Q: 4ms cold start? Really?**

A: Run `time ./lazarus --help` yourself. No JavaScript VM to initialize, no Chromium to load.

**Q: How is a 5-month programmer building this?**

A: Coffee, Claude AI pair programming, and Rust's compiler being a great teacher.

---

## 📜 License

MIT License. Do whatever you want.

---

<div align="center">

### 115GB Wikipedia. +1MB RAM. 4ms startup. Not a typo.

*"Knowledge should be free, accessible, and private."*

**— The Lazarus Project 🦴**

[⬇️ Download](https://github.com/Ochichan/lazarus/releases) · [🐛 Issues](https://github.com/Ochichan/lazarus/issues)

</div>
