#!/bin/bash

# Create output directory if it doesn't exist
mkdir -p "/Users/kerongonzales/code/moodoc/mac_app_icons.iconset"

# macOS icon sizes needed
# Format: icon_SIZExSIZE.png and icon_SIZExSIZE@2x.png
sizes=(16 32 64 128 256 512 1024)

for size in "${sizes[@]}"; do
  # Regular resolution (@1x)
  if [ $size -lt 1024 ]; then
    convert -background none -resize ${size}x${size} "/Users/kerongonzales/code/moodoc/moodle_docker_icon.svg" "/Users/kerongonzales/code/moodoc/mac_app_icons.iconset/icon_${size}x${size}.png"
  fi
  
  # High resolution (@2x) - for sizes that have 2x versions
  if [ $size -lt 512 ]; then
    double=$((size * 2))
    convert -background none -resize ${double}x${double} "/Users/kerongonzales/code/moodoc/moodle_docker_icon.svg" "/Users/kerongonzales/code/moodoc/mac_app_icons.iconset/icon_${size}x${size}@2x.png"
  fi
done

# Create icns file from the iconset
iconutil -c icns "/Users/kerongonzales/code/moodoc/mac_app_icons.iconset" -o "/Users/kerongonzales/code/moodoc/moodle_docker.icns"

# Also create a high-resolution png for direct use
convert -background none "/Users/kerongonzales/code/moodoc/moodle_docker_icon.svg" -resize 1024x1024 "/Users/kerongonzales/code/moodoc/moodle_docker_icon_1024.png"

echo "Icon generation complete!"
echo "→ Individual sizes saved in mac_app_icons.iconset/"
echo "→ macOS .icns file saved as moodle_docker.icns"
echo "→ High-resolution PNG saved as moodle_docker_icon_1024.png"
