# DAD — The Dumbass Daemon

> **Every mistake is a message.**

DAD converts shell-level failure into structured intelligence through continuous, lightweight, deterministic, on-error learning.

## 🧠 Core Principle

The daemon operates as an always-on, low-latency observer of execution contexts. Its goal: to turn stderr noise into predictive insight without introducing significant compute or memory overhead.

## ⚙️ Architecture

```
USER SHELL → DAD CORE (Rust) → ML SIDE-CAR (Python) → KNOWLEDGE GRAPH → USER INTERFACES
```

### Components

- **Event Listener**: Hooks STDERR via shell wrappers, PTY interception, or systemd journald
- **Deterministic Error Hashing**: Reproducible identification of equivalent error events
- **Diff-based Caching**: Stores only deltas to reduce storage overhead
- **Micro-Trainer**: Lightweight ML update process with incremental learning
- **Knowledge Graph**: Semantic similarity network of error patterns

## 🚀 Technology Stack

| Layer | Tech | Rationale |
|-------|------|-----------|
| Core Daemon | Rust | Safety, async IO, low memory footprint |
| Micro-Training | Python (River, FastText, SentenceTransformers) | Mature ML tooling; pluggable |
| Storage | SQLite + DuckDB | Embedded persistence; efficient query & analysis |
| Messaging | ZeroMQ / gRPC | Cross-language comms |
| Packaging | Systemd service + CLI (dadctl) | Easy deployment |

## 🔁 Operational Modes

- **Passive**: Logs and learns silently
- **Advisory**: Suggests known fixes or causes
- **Predictive**: Warns before execution
- **Collaborative**: Federated error sharing (opt-in)

## 📦 Installation

```bash
# Build the Rust daemon
cargo build --release

# Install Python dependencies
pip install -r ml-sidecar/requirements.txt

# Install systemd service
sudo ./scripts/install-service.sh

# Enable shell hooks
source scripts/shell-hooks/bash-hook.sh  # or zsh-hook.sh
```

## 🎯 Usage

```bash
# View recent errors
dad audit

# Get prediction for command
dad predict "python script.py"

# Force learning update
dad learn

# View error graph
dad graph --last 100

# Export knowledge base
dad export --format json
```

## 🧮 Learning Pipeline

1. Error → Event JSON
2. Event JSON → Embedding (Text + Diff)
3. Embedding → Micro-Training Update
4. Model Delta → Merge
5. Checkpoint → Predictive Readiness

## 🧱 Philosophical Standards

- **Determinism**: Same failure → same hash → same learning state
- **Asymptotic Adaptivity**: System grows sharper over time without forgetting valid variance
- **Informational Conservation**: Nothing is wasted — every error adds value
- **Decentralized Intelligence**: Every node learns locally but can federate optionally
- **Low Friction, High Yield**: Near-zero cognitive overhead for the developer

## 🧭 Development Roadmap

- [x] Phase 0: Project scaffolding
- [ ] Phase 1: Core Prototype (Rust listener + SQLite cache + CLI)
- [ ] Phase 2: Micro-Trainer Integration (Online ML + Embedding model)
- [ ] Phase 3: Predictive Layer (Real-time failure probability)
- [ ] Phase 4: Federated Mode (Peer sync and shared embeddings)
- [ ] Phase 5: Visualization & Intelligence (Error graph viewer + Plugin API)

## 📁 Project Structure

```
DAD/
├── dad-core/           # Rust daemon
│   ├── src/
│   │   ├── listener/   # Event capture
│   │   ├── hasher/     # Deterministic hashing
│   │   ├── diff/       # Delta computation
│   │   ├── trainer/    # Micro-trainer spawner
│   │   └── notify/     # Desktop notifications
│   └── Cargo.toml
├── ml-sidecar/         # Python ML components
│   ├── embedding/      # Text encoders
│   ├── learning/       # Online learning
│   └── storage/        # Embedding cache
├── dadctl/             # CLI interface
├── schemas/            # Database schemas
├── config/             # Configuration templates
├── scripts/            # Installation and hooks
└── docs/               # Additional documentation
```

## 🧬 Tagline

**"The daemon that never stops learning — because every mistake matters."**

## 📄 License

MIT License - see LICENSE file for details
