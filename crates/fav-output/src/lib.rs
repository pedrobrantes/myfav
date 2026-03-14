use fav_core::Favorite;
use std::collections::BTreeMap;

pub struct MarkdownFormatter;

impl MarkdownFormatter {
    pub fn format(favorites: &[Favorite]) -> String {
        let mut markdown = String::from("# My Favorites\n\n");
        let tree = Self::build_category_tree(favorites);
        markdown.push_str(&Self::render_tree(&tree, 0));
        markdown
    }

    fn build_category_tree(favorites: &[Favorite]) -> BTreeMap<String, CategoryNode> {
        let mut root = BTreeMap::new();
        for fav in favorites {
            let mut current_level = &mut root;
            for (i, cat) in fav.categories.iter().enumerate() {
                let node = current_level
                    .entry(cat.clone())
                    .or_insert_with(CategoryNode::default);
                if i == fav.categories.len() - 1 {
                    node.favorites.push(fav.clone());
                }
                current_level = &mut node.subcategories;
            }
        }
        root
    }

    fn render_tree(tree: &BTreeMap<String, CategoryNode>, level: usize) -> String {
        let mut output = String::new();
        let header_prefix = "#".repeat(level + 2);

        for (name, node) in tree {
            output.push_str(&format!("{} {}\n\n", header_prefix, name));
            
            for fav in &node.favorites {
                output.push_str(&format!("- [{}]({}) - {}\n", fav.title, fav.url, fav.description));
                if !fav.tags.is_empty() {
                    output.push_str(&format!("  *Tags: {}*\n", fav.tags.join(", ")));
                }
            }
            if !node.favorites.is_empty() {
                output.push('\n');
            }

            output.push_str(&Self::render_tree(&node.subcategories, level + 1));
        }
        output
    }
}

#[derive(Default)]
struct CategoryNode {
    favorites: Vec<Favorite>,
    subcategories: BTreeMap<String, CategoryNode>,
}

pub struct JsonDistributionFormatter;

impl JsonDistributionFormatter {
    pub fn format(favorites: &[Favorite]) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(favorites)?)
    }
}
