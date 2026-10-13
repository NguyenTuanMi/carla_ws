FROM ubuntu:22.04

ENV ROS_DISTRO=humble
ENV DEBIAN_FRONTEND=noninteractive

# Optional: pin ROS packages to a snapshot date.
# Leave empty to use the normal packages.ros.org repo. If set, use a date that exists at
# https://snapshots.ros.org/humble/ . Keep it identical in every image that shares a workspace.
ARG ROS_SNAPSHOT=""

# carla wheel version on PyPI — must match the server binary
ARG CARLA_PIP_VERSION=0.9.16

# --- System packages + ROS 2 Humble ---
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        sudo vim nano tmux ca-certificates gnupg curl locales software-properties-common \
        build-essential python3-dev python3-pip libgeos-dev \
        libgl1 libglib2.0-0 libsm6 libxext6 libxrender1 libx11-6; \
    locale-gen en_US en_US.UTF-8; \
    update-locale LC_ALL=en_US.UTF-8 LANG=en_US.UTF-8; \
    add-apt-repository universe -y; \
    if [ -n "${ROS_SNAPSHOT}" ]; then \
        curl -fsSL "https://keyserver.ubuntu.com/pks/lookup?op=get&search=0x4B63CF8FDE49746E98FA01DDAD19BAB3CBF125EA" \
            -o /usr/share/keyrings/ros-snapshots.asc; \
        echo "deb [signed-by=/usr/share/keyrings/ros-snapshots.asc] http://snapshots.ros.org/humble/${ROS_SNAPSHOT}/ubuntu jammy main" \
            > /etc/apt/sources.list.d/ros-snapshots.list; \
        printf '%s\n' "Package: *" "Pin: origin snapshots.ros.org" "Pin-Priority: 1001" \
            > /etc/apt/preferences.d/ros-snapshots; \
    else \
        curl -fsSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key \
            -o /usr/share/keyrings/ros-archive-keyring.gpg; \
        echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] http://packages.ros.org/ros2/ubuntu jammy main" \
            > /etc/apt/sources.list.d/ros2.list; \
    fi; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ros-${ROS_DISTRO}-ros-base \
        ros-${ROS_DISTRO}-rmw-fastrtps-cpp \
        ros-${ROS_DISTRO}-cv-bridge \
        ros-${ROS_DISTRO}-image-transport \
        ros-${ROS_DISTRO}-compressed-image-transport \
        ros-${ROS_DISTRO}-ackermann-msgs \
        ros-${ROS_DISTRO}-teleop-twist-keyboard \
        ros-${ROS_DISTRO}-usb-cam \
        ros-${ROS_DISTRO}-rosbag2-storage-mcap \
        ros-${ROS_DISTRO}-foxglove-bridge \
        python3-colcon-common-extensions; \
    rm -rf /var/lib/apt/lists/*

# --- Python dependencies for CARLA + ScenarioRunner ---
# Requirements files are copied from the volume-mounted dependencies/ at build time.
# The actual source directories are bind-mounted at runtime (not baked into the image).
#
# numpy<2: ROS 2 Humble's compiled Python packages (cv_bridge etc.) are built against numpy 1.x.
# Note: jammy's pip (22.x) has no --break-system-packages flag and does not need it.
COPY dependencies/PythonAPI/carla/requirements.txt /tmp/carla_requirements.txt
COPY dependencies/scenario_runner/requirements.txt /tmp/scenario_runner_requirements.txt

RUN printf 'numpy<2\n' > /tmp/constraints.txt \
    && pip3 install --no-cache-dir -c /tmp/constraints.txt \
        carla==${CARLA_PIP_VERSION} pygame \
        -r /tmp/carla_requirements.txt \
        -r /tmp/scenario_runner_requirements.txt


RUN SITE="$(python3 -c 'import site; print(site.getsitepackages()[0])')" \
    && echo /opt/carla_src/PythonAPI/carla > "${SITE}/carla_agents.pth" \
    && echo /opt/scenario_runner          > "${SITE}/scenario_runner.pth"

ENV SCENARIO_RUNNER_ROOT=/opt/scenario_runner

# --- User ---
ARG USERNAME=carlauser
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Delete user if UID already exists in the base image
RUN if id -u $USER_UID 2>/dev/null; then userdel "$(id -un $USER_UID)"; fi

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash \
    && usermod -aG video $USERNAME \
    && echo "$USERNAME ALL=(root) NOPASSWD:ALL" > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME

# --- Workspace ---
RUN mkdir -p /home/$USERNAME/carla_ws
WORKDIR /home/$USERNAME/carla_ws

COPY --chown=$USERNAME:$USERNAME docker/bashrc_custom.txt /tmp/bashrc_custom.txt
RUN cat /tmp/bashrc_custom.txt >> /home/$USERNAME/.bashrc && rm /tmp/bashrc_custom.txt

CMD ["bash"]