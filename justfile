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

# Run the game in development mode (milestone M0, step 2).
run:
    @echo "the game crate arrives in M0 step 2" && exit 1

# Gamepad diagnostic screen (milestone M0, step 2).
labo-manette:
    @echo "the gamepad lab arrives in M0 step 2" && exit 1

# Render a reference scene to target/captures/<scene>.png (milestone M0, step 2).
capture scene:
    @echo "capture mode arrives in M0 step 2 (scene: {{scene}})" && exit 1
