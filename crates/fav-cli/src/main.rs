use clap::{Parser, Subcommand};
use fav_core::{Favorite, FavoriteRepository, JsonRepository};
use fav_output::{MarkdownFormatter, JsonDistributionFormatter};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fav")]
#[command(about = "A CLI to manage your favorites", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "data/favorites.json")]
    data: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new favorite
    Add {
        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        description: String,

        #[arg(short, long)]
        url: String,

        /// Categories in order (e.g. -c Android -c Apps -c Store)
        #[arg(short, long, action = clap::ArgAction::Append, required = true)]
        categories: Vec<String>,

        /// Tags (can be repeated or comma-separated)
        #[arg(short, long, action = clap::ArgAction::Append, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },
    /// List all favorites
    List,
    /// Sync and generate output files
    Sync {
        #[arg(short, long, default_value = "README.md")]
        readme: PathBuf,

        #[arg(short, long, default_value = "dist/favorites.json")]
        dist: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let repo = JsonRepository::new(&cli.data);

    match cli.command {
        Commands::Add { title, description, url, categories, tags } => {
            let favorite = Favorite {
                title,
                description,
                url,
                categories,
                tags: tags.unwrap_or_default(),
            };
            repo.add(favorite)?;
            println!("Added favorite!");
        }
        Commands::List => {
            let favorites = repo.list()?;
            for fav in favorites {
                println!("- {} ({})", fav.title, fav.url);
            }
        }
        Commands::Sync { readme, dist } => {
            let favorites = repo.list()?;
            
            // Generate README.md
            let md = MarkdownFormatter::format(&favorites);
            std::fs::write(&readme, md)?;
            println!("Generated {}", readme.display());

            // Generate JSON distribution
            let json = JsonDistributionFormatter::format(&favorites)?;
            if let Some(parent) = dist.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&dist, json)?;
            println!("Generated {}", dist.display());

            // Git commit
            let status = std::process::Command::new("git")
                .arg("add")
                .arg(&cli.data)
                .arg(&readme)
                .arg(&dist)
                .status();
            
            if let Ok(s) = status {
                if s.success() {
                    let commit = std::process::Command::new("git")
                        .arg("commit")
                        .arg("-m")
                        .arg("docs(sync): update README and favorites distribution")
                        .status();
                    if let Ok(cs) = commit {
                        if cs.success() {
                            println!("Committed changes to git.");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
