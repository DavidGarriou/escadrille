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

# Provisional 3D model of an aircraft: assets/aircraft/<avion>/model.glb (boxes and plates).
mockup avion:
    cargo run -p mockup -- {{avion}}

# Modelled 3D model of an aircraft, built by Blender from its script in art/ (ADR 0007).
# Writes target/modele/<avion>.glb and .blend and checks it; `just modele <avion> assets`
# then replaces assets/aircraft/<avion>/model.glb.
modele avion dest="target":
    mkdir -p target/modele
    cargo run -q -p mockup -- {{avion}} --json target/modele/{{avion}}.json
    blender -b --factory-startup --python art/{{avion}}/build.py -- target/modele/{{avion}}.json target/modele/{{avion}}.glb target/modele/{{avion}}.blend
    cargo run -q -p asset_check -- {{avion}} --model target/modele/{{avion}}.glb
    if [ "{{dest}}" = "assets" ]; then cp target/modele/{{avion}}.glb assets/aircraft/{{avion}}/model.glb; fi

# Run the game in development mode (dynamic linking for faster rebuilds).
run:
    cargo run -p escadrille --features dev

# Gamepad diagnostic screen; writes target/labo-manette/rapport.txt on request.
labo-manette:
    cargo run -p escadrille --features dev -- --labo-manette

# Render a reference scene to target/captures/<scene>.png (+ .json), then quit.
# Without a display (cloud sessions), runs under Xvfb with software rendering (lavapipe).
capture scene *args:
    cargo build -p escadrille --release
    if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then \
        xvfb-run -a -s "-screen 0 2560x1440x24" target/release/escadrille --capture {{scene}} {{args}}; \
    else \
        target/release/escadrille --capture {{scene}} {{args}}; \
    fi
