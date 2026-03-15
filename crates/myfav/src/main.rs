mod args;
mod git;
mod handlers;

use args::Cli;
use clap::Parser;
use myfav_core::JsonRepository;
use handlers::handle_command;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let repo = JsonRepository::new(&cli.data);
    handle_command(&cli, &repo)
}
