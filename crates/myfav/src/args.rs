use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fav")]
#[command(about = "A CLI to manage your favorites", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'D', long, default_value = "data/favorites.json")]
    pub data: PathBuf,

    #[arg(short = 'R', long, default_value = "README.md")]
    pub readme: PathBuf,

    #[arg(short = 'O', long, default_value = "dist/favorites.json")]
    pub dist: PathBuf,

    #[arg(long)]
    pub git: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short = 't', long)]
        title: String,

        #[arg(short = 'd', long)]
        description: String,

        #[arg(short = 'u', long)]
        url: String,

        #[arg(short = 'p', long)]
        path: Option<String>,

        #[arg(short = 'c', long = "category", action = clap::ArgAction::Append)]
        categories: Vec<String>,

        #[arg(short = 'T', long, action = clap::ArgAction::Append, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        #[arg(short = 'i', long)]
        icon: Option<String>,

        #[arg(long)]
        deprecated: bool,

        #[arg(long)]
        alternative: Option<String>,
    },
    Rm {
        title: String,
    },
    Mv {
        title: String,
        #[arg(short, long)]
        path: String,
    },
    List,
    Sync,
}
