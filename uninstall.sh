#!/bin/sh
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
RESET='\033[0m'

info() { printf "${CYAN}→${RESET} %s\n" "$1"; }
ok() { printf "${GREEN}✓${RESET} %s\n" "$1"; }

echo ""
echo "  Uninstalling awsx..."
echo ""

# Remove binary
for path in /usr/local/bin/awsx "$HOME/.cargo/bin/awsx"; do
  if [ -f "$path" ]; then
    if [ -w "$path" ]; then
      rm "$path"
    else
      sudo rm "$path"
    fi
    ok "Removed binary: $path"
  fi
done

# Remove config
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
case "$OS" in
  darwin) CONFIG_DIR="$HOME/Library/Application Support/awsx" ;;
  *)      CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/awsx" ;;
esac

if [ -d "$CONFIG_DIR" ]; then
  rm -rf "$CONFIG_DIR"
  ok "Removed config: $CONFIG_DIR"
fi

# Clean shell hooks
for rc in "$HOME/.zshrc" "$HOME/.zshrc.local" "$HOME/.bashrc" "$HOME/.config/fish/config.fish"; do
  if [ -f "$rc" ] && grep -q "awsx" "$rc" 2>/dev/null; then
    # Remove awsx comment and eval lines
    if [ "$(uname -s)" = "Darwin" ]; then
      sed -i '' '/# AWS Context Switcher (awsx)/d;/awsx shell-hook/d' "$rc"
    else
      sed -i '/# AWS Context Switcher (awsx)/d;/awsx shell-hook/d' "$rc"
    fi
    ok "Cleaned shell hook from: $rc"
  fi
done

echo ""
echo "  🗑️  awsx uninstalled."
echo ""
echo "  Run this to complete cleanup in your current shell:"
echo ""
echo "    unset -f awsx 2>/dev/null; exec \$SHELL"
echo ""
