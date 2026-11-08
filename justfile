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

# Run the game in development mode (dynamic linking for faster rebuilds): the test flight
# on the field, or a reference scene flown with the gamepad (`just run --scene traversee_nuage`).
run *args:
    cargo run -p escadrille --features dev -- {{args}}

# Gamepad diagnostic screen; writes target/labo-manette/rapport.txt on request.
labo-manette:
    cargo run -p escadrille --features dev -- --labo-manette

# Render a reference scene to target/captures/<scene>.png (+ .json), then quit; the
# previous one is kept as <scene>.avant.png (and .json) for the comparison. A scene with a
# sequence also saves <scene>-<frame>.png, kept likewise as <scene>-<frame>.avant.png.
# Without a display (cloud sessions), runs under Xvfb with software rendering (lavapipe).
capture scene *args:
    cargo build -p escadrille --release
    for ext in png json; do \
        if [ -f target/captures/{{scene}}.$ext ]; then cp target/captures/{{scene}}.$ext target/captures/{{scene}}.avant.$ext; fi; \
    done
    for shot in target/captures/{{scene}}-[0-9][0-9][0-9][0-9].png; do \
        if [ -f "$shot" ]; then cp "$shot" "${shot%.png}.avant.png"; fi; \
    done
    if [ -z "${DISPLAY:-}" ] && [ -z "${WAYLAND_DISPLAY:-}" ]; then \
        xvfb-run -a -s "-screen 0 2560x1440x24" target/release/escadrille --capture {{scene}} {{args}}; \
    else \
        target/release/escadrille --capture {{scene}} {{args}}; \
    fi
