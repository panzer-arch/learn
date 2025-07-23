use crate::parser::url::load;
use anyhow::Result;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    let result = load("http://localhost:8080/").await;
    match result {
        Ok(url) => {
            println!("Loaded URL: {}", url.url);
        }
        Err(e) => {
            eprintln!("Error loading URL: {}, {}", e, e.backtrace());
        }
    }
    Ok(())
}
