#!/bin/sh
set -e

REPO="abdul-zailani/awsx"
INSTALL_DIR="/usr/local/bin"
BINARY="awsx"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
DIM='\033[2m'
RESET='\033[0m'

info() { printf "${CYAN}→${RESET} %s\n" "$1"; }
ok() { printf "${GREEN}✓${RESET} %s\n" "$1"; }
fail() { printf "${RED}✗${RESET} %s\n" "$1"; exit 1; }

# Detect OS and arch
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
  linux)  TARGET_OS="unknown-linux-gnu" ;;
  darwin) TARGET_OS="apple-darwin" ;;
  *)      fail "Unsupported OS: $OS" ;;
esac

case "$ARCH" in
  x86_64|amd64)  TARGET_ARCH="x86_64" ;;
  aarch64|arm64) TARGET_ARCH="aarch64" ;;
  *)             fail "Unsupported architecture: $ARCH" ;;
esac

TARGET="${TARGET_ARCH}-${TARGET_OS}"
info "Detected platform: ${TARGET}"

# Get latest release tag
info "Fetching latest release..."
if command -v curl >/dev/null 2>&1; then
  LATEST=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')
elif command -v wget >/dev/null 2>&1; then
  LATEST=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')
else
  fail "curl or wget required"
fi

if [ -z "$LATEST" ]; then
  # Fallback: build from source
  info "No release found, building from source..."
  if ! command -v cargo >/dev/null 2>&1; then
    info "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    . "$HOME/.cargo/env"
  fi
  cargo install --git "https://github.com/${REPO}"
  INSTALL_DIR="$HOME/.cargo/bin"
else
  info "Latest release: ${LATEST}"
  URL="https://github.com/${REPO}/releases/download/${LATEST}/${BINARY}-${TARGET}"

  # Download
  TMPDIR=$(mktemp -d)
  TMPFILE="${TMPDIR}/${BINARY}"
  info "Downloading ${BINARY}-${TARGET}..."

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$URL" -o "$TMPFILE" || fail "Download failed. Binary may not exist for ${TARGET}"
  else
    wget -q "$URL" -O "$TMPFILE" || fail "Download failed. Binary may not exist for ${TARGET}"
  fi

  chmod +x "$TMPFILE"

  # Install
  if [ -w "$INSTALL_DIR" ]; then
    mv "$TMPFILE" "${INSTALL_DIR}/${BINARY}"
  else
    info "Need sudo to install to ${INSTALL_DIR}"
    sudo mv "$TMPFILE" "${INSTALL_DIR}/${BINARY}"
  fi
  rm -rf "$TMPDIR"
fi

INSTALLED_PATH=$(command -v "$BINARY" 2>/dev/null || echo "${INSTALL_DIR}/${BINARY}")
ok "Installed: ${INSTALLED_PATH}"
printf "${DIM}%s${RESET}\n" "$($INSTALLED_PATH --version)"

# Detect shell and suggest hook
CURRENT_SHELL=$(basename "${SHELL:-/bin/sh}")
case "$CURRENT_SHELL" in
  zsh)  RC="$HOME/.zshrc";  HOOK="eval \"\$(${INSTALLED_PATH} shell-hook zsh --prompt)\"" ;;
  bash) RC="$HOME/.bashrc"; HOOK="eval \"\$(${INSTALLED_PATH} shell-hook bash --prompt)\"" ;;
  fish) RC="$HOME/.config/fish/config.fish"; HOOK="${INSTALLED_PATH} shell-hook fish --prompt | source" ;;
  *)    RC=""; HOOK="" ;;
esac

if [ -n "$RC" ] && [ -n "$HOOK" ]; then
  if [ -f "$RC" ] && grep -q "awsx shell-hook" "$RC" 2>/dev/null; then
    ok "Shell hook already in ${RC}"
  else
    printf "\n# AWS Context Switcher (awsx)\n%s\n" "$HOOK" >> "$RC"
    ok "Shell hook added to ${RC}"
  fi
fi

echo ""
echo "🚀 Setup complete! Run: source ${RC}"
echo ""
echo "   awsx init                                 # auto-discover profiles & contexts"
echo "   awsx use                                  # switch context (interactive)"
echo "   awsx list                                 # list saved contexts"

# Auto-init if aws or kubectl available
if command -v aws >/dev/null 2>&1 || command -v kubectl >/dev/null 2>&1; then
  echo ""
  info "Running awsx init to discover existing profiles and contexts..."
  echo ""
  "$INSTALLED_PATH" init
fi
