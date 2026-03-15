# fav

A modular Rust CLI to manage and distribute your favorites/recommendations.

`fav` allows you to maintain a "source of truth" for your recommendations in a structured JSON format and automatically distribute them to multiple outputs, such as a hierarchical `README.md` and a consolidated distribution JSON for external consumption (e.g., blogs, mobile apps).

## Features

- **Hierarchical Categories:** Add favorites using flexible paths (e.g., `Android/Apps/Store`).
- **Surgical Updates:** Automatically updates your `README.md` between markers, preserving manual edits.
- **Smart Git Integration:** Optional automated commits with descriptive messages (`feat`, `chore`, `fix`).
- **Distribution Ready:** Generates a clean JSON file ready for your frontend.
- **Minimalist:** Clean code, no emojis by default, focused on maintenance.

## Installation

### Using Cargo

```bash
cargo install --path crates/cli
```

### Using Nix

```bash
nix profile add github:pedrobrantes/myfav
```

## Usage

### Add a favorite

```bash
myfav add -t "Obtainium" -d "App manager for GitHub" -u "https://github..." -p "Android/Apps" -T "store,open-source"
```

### Move a favorite

```bash
myfav mv "Obtainium" -p "Android/Archive"
```

### Remove a favorite

```bash
myfav rm "Obtainium"
```

### Sync outputs (Manual)

```bash
myfav sync
```

### Sync with Git Commit

```bash
myfav --git add ...
```

## Options

```text
Usage: myfav [OPTIONS] <COMMAND>

Commands:
  add   Add or Update a favorite
  rm    Remove a favorite
  mv    Move a favorite to a new category path
  list  List all favorites
  sync  Sync and generate output files
  help  Print this message or the help of the given subcommand(s)

Options:
  -D, --data <DATA>      [default: data/favorites.json]
  -R, --readme <README>  [default: README.md]
  -O, --dist <DIST>      [default: dist/favorites.json]
      --git              Enable automatic git commits
  -h, --help             Print help
  -V, --version          Print version
```

## License

This project is licensed under the MIT license.
