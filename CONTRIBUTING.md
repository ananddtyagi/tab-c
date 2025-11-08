# Contributing to MacAutoComplete

Thank you for your interest in contributing to MacAutoComplete! This document provides guidelines and instructions for contributing.

---

## 🚀 Getting Started

### Prerequisites

- macOS 13.0+ (for IME development)
- Rust 1.91+
- Xcode 15+
- Git

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourorg/mac-autocomplete.git
cd mac-autocomplete

# Run setup script
./scripts/setup_dev.sh

# Build the engine
make engine

# Run tests
make test
```

---

## 📝 Code Style

### Rust

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings

```bash
# Format code
make format

# Run linter
make lint
```

### Swift

- Follow [Swift API Design Guidelines](https://swift.org/documentation/api-design-guidelines/)
- Use SwiftLint (if configured)
- 4 spaces for indentation

---

## 🧪 Testing

### Rust Tests

```bash
# Run all tests
cd engine && cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

### Manual IME Testing

1. Build the IME in Xcode
2. Install to `~/Library/Input Methods/`
3. Enable in System Settings
4. Test in various apps:
   - TextEdit
   - Safari address bar
   - Slack
   - VS Code
   - Terminal

---

## 🐛 Reporting Bugs

### Before Submitting

1. Check existing [issues](https://github.com/yourorg/mac-autocomplete/issues)
2. Update to the latest version
3. Reproduce the bug in a clean environment

### Bug Report Template

```markdown
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce:
1. Open TextEdit
2. Type "hello "
3. Press Tab
4. See error

**Expected behavior**
Suggestion should be accepted.

**Actual behavior**
Nothing happens.

**Environment:**
- macOS version: 14.0
- MacAutoComplete version: 0.1.0
- CPU: M1 Pro
- RAM: 16GB

**Logs**
```
Paste relevant logs here
```
```

---

## ✨ Feature Requests

We welcome feature requests! Please:

1. Check if the feature already exists or is planned
2. Explain the use case
3. Provide examples or mockups if applicable

Use the [Feature Request template](https://github.com/yourorg/mac-autocomplete/issues/new?template=feature_request.md)

---

## 🔀 Pull Requests

### Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests
5. Run tests and linters
6. Commit with clear messages
7. Push to your fork
8. Open a Pull Request

### PR Guidelines

- **One feature per PR** — Keep PRs focused
- **Write tests** — For new features and bug fixes
- **Update docs** — If adding/changing features
- **Follow code style** — Use formatters and linters
- **Meaningful commits** — Use clear commit messages

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

Example:
```
feat(engine): add code-aware suggestion mode

- Detect programming language from file extension
- Use language-specific stop tokens
- Bias towards code completions

Closes #123
```

---

## 🏗️ Architecture Overview

Understanding the architecture helps when contributing:

```
┌──────────────────────────────────────┐
│  IME (Swift/InputMethodKit)          │
│  • User-facing UI                    │
│  • Event handling                    │
└────────────┬─────────────────────────┘
             │ IPC (Unix socket)
             ▼
┌──────────────────────────────────────┐
│  Engine (Rust)                       │
│  • Instant suggestions (Trie)        │
│  • LLM refinement (llama.cpp)        │
│  • Caching & ranking                 │
└──────────────────────────────────────┘
```

See [ARCHITECTURE.md](ARCHITECTURE.md) for details.

---

## 🎯 Good First Issues

Look for issues labeled [`good first issue`](https://github.com/yourorg/mac-autocomplete/labels/good%20first%20issue):

- Documentation improvements
- Simple bug fixes
- Adding tests
- UI polish

---

## 💡 Development Tips

### Debugging the Engine

```bash
# Run with debug logging
RUST_LOG=debug ./engine/target/debug/mac_autocomplete_engine

# Attach debugger
lldb ./engine/target/debug/mac_autocomplete_engine
```

### Debugging the IME

1. Build in Xcode
2. Product → Run
3. Set breakpoints in Swift code
4. Trigger autocomplete in test app

### Performance Profiling

```bash
# Rust: Flamegraph
cargo install flamegraph
cargo flamegraph

# Swift: Instruments
# Open Instruments → Time Profiler
# Attach to IME process
```

---

## 📚 Resources

- [InputMethodKit Documentation](https://developer.apple.com/documentation/inputmethodkit)
- [Rust Book](https://doc.rust-lang.org/book/)
- [llama.cpp GitHub](https://github.com/ggerganov/llama.cpp)
- [SwiftUI Tutorials](https://developer.apple.com/tutorials/swiftui)

---

## 🤝 Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive experience for everyone.

### Standards

- Be respectful and constructive
- Accept feedback gracefully
- Focus on what's best for the community
- Show empathy towards others

### Enforcement

Violations can be reported to support@macautocomplete.com. All complaints will be reviewed promptly and fairly.

---

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## ❓ Questions?

- Open a [Discussion](https://github.com/yourorg/mac-autocomplete/discussions)
- Join our [Discord](https://discord.gg/macautocomplete) (if available)
- Email: support@macautocomplete.com

---

**Thank you for contributing to MacAutoComplete!** 🎉
