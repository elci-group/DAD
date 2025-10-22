-- DAD SQLite Database Schema
-- Version 1.0

-- Main events table: stores error events with deterministic hashing
CREATE TABLE IF NOT EXISTS events (
    hash TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    command TEXT NOT NULL,
    working_dir TEXT NOT NULL,
    exit_code INTEGER NOT NULL,
    stderr TEXT NOT NULL,
    stdout TEXT,
    env_signature TEXT NOT NULL,
    context_diff TEXT,
    fix_applied INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Embeddings table: stores ML embeddings separately for efficient storage
CREATE TABLE IF NOT EXISTS embeddings (
    hash TEXT PRIMARY KEY,
    embedding BLOB NOT NULL,
    model_version TEXT DEFAULT 'all-MiniLM-L6-v2',
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY(hash) REFERENCES events(hash) ON DELETE CASCADE
);

-- Similarity graph: stores semantic relationships between errors
CREATE TABLE IF NOT EXISTS error_graph (
    hash_a TEXT NOT NULL,
    hash_b TEXT NOT NULL,
    similarity REAL NOT NULL,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    PRIMARY KEY(hash_a, hash_b),
    FOREIGN KEY(hash_a) REFERENCES events(hash) ON DELETE CASCADE,
    FOREIGN KEY(hash_b) REFERENCES events(hash) ON DELETE CASCADE
);

-- Training history: tracks ML model updates
CREATE TABLE IF NOT EXISTS training_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_hash TEXT NOT NULL,
    label INTEGER NOT NULL,
    model_version TEXT,
    timestamp INTEGER DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY(event_hash) REFERENCES events(hash) ON DELETE CASCADE
);

-- Predictions: stores prediction results for analysis
CREATE TABLE IF NOT EXISTS predictions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    command TEXT NOT NULL,
    probability REAL NOT NULL,
    actual_result INTEGER,
    timestamp INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_events_command ON events(command);
CREATE INDEX IF NOT EXISTS idx_events_exit_code ON events(exit_code);
CREATE INDEX IF NOT EXISTS idx_events_env ON events(env_signature);
CREATE INDEX IF NOT EXISTS idx_graph_similarity ON error_graph(similarity DESC);
CREATE INDEX IF NOT EXISTS idx_training_timestamp ON training_log(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_predictions_timestamp ON predictions(timestamp DESC);

-- Views for common queries
CREATE VIEW IF NOT EXISTS recent_errors AS
SELECT 
    hash,
    datetime(timestamp, 'unixepoch') as error_time,
    command,
    working_dir,
    exit_code,
    substr(stderr, 1, 100) as stderr_preview,
    env_signature,
    fix_applied
FROM events
ORDER BY timestamp DESC
LIMIT 100;

CREATE VIEW IF NOT EXISTS error_frequency AS
SELECT 
    hash,
    command,
    COUNT(*) as occurrence_count,
    MAX(timestamp) as last_seen,
    AVG(exit_code) as avg_exit_code
FROM events
GROUP BY hash
ORDER BY occurrence_count DESC;
