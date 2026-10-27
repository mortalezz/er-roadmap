#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="${SCRIPT_DIR}/ER Roadmap GPU.app"
ICON_SRC="${SCRIPT_DIR}/../static/icon-512.png"

export PATH="${HOME}/.cargo/bin:${PATH}"

echo "==> Building release binary (er-roadmap-gpu)..."
cargo build --release --manifest-path "${SCRIPT_DIR}/Cargo.toml"

echo "==> Assembling macOS application bundle (${APP_DIR})..."
mkdir -p "${APP_DIR}/Contents/MacOS" "${APP_DIR}/Contents/Resources"
cp "${SCRIPT_DIR}/target/release/er-roadmap-gpu" "${APP_DIR}/Contents/MacOS/ER Roadmap GPU"
chmod +x "${APP_DIR}/Contents/MacOS/ER Roadmap GPU"

cat > "${APP_DIR}/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleDisplayName</key>
  <string>ER Roadmap GPU</string>
  <key>CFBundleExecutable</key>
  <string>ER Roadmap GPU</string>
  <key>CFBundleIconFile</key>
  <string>AppIcon</string>
  <key>CFBundleIdentifier</key>
  <string>dev.mortalezz.er-roadmap-gpu</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>ER Roadmap GPU</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>1.2.0</string>
  <key>CFBundleVersion</key>
  <string>1</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>NSRequiresAquaSystemAppearance</key>
  <false/>
</dict>
</plist>
EOF

if [[ -f "${ICON_SRC}" ]] && command -v sips >/dev/null 2>&1 && command -v iconutil >/dev/null 2>&1; then
  ICONSET_DIR="$(mktemp -d)/AppIcon.iconset"
  mkdir -p "${ICONSET_DIR}"
  for sz in 16 32 128 256 512; do
    sips -z "${sz}" "${sz}" "${ICON_SRC}" --out "${ICONSET_DIR}/icon_${sz}x${sz}.png" >/dev/null
    sz2=$((sz * 2))
    sips -z "${sz2}" "${sz2}" "${ICON_SRC}" --out "${ICONSET_DIR}/icon_${sz}x${sz}@2x.png" >/dev/null
  done
  iconutil -c icns "${ICONSET_DIR}" -o "${APP_DIR}/Contents/Resources/AppIcon.icns"
  rm -rf "$(dirname "${ICONSET_DIR}")"
fi

if command -v codesign >/dev/null 2>&1; then
  codesign -s - --force --deep "${APP_DIR}"
fi

echo "==> Built: ${APP_DIR}"
