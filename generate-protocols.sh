#!/bin/bash

# Output JSON file
OUTPUT_FILE="gen/src/protocols.json"

# Check if jq is available
if ! command -v jq &>/dev/null; then
    echo "Error: jq is required but not installed."
    echo "Install with:"
    echo "  Ubuntu/Debian: sudo apt install jq"
    echo "  Arch Linux:    sudo pacman -S jq"
    echo "  macOS:         brew install jq"
    exit 1
fi

TEMP_JSON=$(mktemp)

# Initialize empty JSON object
echo '{}' >"$TEMP_JSON"

declare -a ordered_keys=("core" "stable" "staging" "unstable" "wlr" "plasma" "weston" "cosmic" "frog" "ivi" "hyprland")
declare -A folders=(
    ["core"]="protocols/wayland/protocol"
    ["stable"]="protocols/wayland-protocols/stable"
    ["staging"]="protocols/wayland-protocols/staging"
    ["unstable"]="protocols/wayland-protocols/unstable"
    ["wlr"]="protocols/wlr-protocols/unstable"
    ["plasma"]="protocols/plasma-wayland-protocols/src/protocols"
    ["weston"]="protocols/weston/protocol"
    ["cosmic"]="protocols/cosmic-protocols/unstable"
    ["frog"]="protocols/frog-protcols/frog-protocols"
    ["ivi"]="protocols/wayland-ivi-extension/protocol"
    ["hyprland"]="protocols/hyprland-protocols/protocols"
)
declare -a excluded_files=(
    "tests.xml"
    "cosmic-image-source-unstable-v1.xml"
    "cosmic-workspace-unstable-v1.xml"
    "cosmic-workspace-unstable-v2.xml"
)

# Process each folder in the specified order
for key in "${ordered_keys[@]}"; do
    folder="${folders[$key]}"
    echo "Processing $folder -> $key"

    if [[ -d "$folder" ]]; then
        # Get XML files, filter out excluded ones, and create JSON array
        xml_files=$(find "$folder" -name "*.xml" -type f | sort | while read -r file; do
            filename=$(basename "$file")
            excluded=false
            for excluded_file in "${excluded_files[@]}"; do
                if [[ "$filename" == "$excluded_file" ]]; then
                    excluded=true
                    echo "  Excluding: $file" >&2
                    break
                fi
            done
            if [[ "$excluded" == false ]]; then
                echo "$file"
            fi
        done | jq -R . | jq -s .)

        jq --argjson files "$xml_files" --arg key "$key" '. + {($key): $files}' "$TEMP_JSON" >"${TEMP_JSON}.tmp"
        mv "${TEMP_JSON}.tmp" "$TEMP_JSON"
    else
        echo "Warning: Directory $folder not found"
        # Add empty array for missing directories
        jq --arg key "$key" '. + {($key): []}' "$TEMP_JSON" >"${TEMP_JSON}.tmp"
        mv "${TEMP_JSON}.tmp" "$TEMP_JSON"
    fi
done

# Move final result to output file
mv "$TEMP_JSON" "$OUTPUT_FILE"

echo "JSON file created: $OUTPUT_FILE"
echo "Done!"
