use anyhow::Result;

pub async fn run(hash: Option<String>) -> Result<()> {
    if let Some(h) = hash {
        println!("Triggering learning for event: {}", h);
    } else {
        println!("Triggering learning for all recent events");
    }
    println!("(Learning not yet implemented)");
    Ok(())
}
