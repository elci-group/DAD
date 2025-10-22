use anyhow::Result;
use clap::{Parser, Subcommand};

mod audit;
mod predict;
mod learn;
mod export;

#[derive(Parser)]
#[command(name = "dad")]
#[command(about = "DAD - The Dumbass Daemon CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// View recent errors
    Audit {
        /// Number of recent errors to show
        #[arg(short, long, default_value = "20")]
        limit: usize,
        
        /// Output format (table, json)
        #[arg(short, long, default_value = "table")]
        format: String,
    },
    
    /// Predict failure probability for a command
    Predict {
        /// Command to predict
        command: String,
    },
    
    /// Manually trigger learning update
    Learn {
        /// Hash of event to learn from
        hash: Option<String>,
    },
    
    /// View error knowledge graph
    Graph {
        /// Number of recent errors to include
        #[arg(long, default_value = "100")]
        last: usize,
    },
    
    /// Export knowledge base
    Export {
        /// Output format (json, csv)
        #[arg(short, long, default_value = "json")]
        format: String,
        
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Audit { limit, format } => {
            audit::run(limit, &format).await?;
        }
        Commands::Predict { command } => {
            predict::run(&command).await?;
        }
        Commands::Learn { hash } => {
            learn::run(hash).await?;
        }
        Commands::Graph { last } => {
            println!("Graph visualization not yet implemented (last: {})", last);
        }
        Commands::Export { format, output } => {
            export::run(&format, output).await?;
        }
    }
    
    Ok(())
}
