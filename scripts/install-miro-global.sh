#!/bin/zsh

set -euo pipefail

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
PROJECT_ROOT=$(cd "$SCRIPT_DIR/.." && pwd)
TARGET_BIN="$PROJECT_ROOT/target/release/miro"
INSTALL_DIR="${HOME}/bin"
WRAPPER_PATH="${INSTALL_DIR}/miro"

usage() {
  cat <<EOF
Usage: scripts/install-miro-global.sh [install|uninstall|status]

Commands:
  install    Create or update ~/bin/miro wrapper
  uninstall  Remove ~/bin/miro wrapper
  status     Show current global installation state
EOF
}

print_status() {
  if [[ -x "$WRAPPER_PATH" ]]; then
    echo "wrapper: installed at $WRAPPER_PATH"
  else
    echo "wrapper: not installed"
  fi

  if [[ -x "$TARGET_BIN" ]]; then
    echo "binary: present at $TARGET_BIN"
  else
    echo "binary: missing at $TARGET_BIN"
  fi

  if print -r -- "$PATH" | tr ':' '\n' | grep -Fxq "$INSTALL_DIR"; then
    echo "path: $INSTALL_DIR is available in PATH"
  else
    echo "path: $INSTALL_DIR is not in PATH"
  fi
}

install_wrapper() {
  mkdir -p "$INSTALL_DIR"

  cat > "$WRAPPER_PATH" <<EOF
#!/bin/zsh
set -euo pipefail

TARGET_BIN="$TARGET_BIN"

if [[ ! -x "\$TARGET_BIN" ]]; then
  echo "miro: release binary not found at \$TARGET_BIN" >&2
  echo "miro: run 'cargo build --release' in $PROJECT_ROOT and try again" >&2
  exit 1
fi

exec "\$TARGET_BIN" "\$@"
EOF

  chmod +x "$WRAPPER_PATH"
  echo "installed: $WRAPPER_PATH"
}

uninstall_wrapper() {
  if [[ -e "$WRAPPER_PATH" ]]; then
    rm -f "$WRAPPER_PATH"
    echo "removed: $WRAPPER_PATH"
  else
    echo "wrapper already absent: $WRAPPER_PATH"
  fi
}

COMMAND="${1:-install}"

case "$COMMAND" in
  install)
    install_wrapper
    print_status
    ;;
  uninstall)
    uninstall_wrapper
    ;;
  status)
    print_status
    ;;
  -h|--help|help)
    usage
    ;;
  *)
    usage >&2
    exit 1
    ;;
esac
