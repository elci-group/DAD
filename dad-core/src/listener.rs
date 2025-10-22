use anyhow::Result;
use tokio::net::UnixListener;
use tokio::io::AsyncReadExt;
use tracing::{info, warn, error};

use crate::storage::Storage;
use crate::event::ErrorEvent;
use crate::hasher::compute_error_hash;

pub struct EventListener {
    storage: Storage,
    socket_path: String,
}

impl EventListener {
    pub async fn new(storage: Storage) -> Result<Self> {
        let socket_path = "/tmp/dad-events.sock".to_string();
        Ok(Self {
            storage,
            socket_path,
        })
    }
    
    pub async fn run(&self) -> Result<()> {
        // Remove old socket if exists
        let _ = std::fs::remove_file(&self.socket_path);
        
        let listener = UnixListener::bind(&self.socket_path)?;
        info!("Listening on {}", self.socket_path);
        
        loop {
            match listener.accept().await {
                Ok((mut stream, _addr)) => {
                    let storage = self.storage.clone();
                    tokio::spawn(async move {
                        let mut buffer = Vec::new();
                        if let Err(e) = stream.read_to_end(&mut buffer).await {
                            error!("Failed to read from socket: {}", e);
                            return;
                        }
                        
                        match serde_json::from_slice::<ErrorEvent>(&buffer) {
                            Ok(mut event) => {
                                // Compute deterministic hash
                                event.hash = compute_error_hash(
                                    &event.command,
                                    &event.stderr,
                                    &event.env_signature,
                                );
                                
                                info!("Received error event: {} (hash: {})", 
                                      event.stderr_preview(50), &event.hash);
                                
                                // Store event
                                if let Err(e) = storage.store_event(&event).await {
                                    error!("Failed to store event: {}", e);
                                }
                                
                                // TODO: Trigger micro-trainer
                            }
                            Err(e) => {
                                error!("Failed to deserialize event: {}", e);
                            }
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}
