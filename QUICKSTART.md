# Quick Start Guide

Get MacAutoComplete running in under 10 minutes!

---

## Prerequisites Check

Before starting, ensure you have:

- [ ] macOS 13.0+ (Ventura or later)
- [ ] Xcode installed (`xcode-select --install`)
- [ ] Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

---

## Step 1: Clone & Setup (2 minutes)

```bash
# Clone repository
git clone https://github.com/yourorg/mac-autocomplete.git
cd mac-autocomplete

# Run automated setup
./scripts/setup_dev.sh

# You should see:
# ✓ Xcode Command Line Tools
# ✓ Homebrew
# ✓ Rust
```

---

## Step 2: Build Engine (3 minutes)

```bash
# Build the Rust engine
make engine

# You should see:
# Compiling mac_autocomplete_engine...
# ✓ Engine built successfully
```

Verify:
```bash
./engine/target/release/mac_autocomplete_engine --help
```

---

## Step 3: Download Model (Optional, 5 minutes)

```bash
# Download TinyLlama model (~700MB)
./scripts/download_models.sh

# Select: 1 (TinyLlama 1.1B Q4_K_M)
```

**Skip this if:** You want to test instant suggestions only (no LLM).

---

## Step 4: Build IME (2 minutes)

```bash
# Open Xcode project
open mac/IME/IMEApp.xcodeproj
```

In Xcode:
1. Select `IMEApp` scheme
2. **Product → Build** (⌘B)
3. Wait for build to complete

---

## Step 5: Install IME (1 minute)

### Option A: Manual Install

```bash
# Copy IME bundle
cp -r ~/Library/Developer/Xcode/DerivedData/.../AutocompleteIME.bundle \
  ~/Library/Input\ Methods/
```

### Option B: Automated Install

```bash
make install
```

### Enable IME

1. Open **System Settings**
2. Go to **Keyboard → Input Sources**
3. Click **+** button
4. Search for "MacAutoComplete"
5. Click **Add**

---

## Step 6: Run! (30 seconds)

### Start the Engine

```bash
# Development mode (shows logs)
make dev

# OR run in background
./scripts/dev_run.sh
```

You should see:
```
MacAutoComplete Engine starting...
Socket path: /Users/you/Library/Application Support/MacAutoComplete/engine.sock
Instant engine ready
Engine listening...
```

### Start the Host App

Either:
- Run from Xcode (Product → Run)
- Or launch from Applications folder

You should see a **⌨️** icon in your menu bar.

---

## Step 7: Test! (1 minute)

1. Open **TextEdit**
2. Type: `hello w`
3. You should see suggestions appear!
4. Press **Tab** to accept

### First Test Checklist

- [ ] Suggestion appears within 1 second
- [ ] Tab accepts the suggestion
- [ ] Esc dismisses the suggestion
- [ ] Arrow keys navigate candidates

---

## Troubleshooting

### Suggestions not appearing?

```bash
# Check engine is running
ps aux | grep mac_autocomplete_engine

# Check socket exists
ls -l ~/Library/Application\ Support/MacAutoComplete/engine.sock
```

### IME not in Input Sources?

1. Log out and log back in
2. Make sure bundle is in `~/Library/Input Methods/`
3. Check Console.app for errors

### Engine crashes?

```bash
# View logs
tail -f ~/Library/Application\ Support/MacAutoComplete/engine.log

# Run with debug output
RUST_LOG=debug ./engine/target/debug/mac_autocomplete_engine
```

---

## Next Steps

### Configure Per-App Settings

1. Click menu bar icon **⌨️**
2. Select **Preferences**
3. Go to **Apps** tab
4. Add apps to exclude list (e.g., password managers)

### Optimize Performance

Edit `~/Library/Application Support/MacAutoComplete/config.json`:

```json
{
  "instant": {
    "max_candidates": 5,     // More candidates = slower
    "timeout_ms": 15         // Faster timeout = quicker display
  },
  "llm": {
    "max_tokens": 32,        // Shorter suggestions = faster
    "temperature": 0.7       // Lower = more predictable
  }
}
```

### Learn More

- [README.md](README.md) — Full documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) — How it works
- [CONTRIBUTING.md](CONTRIBUTING.md) — Development guide

---

## Quick Commands Reference

```bash
# Build everything
make all

# Run tests
make test

# Format code
make format

# Clean build
make clean

# Development mode
make dev

# View help
make help
```

---

## Support

Having trouble?

- Check [Troubleshooting](README.md#troubleshooting) section
- Open an [Issue](https://github.com/yourorg/mac-autocomplete/issues)
- Join [Discussions](https://github.com/yourorg/mac-autocomplete/discussions)

---

**Congratulations! You're now running MacAutoComplete!** 🎉
