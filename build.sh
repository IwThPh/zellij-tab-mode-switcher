echo "🛠   Building Zellij tab-mode-switcher..."

ZELLIJ_PLUGIN_SRC="$HOME/bin/zellij/tab-mode-switcher"
ZELLIJ_PLUGIN_DEST="$HOME/.config/zellij/plugins"
ZELLIJ_PLUGIN_BIN="tab-mode-switcher.wasm"

mkdir -p "$ZELLIJ_PLUGIN_DEST"
rm "$ZELLIJ_PLUGIN_DEST/$ZELLIJ_PLUGIN_BIN" || true
cd "$ZELLIJ_PLUGIN_SRC" || exit

cargo build --release
cp "target/wasm32-wasi/release/$ZELLIJ_PLUGIN_BIN" "$ZELLIJ_PLUGIN_DEST/"

echo "🛠   Building Zellij tab-mode-switcher... done."
