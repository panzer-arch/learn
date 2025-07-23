use crate::parser::url::load;
use anyhow::Result;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    let result = load("https://www.baidu.com/").await;
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
