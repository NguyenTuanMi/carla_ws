#!/bin/bash

declare -A ALIASES=(
    ["build-core"]="docker compose run --rm dev bash -c \"source /opt/ros/humble/setup.bash && colcon build --symlink-install\""
    ["up-carla"]="(cd ~/carla_ws && ./scripts/deploy.sh)"
    ["down-carla"]="docker compose down"
)

BASHRC="$HOME/.bashrc"

# Ensure the file ends with a newline character before we append anything
if [ -n "$(tail -c 1 "$BASHRC")" ]; then
    echo "" >> "$BASHRC"
fi

# Loop through the list: add missing aliases, update ones whose command changed
for name in "${!ALIASES[@]}"; do
    cmd="${ALIASES[$name]}"
    
    if grep -qxF "alias $name='$cmd'" "$BASHRC"; then
        echo "Alias '$name' is up to date. Skipping."
    elif grep -q "^alias $name=" "$BASHRC"; then
        # awk reads the new line from the environment, so quotes pass through as-is
        LINE="alias $name='$cmd'" awk -v p="alias $name=" \
            'index($0, p) == 1 { $0 = ENVIRON["LINE"] } { print }' "$BASHRC" > "$BASHRC.tmp" \
            && cat "$BASHRC.tmp" > "$BASHRC" && rm "$BASHRC.tmp"
        echo "Updated alias: $name -> $cmd"
    else
        echo "alias $name='$cmd'" >> "$BASHRC"
        echo "Added alias: $name -> $cmd"
    fi
done

echo ""
echo "Done! Run 'source ~/.bashrc' to apply the changes."