use anyhow::Result;
use tokio::process::Command;
use tracing::{info, warn};

use crate::event::ErrorEvent;

/// Notification urgency levels matching dunst
#[derive(Debug, Clone, Copy)]
pub enum NotifyUrgency {
    Low,
    Normal,
    Critical,
}

impl NotifyUrgency {
    fn as_str(&self) -> &str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::Critical => "critical",
        }
    }
}

/// Send notification via notify-send (dunst compatible)
pub async fn send_training_notification(
    event: &ErrorEvent,
    training_result: &str,
) -> Result<()> {
    let summary = "🧠 DAD Learning Episode";
    
    let body = format!(
        "Command: {}\nError Type: {}\nStatus: {}",
        truncate(&event.command, 50),
        extract_error_type(&event.stderr),
        training_result
    );
    
    // Determine urgency based on error recoverability
    let urgency = if event.is_recoverable() {
        NotifyUrgency::Low
    } else {
        NotifyUrgency::Normal
    };
    
    send_notification(summary, &body, urgency, Some(5000)).await
}

/// Send notification for similar error detection
pub async fn send_similarity_notification(
    current_hash: &str,
    similar_count: usize,
) -> Result<()> {
    let summary = "🔍 DAD Pattern Detected";
    
    let body = format!(
        "Similar error detected!\n{} related errors in history\nHash: {}",
        similar_count,
        &current_hash[..8]
    );
    
    send_notification(&summary, &body, NotifyUrgency::Normal, Some(4000)).await
}

/// Send notification for prediction warning
pub async fn send_prediction_notification(
    command: &str,
    probability: f32,
) -> Result<()> {
    let summary = "⚠️  DAD Prediction Warning";
    
    let body = format!(
        "Command likely to fail: {}\nFailure probability: {:.0}%",
        truncate(command, 60),
        probability * 100.0
    );
    
    send_notification(&summary, &body, NotifyUrgency::Critical, Some(8000)).await
}

/// Core notification sender using notify-send
async fn send_notification(
    summary: &str,
    body: &str,
    urgency: NotifyUrgency,
    timeout_ms: Option<u32>,
) -> Result<()> {
    // Check if notify-send is available
    let check = Command::new("which")
        .arg("notify-send")
        .output()
        .await?;
    
    if !check.status.success() {
        warn!("notify-send not found, skipping notification");
        return Ok(());
    }
    
    let mut cmd = Command::new("notify-send");
    
    // Add arguments
    cmd.arg("--app-name=DAD")
        .arg("--urgency")
        .arg(urgency.as_str())
        .arg("--icon=dialog-information");
    
    if let Some(timeout) = timeout_ms {
        cmd.arg("--expire-time").arg(timeout.to_string());
    }
    
    // Add category for dunst filtering
    cmd.arg("--category=dad.training");
    
    cmd.arg(summary).arg(body);
    
    let output = cmd.output().await?;
    
    if output.status.success() {
        info!("Notification sent: {}", summary);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!("Failed to send notification: {}", stderr);
    }
    
    Ok(())
}

/// Extract error type from stderr for display
fn extract_error_type(stderr: &str) -> String {
    // Try to extract common error patterns
    let lines: Vec<&str> = stderr.lines().collect();
    
    if let Some(first_line) = lines.first() {
        // Python errors
        if first_line.contains("Error:") {
            if let Some(err) = first_line.split(':').next() {
                return err.trim().to_string();
            }
        }
        
        // Bash errors
        if first_line.contains("command not found") {
            return "CommandNotFound".to_string();
        }
        if first_line.contains("No such file or directory") {
            return "FileNotFound".to_string();
        }
        if first_line.contains("Permission denied") {
            return "PermissionDenied".to_string();
        }
        
        // Generic fallback
        return truncate(first_line, 30);
    }
    
    "Unknown".to_string()
}

/// Truncate string to max length with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_error_type() {
        let stderr = "ModuleNotFoundError: No module named 'numpy'";
        assert_eq!(extract_error_type(stderr), "ModuleNotFoundError");
        
        let stderr = "bash: command not found: foo";
        assert_eq!(extract_error_type(stderr), "CommandNotFound");
    }
    
    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("this is a very long string", 10), "this is...");
    }
}
