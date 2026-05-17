#!/usr/bin/env bash
# post_build.sh — Fix asset references in index.html after Dioxus build
# Dioxus hashes asset filenames but doesn't update index.html references.
# This script finds hashed assets and injects them into the HTML.

set -euo pipefail

BUILD_DIR="${1:-target/dx/velvet-ui/release/web/public}"
INDEX_HTML="$BUILD_DIR/index.html"

if [ ! -f "$INDEX_HTML" ]; then
    echo "[ERROR] index.html not found in $BUILD_DIR"
    exit 1
fi

echo "[INFO] Fixing asset references in index.html..."

# Find hashed CSS file
CSS_FILE=$(ls -1 "$BUILD_DIR/assets/theme-"*.css 2>/dev/null | head -1 || true)
if [ -n "$CSS_FILE" ]; then
    CSS_BASENAME=$(basename "$CSS_FILE")
    CSS_PATH="assets/$CSS_BASENAME"
    # Check if CSS link already exists
    if ! grep -q 'rel="stylesheet"' "$INDEX_HTML"; then
        # Inject CSS link before </head>
        sed -i '' "s|</head>|<link rel=\"stylesheet\" href=\"$CSS_PATH\"></head>|" "$INDEX_HTML"
        echo "[OK] Injected CSS: $CSS_PATH"
    else
        # Update existing CSS reference
        sed -i '' "s|href=\"assets/theme.css\"|href=\"$CSS_PATH\"|" "$INDEX_HTML"
        echo "[OK] Updated CSS reference: $CSS_PATH"
    fi
else
    echo "[WARN] No hashed CSS file found"
fi

# Find hashed JS file (Dioxus usually handles this correctly)
JS_FILE=$(ls -1 "$BUILD_DIR/assets/velvet-ui-"*.js 2>/dev/null | head -1 || true)
if [ -n "$JS_FILE" ]; then
    JS_BASENAME=$(basename "$JS_FILE")
    JS_PATH="assets/$JS_BASENAME"
    # Update JS reference if needed
    if grep -q 'velvet-ui-dxh' "$INDEX_HTML"; then
        echo "[OK] JS reference already correct"
    else
        sed -i '' "s|velvet-ui.js|$JS_PATH|g" "$INDEX_HTML"
        echo "[OK] Updated JS reference: $JS_PATH"
    fi
fi

# Find hashed WASM file
WASM_FILE=$(ls -1 "$BUILD_DIR/assets/velvet-ui_bg-"*.wasm 2>/dev/null | head -1 || true)
if [ -n "$WASM_FILE" ]; then
    WASM_BASENAME=$(basename "$WASM_FILE")
    WASM_PATH="assets/$WASM_BASENAME"
    WASM_SIZE=$(du -h "$WASM_FILE" | cut -f1)
    echo "[OK] WASM: $WASM_PATH ($WASM_SIZE)"
fi

echo "[OK] Asset references fixed"
