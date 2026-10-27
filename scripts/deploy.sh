#!/bin/bash

# 1. Determine which env file to use
VEHICLE=$1
NAMESPACE="carla"
ENV_FILE=".env"
LEADERBOARD_DIR=~/leaderboard

# 3. Load environment variables
source "$ENV_FILE"

G29_EVENT_LINK=$(ls /dev/input/by-id/*Logitech*G29*event-joystick 2>/dev/null | head -n 1)

if [ -n "$G29_EVENT_LINK" ]; then
    export G29_EVENT_DEVICE=$(readlink -f "$G29_EVENT_LINK")
else
    echo "[ERROR] Logitech G29 controller not detected! Stopping deployment."
    exit 0
fi

G29_JS_LINK=$(ls /dev/input/by-id/*Logitech*G29* 2>/dev/null | grep -v 'event-joystick' | grep 'joystick' | head -n 1)

if [ -n "$G29_JS_LINK" ]; then
    export G29_JS_DEVICE=$(readlink -f "$G29_JS_LINK")
else
    echo "[ERROR] Logitech G29 controller not detected! Stopping deployment."
    exit 0
fi

echo "[INFO] Successfully detected G29 Controller:"
echo "  - JS Device:    $G29_JS_DEVICE"
echo "  - Event Device: $G29_EVENT_DEVICE"

# Configuration
# Unique session name per vehicle to prevent overwriting
SESSION_NAME="${NAMESPACE}_window"

# Base docker compose command with environment file and unique project name
DC_CMD="docker compose"

# Kill existing session for THIS specific vehicle if it exists
tmux kill-session -t $SESSION_NAME 2>/dev/null

# Create new tmux session with first window
tmux new-session -d -s $SESSION_NAME -n "main"

# 2x2 grid: split into top/bottom, then split each half in two.
tmux split-window -v -t $SESSION_NAME:0
tmux split-window -h -t $SESSION_NAME:0.0
tmux split-window -h -t $SESSION_NAME:0.2

# Panel 0 (top-left): leaderboard
tmux send-keys -t $SESSION_NAME:0.0 "cd $LEADERBOARD_DIR" C-m
tmux send-keys -t $SESSION_NAME:0.0 "conda activate carla_0.9.16" C-m
tmux send-keys -t $SESSION_NAME:0.0 "./run_leaderboard.sh" C-m

# Panel 1 (top-right): g29 control
tmux send-keys -t $SESSION_NAME:0.1 "$DC_CMD up g29" C-m

# Panel 2 (bottom-left): carla_monitoring
tmux send-keys -t $SESSION_NAME:0.2 "$DC_CMD up carla_monitoring" C-m

# Panel 3 (bottom-right): carla_extensions
tmux send-keys -t $SESSION_NAME:0.3 "$DC_CMD up carla_extensions" C-m

# Create new tmux session with first window
tmux new-window -t $SESSION_NAME -n "debug"

# 2x2 grid: split into top/bottom, then split each half in two.
tmux split-window -v -t $SESSION_NAME:1
tmux split-window -h -t $SESSION_NAME:1.0
tmux split-window -h -t $SESSION_NAME:1.2

# Panel 0 (top-left): foxglove
tmux send-keys -t $SESSION_NAME:1.0 "$DC_CMD up foxglove" C-m

# Panel 1 (top-right): rosbag recorder
tmux send-keys -t $SESSION_NAME:1.1 "$DC_CMD up rosbag_recorder" C-m

# Panel 2 (bottom-left): debug terminal
tmux send-keys -t $SESSION_NAME:1.2 "$DC_CMD up -d dev && $DC_CMD exec dev bash" C-m

# Panel 3 (bottom-right): debug terminal
tmux send-keys -t $SESSION_NAME:1.3 "sleep 2 && $DC_CMD exec dev bash" C-m

# --- FINALIZE ---
# Select the first window/pane before attaching
tmux select-window -t $SESSION_NAME:0
tmux select-pane -t $SESSION_NAME:0.0

# Attach to the session
tmux attach-session -t $SESSION_NAME