use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::event::ErrorEvent;

#[derive(Clone)]
pub struct Storage {
    conn: Arc<Mutex<Connection>>,
}

impl Storage {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let path = shellexpand::tilde(db_path.as_ref().to_str().unwrap());
        
        // Ensure parent directory exists
        if let Some(parent) = Path::new(path.as_ref()).parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let conn = Connection::open(path.as_ref())?;
        
        // Initialize schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
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
                notes TEXT
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS embeddings (
                hash TEXT PRIMARY KEY,
                embedding BLOB NOT NULL,
                FOREIGN KEY(hash) REFERENCES events(hash)
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_timestamp ON events(timestamp DESC)",
            [],
        )?;
        
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_command ON events(command)",
            [],
        )?;
        
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
    
    pub async fn store_event(&self, event: &ErrorEvent) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO events 
             (hash, timestamp, command, working_dir, exit_code, stderr, stdout, 
              env_signature, context_diff, fix_applied, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                &event.hash,
                event.timestamp.timestamp(),
                &event.command,
                &event.working_dir,
                event.exit_code,
                &event.stderr,
                &event.stdout,
                &event.env_signature,
                &event.context_diff,
                event.fix_applied as i32,
                &event.notes,
            ],
        )?;
        Ok(())
    }
    
    pub async fn get_event(&self, hash: &str) -> Result<Option<ErrorEvent>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT hash, timestamp, command, working_dir, exit_code, stderr, stdout,
                    env_signature, context_diff, fix_applied, notes
             FROM events WHERE hash = ?1"
        )?;
        
        let mut rows = stmt.query(params![hash])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(ErrorEvent {
                hash: row.get(0)?,
                timestamp: chrono::DateTime::from_timestamp(row.get(1)?, 0)
                    .unwrap_or_default(),
                command: row.get(2)?,
                working_dir: row.get(3)?,
                exit_code: row.get(4)?,
                stderr: row.get(5)?,
                stdout: row.get(6)?,
                env_signature: row.get(7)?,
                context_diff: row.get(8)?,
                embedding: None, // TODO: load from embeddings table
                fix_applied: row.get::<_, i32>(9)? != 0,
                notes: row.get(10)?,
            }))
        } else {
            Ok(None)
        }
    }
    
    pub async fn get_recent_events(&self, limit: usize) -> Result<Vec<ErrorEvent>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT hash, timestamp, command, working_dir, exit_code, stderr, stdout,
                    env_signature, context_diff, fix_applied, notes
             FROM events ORDER BY timestamp DESC LIMIT ?1"
        )?;
        
        let rows = stmt.query_map(params![limit], |row| {
            Ok(ErrorEvent {
                hash: row.get(0)?,
                timestamp: chrono::DateTime::from_timestamp(row.get(1)?, 0)
                    .unwrap_or_default(),
                command: row.get(2)?,
                working_dir: row.get(3)?,
                exit_code: row.get(4)?,
                stderr: row.get(5)?,
                stdout: row.get(6)?,
                env_signature: row.get(7)?,
                context_diff: row.get(8)?,
                embedding: None,
                fix_applied: row.get::<_, i32>(9)? != 0,
                notes: row.get(10)?,
            })
        })?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(row?);
        }
        Ok(events)
    }
}
