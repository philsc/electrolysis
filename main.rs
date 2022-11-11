use std::env;
use serde_derive::Deserialize;
use toml::Value;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Path to the configuration file.
   #[arg(short, long)]
   config: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    linters: Vec<Linter>,
}

#[derive(Deserialize)]
struct Linter {
    name: String,
    includes: Vec<String>,
    excludes: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let contents = tokio::fs::read_to_string(&args.config)
        .await
        .unwrap_or_else(|why| {
            panic!("Failed to read {}: {:?}", args.config.as_str(), why);
        });
    let config = toml::from_str(&contents);

    let source_dir = env::var("BUILD_WORKSPACE_DIRECTORY").unwrap_or(".".to_owned());
}
