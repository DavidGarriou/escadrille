#!/usr/bin/env bash
# SessionStart hook: prepare a Claude Code cloud container (Ubuntu) so that
# `just check` and `just test` can run. Does nothing on a local machine
# (Fedora setup is described in README.md). Idempotent and non-interactive.
set -euo pipefail

[[ "${CLAUDE_CODE_REMOTE:-}" == "true" ]] || exit 0

# Pinned tool versions: bump them deliberately, like any other dependency.
NEXTEST_VERSION="0.9.148"
DENY_VERSION="0.20.2"

# System libraries needed to build Bevy on Linux, plus build tools.
APT_PACKAGES=(
  pkg-config clang mold just
  libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev libx11-dev
  # Software rendering for `just capture` without a GPU: Mesa lavapipe + virtual X server.
  mesa-vulkan-drivers xvfb xauth libxkbcommon-x11-0 libxcursor1 libxi6 libxrandr2
)

log() { echo "[session-start] $*" >&2; }

sudo_if_needed() {
  if [[ "$(id -u)" -eq 0 ]]; then "$@"; else sudo "$@"; fi
}

missing=()
for pkg in "${APT_PACKAGES[@]}"; do
  dpkg -s "$pkg" >/dev/null 2>&1 || missing+=("$pkg")
done
if ((${#missing[@]})); then
  log "installing apt packages: ${missing[*]}"
  sudo_if_needed env DEBIAN_FRONTEND=noninteractive apt-get update -qq
  sudo_if_needed env DEBIAN_FRONTEND=noninteractive \
    apt-get install -y -qq --no-install-recommends "${missing[@]}" >/dev/null
fi

bin_dir="${CARGO_HOME:-$HOME/.cargo}/bin"
mkdir -p "$bin_dir"

# install_release <binary> <wanted version> <tarball url>
install_release() {
  local name="$1" version="$2" url="$3"
  if "$bin_dir/$name" --version 2>/dev/null | grep -qF "$version"; then
    return 0
  fi
  log "installing $name $version"
  local tmp
  tmp="$(mktemp -d)"
  curl -fsSL --retry 3 "$url" | tar -xz -C "$tmp"
  find "$tmp" -type f -name "$name" -exec install -m 0755 {} "$bin_dir/$name" \;
  rm -rf "$tmp"
}

install_release cargo-nextest "$NEXTEST_VERSION" \
  "https://github.com/nextest-rs/nextest/releases/download/cargo-nextest-${NEXTEST_VERSION}/cargo-nextest-${NEXTEST_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
install_release cargo-deny "$DENY_VERSION" \
  "https://github.com/EmbarkStudios/cargo-deny/releases/download/${DENY_VERSION}/cargo-deny-${DENY_VERSION}-x86_64-unknown-linux-musl.tar.gz"

# Toolchain pinned by rust-toolchain.toml (ADR 0004); the container ships an older one.
if command -v rustup >/dev/null 2>&1; then
  (cd "${CLAUDE_PROJECT_DIR:-.}" && rustup toolchain install >/dev/null 2>&1) ||
    log "rustup toolchain install failed"
fi

log "ready: $(just --version), $(cargo nextest --version | head -1), $(cargo deny --version)"
