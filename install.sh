#!/usr/bin/env zsh
set -e

BINARY_NAME="mechajira"
INSTALL_DIR="$HOME/.local/bin"
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=== mechajira installer ==="

# ── 1. Ensure Rust is installed ──────────────────────────────────────────────
if ! command -v cargo &>/dev/null; then
  echo "Rust not found. Installing via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
  source "$HOME/.cargo/env"
fi

if ! command -v cargo &>/dev/null; then
  source "$HOME/.cargo/env" 2>/dev/null || true
fi

CARGO="$(command -v cargo || echo "$HOME/.cargo/bin/cargo")"
echo "Using cargo: $CARGO"

# ── 2. Build release binary ───────────────────────────────────────────────────
echo "Building $BINARY_NAME..."
cd "$PROJECT_DIR"
"$CARGO" build --release

# ── 3. Install to ~/.local/bin ────────────────────────────────────────────────
mkdir -p "$INSTALL_DIR"
cp "target/release/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo ""
echo "✓ Installed to $INSTALL_DIR/$BINARY_NAME"

# ── 4. Verify PATH ────────────────────────────────────────────────────────────
if echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
  echo "✓ $INSTALL_DIR is in your PATH"
else
  echo ""
  echo "⚠ $INSTALL_DIR is not in your PATH."
  echo "  Add this to your ~/.zshrc:"
  echo ""
  echo '    export PATH="$HOME/.local/bin:$PATH"'
  echo ""
  echo "  Then run: source ~/.zshrc"
fi

# ── 5. Install skill files ────────────────────────────────────────────────────
SKILLS_DIR="$HOME/.local/share/mechajira/skills"
mkdir -p "$SKILLS_DIR"
cp -r "$PROJECT_DIR/.claude/skills/." "$SKILLS_DIR/"
echo "✓ Skills installed to $SKILLS_DIR"

echo ""
echo "Run 'mechajira --setup' to configure your Jira credentials."
echo "Run 'mechajira --install-skills' inside any repo to add the Claude Code skills."
