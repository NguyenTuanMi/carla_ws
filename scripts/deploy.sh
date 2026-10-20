#!/bin/bash

# 1. Determine which env file to use
VEHICLE=$1
NAMESPACE="carla"
ENV_FILE=".env"

# 3. Load environment variables
source "$ENV_FILE"

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

# Panel 0 (top-left): foxglove
tmux send-keys -t $SESSION_NAME:0.0 "$DC_CMD up foxglove" C-m

# Panel 1 (top-right): g29 control
tmux send-keys -t $SESSION_NAME:0.1 "$DC_CMD up g29" C-m

# Panel 2 (bottom-left): rosbag record
tmux send-keys -t $SESSION_NAME:0.2 "$DC_CMD up rosbag_recorder" C-m

# Panel 3 (bottom-right): debug terminal
tmux send-keys -t $SESSION_NAME:0.3 "$DC_CMD up -d dev && $DC_CMD exec dev bash" C-m

tmux select-pane -t $SESSION_NAME:0.3
tmux attach-session -t $SESSION_NAME
exit 0

# --- FINALIZE ---
# Select the first window/pane before attaching
tmux select-window -t $SESSION_NAME:0
tmux select-pane -t $SESSION_NAME:0.0

# Attach to the session
tmux attach-session -t $SESSION_NAME