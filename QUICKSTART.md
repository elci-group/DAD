# DAD Quick Start Guide

## What is DAD?

**DAD (The Dumbass Daemon)** is a lightweight, intelligent error-learning system that captures shell errors in real-time, learns from them using machine learning, and helps you avoid making the same mistakes twice.

**Core Principle**: Every mistake is a message.

## Quick Installation

```bash
cd /home/adminx/DAD

# 1. Build the project
cargo build --release

# 2. Install Python dependencies
cd ml-sidecar
pip3 install -r requirements.txt
cd ..

# 3. Test the daemon
RUST_LOG=dad_core=info cargo run --bin dad-daemon
```

## Manual Testing

In a separate terminal:

```bash
# Source the shell hook to capture errors
source scripts/shell-hooks/bash-hook.sh

# Trigger some errors to test
python3 -c "import nonexistentmodule"
ls /nonexistent/directory
cat /missing/file.txt

# View captured errors (once CLI is fully integrated)
cargo run --bin dad -- audit
```

## Project Structure

```
DAD/
├── dad-core/          # Rust daemon (event capture, storage, hashing)
├── dadctl/            # CLI tool (audit, predict, learn commands)
├── ml-sidecar/        # Python ML (embeddings + online learning)
├── scripts/           # Installation and shell hooks
├── config/            # Configuration templates
├── schemas/           # Database schemas
└── docs/              # Documentation
```

## Key Features

- ✅ **Deterministic Error Hashing**: Same error = same hash
- ✅ **Unix Socket Communication**: Low-latency event capture
- ✅ **SQLite Storage**: Efficient local persistence
- ✅ **Async Event Processing**: Non-blocking capture
- ✅ **Incremental Learning**: Real-time ML updates
- ⏳ **Predictive Warnings**: Coming in Phase 3
- ⏳ **Knowledge Graph**: Coming in Phase 5

## Current Status (Phase 1)

**Implemented:**
- Core Rust daemon with event listener
- Deterministic error hashing
- SQLite storage layer
- Python embedding generation (SentenceTransformers)
- Online learning with River
- CLI scaffold
- Shell hooks (Bash)
- Systemd service configuration

**In Progress:**
- CLI-storage integration
- Micro-trainer triggering
- Embedding storage

**Planned:**
- Predictive failure warnings
- Error similarity graph
- Federated learning
- Web UI / visualization

## Next Steps

1. Complete CLI integration with storage
2. Integrate micro-trainer into event pipeline
3. Add embedding persistence
4. Implement prediction engine
5. Build knowledge graph visualization

## Contributing

See `docs/DEVELOPMENT.md` for detailed development guide.

## Architecture

```
┌─────────────────────────────────────────────┐
│         USER SHELL (bash/zsh)               │
│  Command → Error → Shell Hook               │
└──────────────────┬──────────────────────────┘
                   │ JSON via Unix Socket
┌──────────────────▼──────────────────────────┐
│         DAD CORE (Rust Daemon)              │
│  • Event Listener                           │
│  • Deterministic Hasher                     │
│  • SQLite Storage                           │
│  • Diff Generator                           │
└──────────────────┬──────────────────────────┘
                   │ Spawn Process
┌──────────────────▼──────────────────────────┐
│       ML SIDECAR (Python)                   │
│  • SentenceTransformer Embeddings           │
│  • River Online Learning                    │
│  • DuckDB Cache (planned)                   │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│         CLI / USER INTERFACE                │
│  dad audit | predict | learn | export       │
└─────────────────────────────────────────────┘
```

## Philosophy

- **Determinism**: Same inputs → same outputs → reproducible learning
- **Asymptotic Adaptivity**: System gets smarter over time
- **Informational Conservation**: No error is wasted
- **Low Friction**: Zero cognitive overhead for developers
- **Decentralized Intelligence**: Learn locally, share optionally

---

**"The daemon that never stops learning — because every mistake matters."**
