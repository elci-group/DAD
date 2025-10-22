use anyhow::Result;
use tokio::process::Command;
use tracing::{info, error};

use crate::event::ErrorEvent;
use crate::notify::send_training_notification;

/// Spawn micro-trainer process for incremental learning
pub async fn trigger_micro_training(event: &ErrorEvent) -> Result<()> {
    info!("Triggering micro-training for event: {}", event.hash);
    
    // Serialize event to JSON
    let event_json = serde_json::to_string(event)?;
    
    // Spawn Python ML sidecar
    let output = Command::new("python3")
        .arg("-m")
        .arg("ml_sidecar.train")
        .arg("--event")
        .arg(&event_json)
        .output()
        .await?;
    
    if output.status.success() {
        info!("Micro-training completed successfully for {}", event.hash);
        
        // Send notification about training episode
        if let Err(e) = send_training_notification(event, "Training Complete").await {
            error!("Failed to send notification: {}", e);
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Micro-training failed: {}", stderr);
        
        // Notify about training failure
        if let Err(e) = send_training_notification(event, "Training Failed").await {
            error!("Failed to send notification: {}", e);
        }
    }
    
    Ok(())
}

/// Request embedding for an error event from ML sidecar
pub async fn compute_embedding(event: &ErrorEvent) -> Result<Vec<f32>> {
    let event_json = serde_json::to_string(event)?;
    
    let output = Command::new("python3")
        .arg("-m")
        .arg("ml_sidecar.embed")
        .arg("--event")
        .arg(&event_json)
        .output()
        .await?;
    
    if output.status.success() {
        let embedding: Vec<f32> = serde_json::from_slice(&output.stdout)?;
        Ok(embedding)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Embedding computation failed: {}", stderr);
        Err(anyhow::anyhow!("Failed to compute embedding"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_trigger_micro_training() {
        // This test requires ML sidecar to be available
        // For now, it's a placeholder
    }
}
