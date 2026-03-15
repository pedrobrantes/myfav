# Development Guide

This project is a modular monolith structured as a Rust Workspace.

## Project Structure

- `crates/core`: Domain models (`Favorite`) and data persistence (`JsonRepository`).
- `crates/output`: Logic for formatting data into `Markdown` and `JSON`.
- `crates/cli`: Command-line interface logic, argument parsing, and Git orchestration.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable)
- [devenv](https://devenv.sh/) (optional, recommended for isolated environment)
- [Nix](https://nixos.org/) (optional, required for devenv)

### Environment Setup

Using `devenv`:

```bash
devenv shell
```

This will automatically set up Rust and all necessary dependencies.

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- <COMMAND>
```

### Formatting and Linting

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Maintenance Rules

1. **No Comments:** Code should be self-documenting. Use clear names.
2. **Modular Architecture:** Keep logic separated between `core`, `output`, and `cli`.
3. **Surgical Updates:** When updating `README.md`, ensure the markers `<!-- START_FAVORITES -->` and `<!-- END_FAVORITES -->` are respected.
4. **Git Integration:** Favor granular commits for data updates.

## CI/CD

We use GitHub Actions for:
- **CI:** Automated checks, tests, and formatting on every push.
- **Release:** Automated binary builds and distribution when a tag `v*` is pushed.
