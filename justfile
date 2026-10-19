# Panache blanc — project commands (see CLAUDE.md, "Commandes").

set shell := ["bash", "-euo", "pipefail", "-c"]

# List the recipes.
default:
    @just --list

# Format check, clippy (warnings are errors) and cargo-deny.
check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo deny check

# Run all tests with nextest.
test:
    cargo nextest run --workspace

# Format the code.
fmt:
    cargo fmt --all

# Optimised build.
release:
    cargo build --workspace --release

# Headless flight tests of an aircraft (milestone M1).
essai avion:
    cargo run -p fdm_lab -- essai {{avion}}

# Run the game in development mode (dynamic linking for faster rebuilds).
run:
    cargo run -p escadrille --features dev

# Gamepad diagnostic screen; writes target/labo-manette/rapport.txt on request.
labo-manette:
    cargo run -p escadrille --features dev -- --labo-manette

# Render a reference scene to target/captures/<scene>.png (+ .json), then quit.
capture scene *args:
    cargo run -p escadrille --release -- --capture {{scene}} {{args}}
