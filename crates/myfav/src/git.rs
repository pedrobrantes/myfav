use std::process::Command;
use crate::args::Cli;
use core::JsonRepository;
use output::{MarkdownFormatter, JsonDistributionFormatter};
use core::FavoriteRepository;

pub fn sync_and_commit(cli: &Cli, repo: &JsonRepository, commit_msg: &str) -> anyhow::Result<()> {
    let favorites = repo.list()?;
    
    let existing_readme = if cli.readme.exists() {
        Some(std::fs::read_to_string(&cli.readme)?)
    } else {
        None
    };

    let md = MarkdownFormatter::format(&favorites, existing_readme);
    std::fs::write(&cli.readme, md)?;

    let json = JsonDistributionFormatter::format(&favorites)?;
    if let Some(parent) = cli.dist.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&cli.dist, json)?;

    if cli.git {
        run_git(&["add", cli.data.to_str().unwrap(), cli.readme.to_str().unwrap(), cli.dist.to_str().unwrap()])?;
        run_git(&["commit", "-m", commit_msg])?;
        println!("Sync complete and committed: {}", commit_msg);
    } else {
        println!("Sync complete (git commit skipped).");
    }
    
    Ok(())
}

fn run_git(args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new("git").args(args).status()?;
    if !status.success() {
        println!("Git command warning: {:?}", args);
    }
    Ok(())
}
