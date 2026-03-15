mod args;
mod git;
mod handlers;

use clap::Parser;
use core::JsonRepository;
use args::Cli;
use handlers::handle_command;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let repo = JsonRepository::new(&cli.data);
    handle_command(&cli, &repo)
}
