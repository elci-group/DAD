use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a single error event captured from the shell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    /// Deterministic hash of the error
    pub hash: String,
    
    /// Timestamp of the error
    pub timestamp: DateTime<Utc>,
    
    /// The command that was executed
    pub command: String,
    
    /// Working directory where command was executed
    pub working_dir: String,
    
    /// Exit code of the command
    pub exit_code: i32,
    
    /// STDERR output
    pub stderr: String,
    
    /// STDOUT output (optional, for context)
    pub stdout: Option<String>,
    
    /// Environment signature/fingerprint
    pub env_signature: String,
    
    /// Context diff from previous state
    pub context_diff: Option<String>,
    
    /// Embedding vector (populated by ML sidecar)
    pub embedding: Option<Vec<f32>>,
    
    /// Whether a fix was applied
    pub fix_applied: bool,
    
    /// User-provided notes or tags
    pub notes: Option<String>,
}

impl ErrorEvent {
    pub fn new(
        command: String,
        working_dir: String,
        exit_code: i32,
        stderr: String,
        env_signature: String,
    ) -> Self {
        Self {
            hash: String::new(), // Will be computed by hasher
            timestamp: Utc::now(),
            command,
            working_dir,
            exit_code,
            stderr,
            stdout: None,
            env_signature,
            context_diff: None,
            embedding: None,
            fix_applied: false,
            notes: None,
        }
    }
    
    /// Get a shortened version of stderr for display
    pub fn stderr_preview(&self, max_len: usize) -> String {
        if self.stderr.len() <= max_len {
            self.stderr.clone()
        } else {
            format!("{}...", &self.stderr[..max_len])
        }
    }
    
    /// Check if this error is likely recoverable based on exit code
    pub fn is_recoverable(&self) -> bool {
        matches!(self.exit_code, 1 | 2 | 126 | 127)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_creation() {
        let event = ErrorEvent::new(
            "python script.py".to_string(),
            "/home/user/project".to_string(),
            1,
            "ModuleNotFoundError: No module named 'numpy'".to_string(),
            "linux-python3.11".to_string(),
        );
        
        assert_eq!(event.command, "python script.py");
        assert_eq!(event.exit_code, 1);
        assert!(event.is_recoverable());
    }
    
    #[test]
    fn test_stderr_preview() {
        let event = ErrorEvent::new(
            "test".to_string(),
            "/tmp".to_string(),
            1,
            "A".repeat(100),
            "test-env".to_string(),
        );
        
        let preview = event.stderr_preview(50);
        assert_eq!(preview.len(), 53); // 50 + "..."
    }
}
