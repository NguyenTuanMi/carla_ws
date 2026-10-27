<h3 align="center">The ROS2 workspace for CARLA 0.9.16</h3>
<p align="center">
  <em>Putting together the latest and greatest packages for development and deployment of CARLA autonomous vehicles</em>
</p>

## Table of Contents

- [Docker containerization](#docker-containerization)
- [Installation](#installation)
  - [1. Docker installation](#1-docker-installation)
  - [2. Clone and setup](#2-clone-and-setup)
  - [3. Build Docker images](#3-build-docker-images)
  - [4. Development and deployment with docker services](#4-development-and-deployment-with-docker-services)

## Summary

This workspace is for developing and deploying carla's software stacks using CARLA 0.9.16, SCENARIO 0.9.16, and an updated version of LEADERBOARD 2.1. It contains control packages, Docker configurations, and documentation. Core bringup and perception bringup packages are on the way to make development/deployment even easier. 

## Docker Containerization

This codebase uses `ROS2 Humble + Ubuntu 22.04` for all developments/deployments and they are all dockerized!

## Installation

### 1. Docker installation

**Install docker natively using `apt` instead of Docker Desktop**. It makes the setup and development as identical to a Linux environment and a Jetson device as possible.

<details>
<summary><b>For Jetson</b></summary>

There is no need to install anything, Docker comes pre-installed on Jetson devices. Just make sure to run `sudo usermod -aG docker $USER` if you haven't done it already.

</details>

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

### 3. Build Docker images

Please use `docker compose` to build and run the containers.

```bash
# Build the docker image
docker compose build 

# Check existing images
docker images
```

### 4. Development and deployment with docker services

First, you need to build the workspace. I have provided some shortcuts to help you build packages inside the appropriate containers without having to manually spin up a container and then `colcon build` inside the container. These shortcuts are defined in the [alias.sh](/scripts/alias.sh) script. To use these shortcuts, simply run the following commands in your terminal:

```bash
# IMPORTANT: RUN LOCALLY, NOT INSIDE ANY CONTAINER
# Append alias shortcuts to ~/.bashrc
./scripts/alias.sh
source ~/.bashrc

# Compile all current packages and all its dependencies (Probably need to change in the future when perception packages come in)
build-core

# Up the system
up-carla

# Down the system (However, currently, leaderboard must be killed manually inside the tmux pane because it is not incorporated into the docker services yet)
down-carla

# Before up the system, please up the CARLA Simulation by navigating to the folder containing it and run with nvidia + ros2 native:
./CarlaUE4.sh -prefernvidia --ros2
```
