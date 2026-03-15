# Release Notes - v0.1.0

## Initial Release

Modular Rust CLI to manage and distribute your favorites/recommendations.

### Features
- **Modular Monolith Architecture**: Separated into `core`, `output`, and `myfav` (CLI) crates.
- **Hierarchical Categories**: Support for paths like `Android/Apps/Store`.
- **Surgical README Updates**: Automatic updates between `<!-- START_FAVORITES -->` and `<!-- END_FAVORITES -->` markers.
- **Table of Contents**: Automatically generated clickable index in Markdown.
- **Smart Git Integration**: Optional automated commits with conventional messages (`feat`, `chore`, `fix`).
- **Distribution Ready**: Generates `dist/favorites.json` for external consumption.
- **Minimalist Design**: Clean code, no emojis by default, professional Shields.io badges for tags.

### Installation
```bash
cargo install myfav
```
