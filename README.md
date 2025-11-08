# MacAutoComplete

**Ultra-low-latency, 100% local autocomplete for macOS** — powered by a custom Input Method Editor (IME) and a Rust engine with on-device LLM inference.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg)
![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)

---

## 🚀 Features

- **System-wide autocomplete** in any text field (TextEdit, Safari, Slack, VS Code, etc.)
- **Ultra-fast instant suggestions** (≤ 50ms) via trie-based engine
- **LLM-refined completions** (≤ 300ms) using TinyLlama 1.1B quantized model
- **100% local & private** — no data ever leaves your Mac
- **Metal/MPS acceleration** on Apple Silicon
- **Per-app customization** with automatic security field detection
- **Minimal resource usage** — < 3GB RAM, < 1 CPU core when idle

---

## 📋 Requirements

- macOS 13.0+ (Ventura or later)
- Apple Silicon (M1/M2/M3) or Intel Mac
- 8GB RAM minimum (16GB recommended)
- 2GB free disk space (for model)
- Xcode 15+ (for building from source)

---

## 🎯 Quick Start

### Option 1: Install from Release (Recommended)

1. Download the latest release from [Releases](https://github.com/yourorg/mac-autocomplete/releases)
2. Extract the archive
3. Run `install.sh`
4. Open **System Settings → Keyboard → Input Sources**
5. Click **+** and add **MacAutoComplete**
6. Launch **MacAutoComplete.app** from Applications

### Option 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/yourorg/mac-autocomplete.git
cd mac-autocomplete

# Set up development environment
./scripts/setup_dev.sh

# Download model (optional, ~700MB)
./scripts/download_models.sh

# Build engine
make engine

# Build IME (requires Xcode)
open mac/IME/IMEApp.xcodeproj
# Then: Product → Build

# Run in development mode
make dev
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│  Any macOS App (TextEdit, Safari, Slack, etc.)      │
└───────────────────┬─────────────────────────────────┘
                    │ Text input events
                    ▼
┌─────────────────────────────────────────────────────┐
│  IME (InputMethodKit)                               │
│  • AutocompleteInputController.swift                │
│  • CandidateWindow.swift                            │
│  • EngineBridge.swift (IPC client)                  │
└───────────────────┬─────────────────────────────────┘
                    │ Unix domain socket
                    ▼
┌─────────────────────────────────────────────────────┐
│  Rust Engine (mac_autocomplete_engine)              │
│  ┌──────────────────────────────────────────────┐  │
│  │  Instant Engine (< 50ms)                     │  │
│  │  • Trie-based suggestions                    │  │
│  │  • LRU cache                                 │  │
│  └──────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────┐  │
│  │  LLM Engine (< 300ms)                        │  │
│  │  • llama.cpp server                          │  │
│  │  • TinyLlama 1.1B Q4_K_M                     │  │
│  │  • Metal/MPS acceleration                    │  │
│  │  • KV cache reuse                            │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

### Components

1. **IME (Swift/InputMethodKit)**
   - Captures text input system-wide
   - Displays candidate window
   - Communicates with engine via IPC

2. **Rust Engine**
   - Instant suggestions via trie data structure
   - LLM-based refinement via llama.cpp
   - Caching and per-app rules
   - Unix domain socket server

3. **Host App (SwiftUI)**
   - Menu bar application
   - Preferences UI
   - Engine lifecycle management

---

## ⚡ Performance

On M1 MacBook Pro:

| Metric                  | Target    | Actual   |
|------------------------|-----------|----------|
| Instant suggestion     | < 50ms    | ~15ms    |
| LLM suggestion         | < 300ms   | ~180ms   |
| Memory (engine + model)| < 3GB     | ~2.1GB   |
| CPU idle               | < 1 core  | ~0.3 core|

---

## 🔒 Privacy

- **100% local processing** — no network requests
- **No keystroke logging** — only completion requests
- **Secure field detection** — disabled in passwords/2FA
- **Optional learning** — disabled by default
- **No telemetry** — unless explicitly enabled

### What data is collected?

By default: **Nothing**.

If you enable learning:
- Accepted completions (stored locally in `~/Library/Application Support/MacAutoComplete`)
- Used to improve trie-based suggestions

If you enable telemetry:
- Anonymous performance metrics (latency, acceptance rate)
- No text content ever sent

---

## 🎨 Usage

### Keyboard Shortcuts

- **Tab** — Accept top suggestion
- **↓/↑** — Navigate candidates
- **Esc** — Dismiss suggestions
- **Ctrl+Space** — Manually trigger (if auto-suggest off)

### Per-App Rules

Configure behavior per app in Preferences:

- **Text editors** (VS Code, Sublime) → Longer suggestions, higher frequency
- **Terminals** → Conservative, no multiline
- **Browsers** → Balanced
- **Password managers** → Automatically disabled

---

## 🛠️ Development

### Project Structure

```
mac-autocomplete/
├── engine/              # Rust autocomplete engine
│   ├── src/
│   │   ├── main.rs
│   │   ├── ipc/         # Unix socket server
│   │   ├── core/        # Instant & LLM engines
│   │   └── config.rs
│   └── Cargo.toml
├── mac/                 # macOS components
│   └── IME/
│       ├── IMEApp/      # Host app (menu bar)
│       └── AutocompleteIME/  # IME bundle
├── models/              # Model files (.gguf)
├── scripts/             # Build & dev scripts
└── Makefile
```

### Building

```bash
# Build everything
make all

# Build engine only
make engine

# Run tests
make test

# Format code
make format

# Development mode
make dev
```

### Testing

```bash
# Run Rust tests
cd engine && cargo test

# Test IME (manual)
# 1. Build and install IME
# 2. Enable in System Settings
# 3. Type in TextEdit
```

### Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

---

## 📦 Distribution

### Code Signing

```bash
# Sign the app
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: Your Name (TEAMID)" \
  MacAutoComplete.app

# Verify
codesign --verify --verbose MacAutoComplete.app
```

### Notarization

```bash
# Submit for notarization
./scripts/notarize.sh MacAutoComplete.app

# Check status
xcrun notarytool history --apple-id your@email.com
```

### Creating Installer

```bash
# Build .pkg installer
./scripts/build_installer.sh

# Result: dist/MacAutoComplete-installer.pkg
```

---

## 🐛 Troubleshooting

### IME not appearing in Input Sources

1. Ensure the IME bundle is in `~/Library/Input Methods/` or `/Library/Input Methods/`
2. Log out and log back in
3. Check Console.app for errors

### Engine not connecting

```bash
# Check if engine is running
ps aux | grep mac_autocomplete_engine

# Check socket exists
ls -l ~/Library/Application\ Support/MacAutoComplete/engine.sock

# View engine logs
tail -f ~/Library/Logs/MacAutoComplete/engine.log
```

### Suggestions not appearing

1. Check minimum prefix length in Preferences (default: 3 characters)
2. Ensure app is not in excluded list
3. Verify model is downloaded and configured correctly

### High memory usage

- Default model (TinyLlama 1.1B Q4) uses ~2GB
- Use a smaller model or disable LLM engine
- Adjust cache size in config.json

---

## 🗺️ Roadmap

- [x] Basic IME with instant suggestions
- [x] LLM-based refined completions
- [x] Per-app customization
- [x] Preferences UI
- [ ] Multi-language support (Spanish, French, etc.)
- [ ] Code-aware completions (language detection)
- [ ] Cloud sync for learned phrases (opt-in, encrypted)
- [ ] Plugin system for custom suggestion sources
- [ ] Windows/Linux support (via alternative to IME)

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [llama.cpp](https://github.com/ggerganov/llama.cpp) — Fast LLM inference
- [TinyLlama](https://github.com/jzhang38/TinyLlama) — Compact language model
- InputMethodKit — Apple's IME framework

---

## 📬 Contact

- Issues: [GitHub Issues](https://github.com/yourorg/mac-autocomplete/issues)
- Discussions: [GitHub Discussions](https://github.com/yourorg/mac-autocomplete/discussions)
- Email: support@macautocomplete.com

---

**Made with ❤️ for Mac users who love productivity**