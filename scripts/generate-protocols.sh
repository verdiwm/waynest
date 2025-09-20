#!/bin/bash

# Output JSON file
OUTPUT_FILE="crates/gen/src/protocols.json"

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

declare -a ordered_keys=("core" "stable" "staging" "unstable" "wlr" "plasma" "weston" "cosmic" "frog" "ivi" "hyprland" "mesa" "treeland")
declare -A folders=(
    ["core"]="wayland/protocol"
    ["stable"]="wayland-protocols/stable"
    ["staging"]="wayland-protocols/staging"
    ["unstable"]="wayland-protocols/unstable"
    ["wlr"]="wlr-protocols/unstable"
    ["plasma"]="plasma-wayland-protocols/src/protocols"
    ["weston"]="weston/protocol"
    ["cosmic"]="cosmic-protocols/unstable"
    ["frog"]="frog-protcols/frog-protocols"
    ["ivi"]="wayland-ivi-extension/protocol"
    ["hyprland"]="hyprland-protocols/protocols"
    ["mesa"]="mesa/src/egl/wayland/wayland-drm"
    ["treeland"]="treeland-protocols/xml"
)
declare -a excluded_files=(
    "tests.xml"
    "cosmic-image-source-unstable-v1.xml"
    "cosmic-workspace-unstable-v1.xml"
    "treeland-personalization-manager-v1.xml"
    "treeland-capture-unstable-v1.xml" # FIXME: I think this might be the generator fault
)

# Process each folder in the specified order
for key in "${ordered_keys[@]}"; do
    folder="external/protocols/${folders[$key]}"
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
