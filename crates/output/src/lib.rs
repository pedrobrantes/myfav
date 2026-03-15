use myfav_core::Favorite;
use std::collections::BTreeMap;

pub struct MarkdownFormatter;

const START_MARKER: &str = "<!-- START_FAVORITES -->";
const END_MARKER: &str = "<!-- END_FAVORITES -->";

impl MarkdownFormatter {
    pub fn format(favorites: &[Favorite], existing_content: Option<String>) -> String {
        let tree = Self::build_category_tree(favorites);

        let mut favorites_block = String::new();
        favorites_block.push_str("## Table of Contents\n\n");
        favorites_block.push_str(&Self::render_toc(&tree, 0));
        favorites_block.push_str("\n---\n\n");
        favorites_block.push_str(&Self::render_tree(&tree, 0));

        let new_section = format!("{}\n\n{}\n{}", START_MARKER, favorites_block, END_MARKER);

        match existing_content {
            Some(content) => {
                if content.contains(START_MARKER) && content.contains(END_MARKER) {
                    let start_idx = content.find(START_MARKER).unwrap();
                    let end_idx = content.find(END_MARKER).unwrap() + END_MARKER.len();
                    let mut updated = content.clone();
                    updated.replace_range(start_idx..end_idx, &new_section);
                    updated
                } else {
                    format!("{}\n\n{}", content, new_section)
                }
            }
            None => format!("# My Favorites\n\n{}\n", new_section),
        }
    }

    fn render_toc(tree: &BTreeMap<String, CategoryNode>, level: usize) -> String {
        let mut output = String::new();
        let indent = "  ".repeat(level);
        for (name, node) in tree {
            let anchor = name
                .to_lowercase()
                .replace(' ', "-")
                .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            output.push_str(&format!("{}- [{}](#{})\n", indent, name, anchor));
            output.push_str(&Self::render_toc(&node.subcategories, level + 1));
        }
        output
    }

    fn build_category_tree(favorites: &[Favorite]) -> BTreeMap<String, CategoryNode> {
        let mut root: BTreeMap<String, CategoryNode> = BTreeMap::new();
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
                let status = if fav.deprecated { " [DEPRECATED]" } else { "" };
                let icon_str = fav
                    .icon
                    .as_deref()
                    .map(|i| format!("{} ", i))
                    .unwrap_or_default();
                output.push_str(&format!(
                    "- {}[{}]({}){} - {}\n",
                    icon_str, fav.title, fav.url, status, fav.description
                ));
                if let Some(alt) = &fav.alternative {
                    output.push_str(&format!("  *Alternative: {}*\n", alt));
                }
                if !fav.tags.is_empty() {
                    let badges = fav
                        .tags
                        .iter()
                        .map(|t: &String| {
                            let (tag_name, color) = if t.contains(':') {
                                let mut parts = t.splitn(2, ':');
                                (parts.next().unwrap_or(t), parts.next().unwrap_or("brown"))
                            } else {
                                (t.as_str(), "brown")
                            };
                            let tag_clean = tag_name.replace('-', "--").replace(' ', "_");
                            format!(
                                "![{}](https://img.shields.io/badge/{}-{}?style=for-the-badge)",
                                tag_name, tag_clean, color
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    output.push_str(&format!("  {}\n", badges));
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
