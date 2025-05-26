#!/bin/bash
set -e

# Define paths
SVG_FILE="moodle_docker_server_icon_updated.svg"
PNG_FILE="icon_updated.png"
ICONSET_DIR="moodle_docker_updated.iconset"
ICNS_FILE="moodle_docker_updated.icns"

# Current directory
echo "Working in $(pwd)"

# Convert SVG to 1024x1024 PNG
echo "Converting SVG to PNG..."
if command -v rsvg-convert &> /dev/null; then
  rsvg-convert -w 1024 -h 1024 "$SVG_FILE" -o "$PNG_FILE"
elif command -v inkscape &> /dev/null; then
  inkscape "$SVG_FILE" -w 1024 -h 1024 --export-filename="$PNG_FILE"
elif command -v convert &> /dev/null; then
  convert "$SVG_FILE" -resize 1024x1024 "$PNG_FILE"
else
  echo "Error: No suitable SVG conversion tool found (rsvg-convert, inkscape, or convert)"
  exit 1
fi

# Create iconset directory
echo "Creating iconset directory..."
mkdir -p "$ICONSET_DIR"

# Generate various icon sizes
echo "Generating icon sizes..."
sips -z 16 16 "$PNG_FILE" --out "${ICONSET_DIR}/icon_16x16.png"
sips -z 32 32 "$PNG_FILE" --out "${ICONSET_DIR}/icon_16x16@2x.png"
sips -z 32 32 "$PNG_FILE" --out "${ICONSET_DIR}/icon_32x32.png"
sips -z 64 64 "$PNG_FILE" --out "${ICONSET_DIR}/icon_32x32@2x.png"
sips -z 64 64 "$PNG_FILE" --out "${ICONSET_DIR}/icon_64x64.png"
sips -z 128 128 "$PNG_FILE" --out "${ICONSET_DIR}/icon_64x64@2x.png"
sips -z 128 128 "$PNG_FILE" --out "${ICONSET_DIR}/icon_128x128.png"
sips -z 256 256 "$PNG_FILE" --out "${ICONSET_DIR}/icon_128x128@2x.png"
sips -z 256 256 "$PNG_FILE" --out "${ICONSET_DIR}/icon_256x256.png"
sips -z 512 512 "$PNG_FILE" --out "${ICONSET_DIR}/icon_256x256@2x.png"
sips -z 512 512 "$PNG_FILE" --out "${ICONSET_DIR}/icon_512x512.png"
sips -z 1024 1024 "$PNG_FILE" --out "${ICONSET_DIR}/icon_512x512@2x.png"

# Convert iconset to icns
echo "Converting iconset to icns..."
iconutil -c icns "$ICONSET_DIR" -o "$ICNS_FILE"

echo "Copying to Tauri icons directory..."
cp "$PNG_FILE" "moodle-docker-manager/src-tauri/icons/icon.png"
cp "$ICNS_FILE" "moodle-docker-manager/src-tauri/icons/icon.icns"

echo "Done! Created:"
echo "- $PNG_FILE (1024x1024 PNG)"
echo "- $ICONSET_DIR/ (Multiple icon sizes)"
echo "- $ICNS_FILE (macOS icon file)"
echo "- Copied to Tauri icons directory"
