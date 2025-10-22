use anyhow::Result;

pub async fn run(command: &str) -> Result<()> {
    println!("Predicting failure probability for: {}", command);
    println!("(Prediction not yet implemented)");
    Ok(())
}
