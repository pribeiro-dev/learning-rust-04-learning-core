
use anyhow::Result;
use learning_core::load_config;

fn main() -> Result<()> {
    let path = "app.cfg";
    if !std::path::Path::new(path).exists() {
        std::fs::write(path, "host=example.com\nport=8080\n")?;
    }
    let cfg = load_config(path)?;
    println!("Loaded {} keys", cfg.len());
    Ok(())
}
