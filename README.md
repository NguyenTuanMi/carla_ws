<h3 align="center">The ROS2 workspace for CARLA 1.0/CARLA 0.9.16</h3>
<p align="center">
  <em>Putting together the packages for development and deployment of CARLA autonomous vehicles</em>
</p>

## Table of Contents

- [ROS 2 packages](#ros-2-packages)
- [Docker containerization](#docker-containerization)
- [Installation](#installation)
  - [1. Docker installation](#1-docker-installation)
  - [2. Clone and setup](#2-clone-and-setup)
  - [3. Build Docker images](#3-build-docker-images)
  - [4. Development and deployment with docker services](#4-development-and-deployment-with-docker-services)

## Summary

This workspace is for developing and deploying carla's software stacks using CARLA 1.0, SCENARIO 1.0, and an updated version of LEADERBOARD (on branch ue58-dev). There is also an additional branch `carla_0.9.16`, where it works with CARLA_0.9.16, SCENARIO_0.9.16, and an updated version of Leaderboard2.1. It contains control packages, Docker configurations, and documentation. Specifically, this workspace and leaderboard is highly customized and developed for scenario and route generation, with carla_ws's `main` branch and leaderboard's `ue58-dev` branch leverage Hermes Agent, while carla_ws's `carla_0.9.16` branch and leaderboard's `leaderboard2.1` branch run direct OpenAI inference.  

## ROS 2 Packages

The packages in `src/` provide CARLA message definitions, vehicle control, pose and camera utilities, and target monitoring.

| Package | Purpose | Main interfaces / entry points |
| --- | --- | --- |
| `carla_msgs` | Defines ROS 2 messages used by CARLA integrations, including vehicle control, vehicle state, actors, collisions, traffic lights, weather, and world information. | Message types such as `carla_msgs/msg/CarlaEgoVehicleControl`; build this package before packages that depend on its interfaces. |
| `carla_extensions` | Connects to the CARLA Python API, publishes the configured vehicle's map pose, and republishes configured camera images with opaque alpha for viewers that otherwise render transparent BGRA images. | `ros2 run carla_extensions extension_publisher`; publishes `/carla/<role_name>/odom` and `/carla/<role_name>/<view>/image_opaque`. Defaults to role `hero` and camera views `Center`, `Rearview`, `Right`, and `Left`. |
| `carla_monitoring` | Compares the hero pose against configured target coordinates and publishes a `SAFE` or `WARNING` status according to each target's warning radius. | `ros2 launch carla_monitoring monitoring.launch.py`; subscribes to `/carla/hero/odom`, publishes `/carla/hero/status`, and reads targets/radii from `config/carla_monitoring.yaml`. |
| `carla_teleop` | Provides keyboard control using W/A/S/D or arrow keys, with configurable acceleration, braking, steering, and key-repeat timeouts. | `ros2 launch carla_teleop teleop.launch.py`; publishes `carla_msgs/msg/CarlaEgoVehicleControl` to `/carla/<role_name>/vehicle_control_cmd` and autopilot toggles to `/carla/<role_name>/enable_autopilot`. Tuning is in `config/teleop.yaml`. |
| `g29_control` | Reads a Logitech G29 through `joy_node`, maps joystick axes/buttons to CARLA vehicle control, and optionally records the driven route to CSV. | `ros2 launch g29_control manual_recording.launch.py role_name:=hero`; reads `/joy`, publishes `/carla/<role_name>/vehicle_control_cmd`, and logs `/carla/<role_name>/odometry` to `route.csv` by default. Axis indices and output file are launch-configurable. |

`carla_teleop` and `g29_control` both publish vehicle commands to the same control topic for a role. Run only one manual controller at a time; if both are active, their commands can overwrite each other.

## Docker Containerization

This codebase uses `ROS2 Humble + Ubuntu 22.04` for all developments/deployments and they are all dockerized!

## Installation

### 1. Docker installation

**Install docker natively using `apt` instead of Docker Desktop**. It makes the setup and development as identical to a Linux environment as possible.

<details>
<summary><b>For Windows (WSL2) and Linux</b></summary>

1. [Uninstall any older Docker Engine](https://docs.docker.com/engine/install/ubuntu/#uninstall-docker-engine).
2. [Install Docker Engine](https://docs.docker.com/engine/install/ubuntu/).
3. `sudo usermod -aG docker $USER`
4. Close and open Ubuntu terminal again. Or reboot if needed.

</details>

<details>
<summary><b>For Mac</b></summary>

Not tested, seems like you need [Docker Desktop for Mac](https://docs.docker.com/desktop/setup/install/mac-install/).

</details>

### 2. Clone and setup

Clone this repo:
```bash
cd ~ && git clone --recursive https://github.com/NguyenTuanMi/carla_ws.git && cd carla_ws
chmod +x scripts/*
```

Pull dependencies including CARLA PythonAPI and compatible SCENARIO to `dependencies/`:
```bash
./scripts/fetch_dependencies.sh
```

Also, please download and extract the CARLA simulator from [CARLA's official release page](https://github.com/carla-simulator/carla/releases). Please pick either 0.9.16 or 1.0, depended on your intended use branch. 

### 3. Build Docker images

Please use `docker compose` to build and run the containers.

```bash
# Build the docker image
docker compose build 

# Check existing images
docker images
```

### 4. Development and deployment with docker services

First, you need to build the workspace. I have provided some shortcuts to help you build packages inside the appropriate containers without having to manually spin up a container and then `colcon build` inside the container. These shortcuts are defined in the [alias.sh](/scripts/alias.sh) script. To use these shortcuts, simply run the following commands in your terminal (**MUST BE** inside ~/carla_ws directory):

```bash
# IMPORTANT: RUN LOCALLY, NOT INSIDE ANY CONTAINER
# Append alias shortcuts to ~/.bashrc
./scripts/alias.sh
source ~/.bashrc

# Compile all current packages and all its dependencies (Probably need to change in the future when perception packages come in)
build-core

# Up the system (This command will spawn a tmux panel with 2 windows, 1 window is main execution, the second window is for debugging, including foxglove publishing)
up-carla

# Down the system (However, currently, leaderboard must be killed manually inside the tmux pane because it is not incorporated into the docker services yet)
down-carla

# Before up the system, please up the CARLA Simulation by navigating to the folder containing it and run with nvidia + ros2 native:
./.../.../CarlaUnreal.sh -prefernvidia --ros2
```
