# Architecture Documentation

## Overview

MacAutoComplete is a system-wide autocomplete solution for macOS built with three main components:

1. **IME (Input Method Editor)** — Swift/InputMethodKit
2. **Engine** — Rust daemon with dual suggestion system
3. **Host App** — SwiftUI menu bar application

---

## Component Details

### 1. IME (Input Method Editor)

**Technology:** Swift, InputMethodKit (Apple framework)

**Files:**
- `AutocompleteInputController.swift` — Main IME controller
- `CandidateWindow.swift` — UI for displaying suggestions
- `EngineBridge.swift` — IPC client to communicate with engine

**Responsibilities:**
- Capture text input events from any macOS application
- Extract context (prefix text before cursor)
- Send completion requests to engine
- Display candidate window with suggestions
- Handle user acceptance (Tab key) and dismissal (Esc)
- Detect secure text fields and disable suggestions

**Key APIs:**
- `IMKInputController` — Base class for input method
- `IMKTextInput` — Protocol for client apps
- `IMKCandidates` or custom `NSPanel` — Candidate display

**Flow:**
1. User types in any app
2. IME receives keystroke event
3. Extract prefix (up to 512 chars)
4. Send `complete` request to engine via socket
5. Receive `instant` response (< 50ms)
6. Display candidates
7. Optionally receive `refined` response (< 300ms)
8. Update candidates

---

### 2. Engine (Rust Daemon)

**Technology:** Rust, Tokio (async runtime)

**Architecture:**

```
┌──────────────────────────────────────────┐
│         Unix Domain Socket Server        │
│         (ipc/uds.rs)                     │
└────────────┬─────────────────────────────┘
             │
             ├─→ Request Handler
             │
      ┌──────┴────────┐
      │               │
      ▼               ▼
┌──────────┐    ┌────────────┐
│ Instant  │    │   LLM      │
│ Engine   │    │  Engine    │
└──────────┘    └────────────┘
      │               │
      ▼               ▼
   Trie/Cache    llama.cpp
                   server
```

**Modules:**

#### `ipc/` — Inter-Process Communication
- `protocol.rs` — Message definitions (Request/Response)
- `uds.rs` — Unix domain socket server

Protocol:
```
[4-byte length (u32 LE)] + [JSON payload]
```

Messages:
- `Request::Complete` — Request completions
- `Request::Cancel` — Cancel ongoing request
- `Response::Instant` — Quick suggestions
- `Response::Refined` — LLM suggestions

#### `core/instant.rs` — Instant Suggestion Engine

**Data Structure:** Trie (prefix tree)

```rust
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    candidates: Vec<(String, f32)>,  // (completion, score)
    is_end: bool,
}
```

**Algorithm:**
1. Navigate trie to prefix node
2. Collect all completions via DFS
3. Sort by score
4. Return top-k

**Latency:** 5-15ms typical

**Learning:** Optionally insert accepted completions

#### `core/llm.rs` — LLM Engine

**Mode:** HTTP client to llama.cpp server

**Setup:**
```bash
# Start llama.cpp server
./server -m tinyllama.gguf -ngl 33 --ctx-size 1024 \
  --host 127.0.0.1 --port 8087 --mlock --parallel 4
```

**Request:**
```json
{
  "prompt": "The quick brown ",
  "n_predict": 64,
  "temperature": 0.7,
  "top_k": 40,
  "top_p": 0.9,
  "stop": ["\n\n", "\n"],
  "cache_prompt": true
}
```

**Optimizations:**
- KV cache reuse (via `cache_prompt`)
- Context trimming (keep last 512 tokens)
- Request cancellation on new input
- Metal/MPS acceleration

**Latency:** 150-300ms for 32-64 tokens

#### `core/cache.rs` — LRU Cache

**Purpose:** Avoid redundant computations

**Key:** `"{app_id}:{prefix}"`

**Value:** `Vec<String>` (candidates)

**Size:** 10,000 entries (configurable)

**Hit Rate:** ~70-80% in typical usage

#### `core/rank.rs` — Ranking & Merging

**Strategy:**
1. Prioritize refined suggestions if they match prefix
2. Add instant suggestions
3. Deduplicate (case-insensitive)
4. Sort by length (shorter first) and score

**Scoring Factors:**
- Exact prefix match: +1.0
- Length penalty: 1.0 / (1.0 + len/100)
- Context similarity: word overlap * 0.1

#### `config.rs` — Configuration

**File:** `~/Library/Application Support/MacAutoComplete/config.json`

**Structure:**
```json
{
  "socket_path": "...",
  "model_path": "...",
  "instant": { ... },
  "llm": { ... },
  "app_rules": { ... },
  "privacy": { ... },
  "cache": { ... }
}
```

**Per-App Rules:**
```json
"com.apple.TextEdit": {
  "enabled": true,
  "min_prefix_length": 3,
  "max_suggestion_tokens": 64,
  "aggressiveness": 0.7,
  "multiline": false
}
```

#### `metrics.rs` — Performance Tracking

**Metrics Collected:**
- Total requests
- Instant latency (p50, p95)
- Refined latency (p50, p95)
- Cache hit rate
- Acceptance rate

**Storage:** In-memory ring buffers (1000 samples)

**Access:** Via `GetMetrics` request

---

### 3. Host App (Menu Bar)

**Technology:** SwiftUI, AppKit

**Files:**
- `IMEAppDelegate.swift` — App lifecycle, menu bar
- `PreferencesUI.swift` — SwiftUI preferences views

**Responsibilities:**
- Start/stop engine daemon
- Provide preferences UI
- Guide user through setup
- Display status in menu bar

**Menu:**
- Preferences...
- Open Input Sources...
- Restart Engine
- View Metrics
- Quit

---

## Data Flow

### Completion Request Flow

```
┌─────────┐
│  User   │ Types "hello w"
└────┬────┘
     │
     ▼
┌─────────────────┐
│  macOS App      │
│  (TextEdit)     │
└────┬────────────┘
     │ Input event
     ▼
┌─────────────────────────────────────┐
│  IME (AutocompleteInputController)  │
│  • Extract prefix: "hello w"        │
│  • Get app: "com.apple.TextEdit"    │
│  • Generate cursor_id: "uuid-123"   │
└────┬────────────────────────────────┘
     │ Unix socket
     │ {"type":"complete", "prefix":"hello w", ...}
     ▼
┌──────────────────────────────────────┐
│  Engine (UnixSocketServer)           │
│  • Decode request                    │
│  • Check app rules                   │
│  • Check cache                       │
└────┬──────┬──────────────────────────┘
     │      │
     │      └──────┐
     ▼             ▼
┌────────────┐  ┌──────────────────┐
│  Instant   │  │  LLM (async)     │
│  • Trie    │  │  • HTTP to       │
│  • 15ms    │  │    llama.cpp     │
└────┬───────┘  │  • 180ms         │
     │          └───┬──────────────┘
     │              │
     ▼              ▼
┌─────────────────────────────────────┐
│  Response(s)                        │
│  1. instant: ["world", "welcome"]   │
│  2. refined: ["world tour", ...]    │
└────┬────────────────────────────────┘
     │ Unix socket
     ▼
┌──────────────────────────────────────┐
│  IME                                 │
│  • Receive instant → show window     │
│  • Receive refined → update window   │
└────┬─────────────────────────────────┘
     │
     ▼
┌──────────────────┐
│  Candidate       │
│  Window          │
│  • world         │ ← Selected
│  • world tour    │
└──────────────────┘
     │
     │ User presses Tab
     ▼
┌─────────────────┐
│  macOS App      │
│  Text: "hello world"
└─────────────────┘
```

---

## Performance Optimization

### 1. Instant Engine

**Techniques:**
- In-memory trie (no disk I/O)
- Lazy loading of large tries via memmap
- Prefix hash for cache keys
- Early termination (collect only top-k)

### 2. LLM Engine

**Techniques:**
- KV cache reuse (avoid recomputing prefix)
- Request cancellation (new input invalidates old)
- Context trimming (keep last 512 tokens)
- Metal/MPS GPU acceleration
- Quantization (Q4_K_M: 4-bit weights)

### 3. IPC

**Techniques:**
- Unix domain sockets (faster than TCP loopback)
- Length-prefixed binary protocol
- Async I/O (Tokio)
- Connection pooling (keep socket open)

### 4. Caching

**Strategies:**
- LRU eviction
- Cache key: `app_id + prefix`
- Size limit: 10k entries
- No disk persistence (memory only)

---

## Security & Privacy

### Secure Field Detection

**IME Side:**
```swift
func isSecureField(client: IMKTextInput) -> Bool {
    // Check if text input is secure (password field)
    // Disable autocomplete if true
}
```

**Engine Side:**
- App exclusion list: `["1Password", "Keychain Access", ...]`
- Per-app `enabled: false` rule

### Data Minimization

**What's Stored:**
- Trie data: learned completions (if enabled)
- Cache: temporary prefix→candidates mapping
- Metrics: aggregated latency/acceptance stats

**What's NOT Stored:**
- Raw keystrokes
- Full document content
- User identity
- Network requests

### Permissions

**Required:**
- None (IME framework handles access)

**Optional:**
- Accessibility (for advanced caret position detection)

---

## Scaling Considerations

### Model Size

| Model          | Size  | RAM   | Latency |
|----------------|-------|-------|---------|
| TinyLlama 1.1B Q4 | 0.7GB | 2GB   | 180ms   |
| Qwen2.5-1.5B Q4   | 1.0GB | 2.5GB | 220ms   |
| Phi-2 Q4          | 1.6GB | 3GB   | 300ms   |

### Cache Tuning

- Small (1k entries): Low memory, more misses
- Medium (10k): Balanced
- Large (100k): High memory, best hit rate

### Per-App Rules

**Text Editors:**
```json
{
  "min_prefix_length": 2,
  "max_suggestion_tokens": 128,
  "aggressiveness": 0.9,
  "multiline": true
}
```

**Browsers:**
```json
{
  "min_prefix_length": 3,
  "max_suggestion_tokens": 32,
  "aggressiveness": 0.5,
  "multiline": false
}
```

---

## Future Architecture

### Plugin System

Allow third-party suggestion sources:

```
┌──────────────┐
│    Engine    │
└──────┬───────┘
       │
       ├─→ Instant Engine (built-in)
       ├─→ LLM Engine (built-in)
       ├─→ Snippet Plugin (user)
       └─→ Translation Plugin (user)
```

### Cloud Sync (Opt-in)

Encrypted sync of learned phrases:

```
Local Trie → Encrypt → iCloud/Sync → Decrypt → Remote Device
```

### Multi-Language

Per-language trie + model:

```json
{
  "language": "es",
  "trie_path": "trie_es.bin",
  "model_path": "tinyllama_es.gguf"
}
```

---

## Debugging

### Enable Debug Logging

**Engine:**
```bash
RUST_LOG=debug ./engine
```

**IME:**
- Xcode Console when running from Xcode
- `/var/log/system.log` for installed IME

### Inspect Socket

```bash
# Check socket exists
ls -l ~/Library/Application\ Support/MacAutoComplete/engine.sock

# Test connection
nc -U ~/Library/Application\ Support/MacAutoComplete/engine.sock
```

### Profile Performance

**Rust:**
```bash
cargo build --release
cargo flamegraph
```

**Swift:**
- Instruments → Time Profiler
- Sample process while typing

---

## References

- [InputMethodKit Documentation](https://developer.apple.com/documentation/inputmethodkit)
- [llama.cpp GitHub](https://github.com/ggerganov/llama.cpp)
- [Tokio Async Runtime](https://tokio.rs)
