#!/bin/bash

# Script to generate macOS .icns file and .iconset directory from an SVG

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
    echo "Error: ImageMagick (convert command) not found. Please install it."
    echo "On macOS, you can install it using Homebrew: brew install imagemagick"
    exit 1
fi

# Check if iconutil is available (standard on macOS)
if ! command -v iconutil &> /dev/null; then
    echo "Error: iconutil command not found. This script is intended for macOS."
    exit 1
fi

# Input SVG file (first argument, default to moodle_docker_server_icon.svg)
SVG_INPUT="${1:-moodle_docker_server_icon.svg}" # Corrected default SVG name

# Output .icns file name (second argument, default to AppIcon.icns)
ICNS_OUTPUT="${2:-AppIcon.icns}"

# Iconset directory name (derived from the output ICNS name)
ICONSET_DIR="${ICNS_OUTPUT%.*}.iconset"

# High-resolution PNG output name (derived from the output ICNS name)
HIGH_RES_PNG_OUTPUT="${ICNS_OUTPUT%.*}_1024.png"

# Check if input SVG exists
if [ ! -f "$SVG_INPUT" ]; then
    echo "Error: Input SVG file '$SVG_INPUT' not found."
    exit 1
fi

# Create or clean the iconset directory
rm -rf "$ICONSET_DIR"
mkdir "$ICONSET_DIR"

echo "Generating icons in $ICONSET_DIR from $SVG_INPUT..."

# Sizes required for macOS .iconset
SIZES=(16 32 64 128 256 512)

# Generate standard and @2x Retina versions
for size in "${SIZES[@]}"; do
    echo "Generating icon_${size}x${size}.png"
    convert -background none -density 300 -resize "${size}x${size}" "$SVG_INPUT" "$ICONSET_DIR/icon_${size}x${size}.png"
    retina_size=$((size * 2))
    echo "Generating icon_${size}x${size}@2x.png (as ${retina_size}x${retina_size})"
    convert -background none -density 300 -resize "${retina_size}x${retina_size}" "$SVG_INPUT" "$ICONSET_DIR/icon_${size}x${size}@2x.png"
done

# Special case for 512x512@2x which is 1024x1024
# This will also serve as our high-resolution PNG
echo "Generating icon_512x512@2x.png (as 1024x1024)"
convert -background none -density 300 -resize 1024x1024 "$SVG_INPUT" "$ICONSET_DIR/icon_512x512@2x.png"

# Copy the 1024x1024 version for the high-res PNG output
cp "$ICONSET_DIR/icon_512x512@2x.png" "$HIGH_RES_PNG_OUTPUT"

# Convert the iconset to .icns
echo "Creating $ICNS_OUTPUT from $ICONSET_DIR..."
iconutil -c icns "$ICONSET_DIR"

if [ $? -eq 0 ]; then
    mv "${ICONSET_DIR%.iconset}.icns" "$ICNS_OUTPUT" # Ensure correct output name if iconutil defaults
    echo "Icon generation complete!"
    echo "→ Individual sizes saved in $ICONSET_DIR/"
    echo "→ macOS .icns file saved as $ICNS_OUTPUT"
    echo "→ High-resolution PNG saved as $HIGH_RES_PNG_OUTPUT"
else
    echo "Error: iconutil failed to create .icns file."
    exit 1
fi

exit 0
