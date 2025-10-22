use anyhow::Result;

pub async fn run(format: &str, output: Option<String>) -> Result<()> {
    let dest = output.unwrap_or_else(|| "stdout".to_string());
    println!("Exporting knowledge base in {} format to {}", format, dest);
    println!("(Export not yet implemented)");
    Ok(())
}
