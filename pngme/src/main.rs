mod args;
mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod png;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(commands) => match commands {
            cli::Commands::Encode {
                file_path,
                chunk_type,
                message,
                output_file,
            } => commands::encode(file_path, chunk_type, message, output_file.as_deref())?,
            cli::Commands::Decode {
                file_path,
                chunk_type,
            } => {
                let message = commands::decode(&file_path, &chunk_type)?;
                println!("i got the message: {}", message);
            }
            cli::Commands::Remove {
                file_path,
                chunk_type,
            } => commands::remove(&file_path, &chunk_type)?,
            _ => {
                eprintln!("Unknown command");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("No command provided. Use --help for usage information.");
            std::process::exit(1);
        }
    }
    Ok(())
}
