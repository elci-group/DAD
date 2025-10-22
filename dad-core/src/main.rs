use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod listener;
mod hasher;
mod diff;
mod trainer;
mod storage;
mod event;
mod notify;

use listener::EventListener;
use storage::Storage;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dad_core=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("DAD (Dumbass Daemon) starting up...");
    
    // Initialize storage
    let storage = Storage::new("~/.local/share/dad/dad.db").await?;
    info!("Storage initialized");

    // Initialize event listener
    let listener = EventListener::new(storage).await?;
    info!("Event listener initialized");

    // Run the daemon
    info!("DAD daemon running. Listening for errors...");
    listener.run().await?;

    Ok(())
}
