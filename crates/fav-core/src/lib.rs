use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Favorite {
    pub title: String,
    pub description: String,
    pub url: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
}

pub trait FavoriteRepository {
    fn add(&self, favorite: Favorite) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Favorite>>;
    fn save_all(&self, favorites: &[Favorite]) -> anyhow::Result<()>;
}

pub struct JsonRepository {
    path: std::path::PathBuf,
}

impl JsonRepository {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl FavoriteRepository for JsonRepository {
    fn add(&self, favorite: Favorite) -> anyhow::Result<()> {
        let mut favorites = self.list().unwrap_or_default();
        favorites.push(favorite);
        self.save_all(&favorites)
    }

    fn list(&self) -> anyhow::Result<Vec<Favorite>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let content = std::fs::read_to_string(&self.path)?;
        let favorites: Vec<Favorite> = serde_json::from_str(&content)?;
        Ok(favorites)
    }

    fn save_all(&self, favorites: &[Favorite]) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(favorites)?;
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, content)?;
        Ok(())
    }
}
