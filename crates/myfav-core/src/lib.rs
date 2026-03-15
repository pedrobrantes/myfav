use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Favorite {
    pub title: String,
    pub description: String,
    pub url: String,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub deprecated: bool,
    pub alternative: Option<String>,
    pub icon: Option<String>,
}

pub trait FavoriteRepository {
    fn add(&self, favorite: Favorite) -> anyhow::Result<()>;
    fn list(&self) -> anyhow::Result<Vec<Favorite>>;
    fn save_all(&self, favorites: &[Favorite]) -> anyhow::Result<()>;
    fn find_by_title(&self, title: &str) -> anyhow::Result<Option<Favorite>>;
    fn remove(&self, title: &str) -> anyhow::Result<Option<Favorite>>;
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
        favorites.retain(|f| f.title != favorite.title);
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

    fn find_by_title(&self, title: &str) -> anyhow::Result<Option<Favorite>> {
        let favorites = self.list()?;
        Ok(favorites.into_iter().find(|f| f.title == title))
    }

    fn remove(&self, title: &str) -> anyhow::Result<Option<Favorite>> {
        let mut favorites = self.list()?;
        let index = favorites.iter().position(|f| f.title == title);
        if let Some(i) = index {
            let removed = favorites.remove(i);
            self.save_all(&favorites)?;
            Ok(Some(removed))
        } else {
            Ok(None)
        }
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
