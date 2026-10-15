#!/bin/bash
# Source ROS 2 (+ overlays) so every service inherits the environment, then run the command.
set -e
source /opt/ros/${ROS_DISTRO}/setup.bash
if [ -f "$HOME/carla_ws/install/setup.bash" ]; then source "$HOME/carla_ws/install/setup.bash"; fi
export PYTHONPATH=/opt/carla_src/PythonAPI/carla:/opt/scenario_runner${PYTHONPATH:+:$PYTHONPATH}
exec "$@"
