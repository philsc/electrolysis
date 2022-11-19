use std::env;
use serde::Deserialize;
use toml::Value;
use std::path::PathBuf;
use clap::Parser;
use std::str::FromStr;

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
   excludes: Option<Vec<String>>,
}

fn get_changed_files(path: &PathBuf) -> Result<Vec<PathBuf>, git2::Error> {
   let repo = git2::Repository::open(path)?;
   let prev_head = repo.revparse_single("HEAD^")?;
   let tree = repo.find_tree(prev_head.as_commit().unwrap().tree_id())?;
   let mut diff_options = git2::DiffOptions::new();
   let diff = repo.diff_tree_to_workdir(Some(&tree), Some(&mut diff_options))?;

   let mut result = Vec::new();
   for delta in diff.deltas().into_iter() {
      let Some(file) = delta.new_file().path() else {
         continue;
      };
      result.push(file.to_path_buf());
   }
   Ok(result)
}

#[tokio::main]
async fn main() {
   let args = Args::parse();

   let contents = tokio::fs::read_to_string(&args.config)
       .await
       .unwrap_or_else(|why| {
           panic!("Failed to read {}: {:?}", args.config.as_os_str().to_str().unwrap(), why);
       });
   let config: Config = toml::from_str(&contents).unwrap_or_else(|why| {
       panic!("Failed to parse TOML file {}: {:?}", args.config.as_os_str().to_str().unwrap(), why);
   });

   let source_dir = env::var("BUILD_WORKSPACE_DIRECTORY").unwrap_or(".".to_owned());
   let source_dir_path = PathBuf::from_str(source_dir.as_ref()).unwrap_or_else(|why| {
       panic!("Failed parse {} as path: {:?}", source_dir, why);
   });
   let changed_files = get_changed_files(&source_dir_path).unwrap_or_else(|why| {
       panic!("Failed to get list of changed files: {:?}", why);
   });

   for file in changed_files.into_iter() {
      println!("in diff: {}", file.display());
   }
}
