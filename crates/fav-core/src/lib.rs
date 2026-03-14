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
