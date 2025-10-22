# DAD Development Guide

## Architecture Overview

DAD consists of three main components:

1. **Rust Daemon Core** (`dad-core/`) - Event listener, hashing, storage
2. **Python ML Sidecar** (`ml-sidecar/`) - Embedding generation and online learning
3. **CLI Tool** (`dadctl/`) - User interface for querying and managing the system

## Building from Source

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Python 3.9+ with pip
- SQLite 3.x
- netcat (for shell hooks)

### Build Steps

```bash
# Clone repository
git clone <repo-url>
cd DAD

# Build Rust components
cargo build --release

# Install Python dependencies
cd ml-sidecar
pip install -r requirements.txt
pip install -e .
cd ..
```

## Running in Development

### Start the daemon manually

```bash
# Terminal 1: Run daemon with debug logging
RUST_LOG=dad_core=debug cargo run --bin dad-daemon

# Terminal 2: Test CLI
cargo run --bin dad -- audit --limit 10
```

### Testing shell integration

```bash
# Source the bash hook
source scripts/shell-hooks/bash-hook.sh

# Trigger some errors to test capture
python3 -c "import nonexistent"
ls /nonexistent/path
```

### Testing ML components

```bash
# Test embedding generation
python3 -m ml_sidecar.embed --file config/example-event.json

# Test micro-training
python3 -m ml_sidecar.train --file config/example-event.json --label true
```

## Project Structure

```
DAD/
├── dad-core/               # Rust daemon
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   ├── event.rs       # Event data structures
│   │   ├── listener.rs    # Unix socket listener
│   │   ├── hasher.rs      # Deterministic hashing
│   │   ├── diff.rs        # Context diff generation
│   │   ├── trainer.rs     # ML sidecar interface
│   │   └── storage.rs     # SQLite persistence
│   └── Cargo.toml
│
├── dadctl/                 # CLI tool
│   ├── src/
│   │   ├── main.rs        # CLI entry point
│   │   ├── audit.rs       # Audit command
│   │   ├── predict.rs     # Prediction command
│   │   ├── learn.rs       # Learning command
│   │   └── export.rs      # Export command
│   └── Cargo.toml
│
├── ml-sidecar/            # Python ML components
│   ├── ml_sidecar/
│   │   ├── embed.py       # SentenceTransformer embeddings
│   │   ├── train.py       # River online learning
│   │   └── storage.py     # DuckDB caching (TODO)
│   ├── requirements.txt
│   └── setup.py
│
├── scripts/
│   ├── shell-hooks/       # Shell integration
│   │   ├── bash-hook.sh
│   │   └── zsh-hook.sh    # (TODO)
│   ├── systemd/           # Service files
│   │   └── dad.service
│   └── install-service.sh # Installation script
│
├── config/                # Configuration templates
│   ├── dad.toml          # Main config
│   └── example-event.json
│
├── schemas/              # Database schemas
│   └── sqlite-schema.sql
│
└── docs/                 # Documentation
    ├── DEVELOPMENT.md    # This file
    └── API.md           # (TODO)
```

## Testing

### Rust tests

```bash
cargo test
```

### Python tests

```bash
cd ml-sidecar
pytest tests/  # (TODO: add tests)
```

## Key Design Decisions

### Deterministic Hashing

Errors are hashed using `SHA1(command + stderr_signature + env_hash)` to ensure:
- Identical errors produce identical hashes
- Similar errors can be grouped
- Storage efficiency through deduplication

### Async Event Handling

The daemon uses Tokio for async I/O to handle multiple error events concurrently without blocking.

### Micro-Training

Each error triggers an incremental model update rather than batch retraining, enabling:
- Real-time learning
- Low memory footprint
- Continuous adaptation

### Diff-Based Context

Only deltas are stored between error occurrences to minimize storage overhead.

## Contributing

### Code Style

- Rust: Follow `rustfmt` defaults
- Python: Follow PEP 8 with Black formatter

### Commit Messages

Use conventional commit format:
- `feat: add new feature`
- `fix: resolve bug`
- `docs: update documentation`
- `refactor: code restructuring`

## Debugging

### Enable verbose logging

```bash
# Rust daemon
RUST_LOG=dad_core=trace cargo run --bin dad-daemon

# Python components
python3 -m ml_sidecar.embed --file config/example-event.json --verbose
```

### Inspect database

```bash
sqlite3 ~/.local/share/dad/dad.db
```

```sql
-- View recent errors
SELECT * FROM recent_errors LIMIT 10;

-- Check error frequency
SELECT * FROM error_frequency LIMIT 10;
```

## Performance Considerations

- **Socket Communication**: Unix domain sockets for minimal latency
- **Async Processing**: Non-blocking event capture
- **Incremental Learning**: O(1) training updates per event
- **Embedding Cache**: DuckDB for fast similarity searches (TODO)

## Roadmap

- [ ] Phase 1: Core prototype (in progress)
- [ ] Phase 2: ML integration (in progress)
- [ ] Phase 3: Predictive warnings
- [ ] Phase 4: Federated learning
- [ ] Phase 5: Visualization tools
