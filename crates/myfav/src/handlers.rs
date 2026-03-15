use crate::args::{Cli, Commands};
use crate::git::sync_and_commit;
use core::{Favorite, FavoriteRepository, JsonRepository};

pub fn handle_command(cli: &Cli, repo: &JsonRepository) -> anyhow::Result<()> {
    match cli.command {
        Commands::Add {
            ref title,
            ref description,
            ref url,
            ref path,
            ref categories,
            ref tags,
            deprecated,
            ref alternative,
            ref icon,
        } => {
            let mut final_categories = categories.clone();
            if let Some(p) = path {
                final_categories.extend(
                    p.split('/')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty()),
                );
            }

            if final_categories.is_empty() {
                return Err(anyhow::anyhow!("At least one category is required"));
            }

            let favorite = Favorite {
                title: title.clone(),
                description: description.clone(),
                url: url.clone(),
                categories: final_categories.clone(),
                tags: tags.clone().unwrap_or_default(),
                deprecated,
                alternative: alternative.clone(),
                icon: icon.clone(),
            };

            repo.add(favorite)?;
            println!("Added/Updated favorite: {}", title);

            let msg = format!(
                "feat({}): add {} in {} list",
                title,
                title,
                final_categories.join("/")
            );
            sync_and_commit(cli, repo, &msg)?;
        }
        Commands::Rm { ref title } => {
            if let Some(removed) = repo.remove(title)? {
                println!("Removed favorite: {}", title);
                let msg = format!(
                    "fix({}): remove {} from {} list",
                    title,
                    title,
                    removed.categories.join("/")
                );
                sync_and_commit(cli, repo, &msg)?;
            } else {
                println!("Favorite not found: {}", title);
            }
        }
        Commands::Mv {
            ref title,
            ref path,
        } => {
            if let Some(mut fav) = repo.find_by_title(title)? {
                let old_path = fav.categories.join("/");
                let new_cats: Vec<String> = path
                    .split('/')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                fav.categories = new_cats.clone();
                repo.add(fav)?;

                println!("Moved {} from {} to {}", title, old_path, path);
                let msg = format!(
                    "chore({}): move {} from {} to {}",
                    title, title, old_path, path
                );
                sync_and_commit(cli, repo, &msg)?;
            } else {
                println!("Favorite not found: {}", title);
            }
        }
        Commands::List => {
            let favorites = repo.list()?;
            for fav in favorites {
                println!("- {} ({})", fav.title, fav.url);
            }
        }
        Commands::Sync => {
            sync_and_commit(cli, repo, "docs(sync): manual sync of favorites")?;
        }
    }
    Ok(())
}
