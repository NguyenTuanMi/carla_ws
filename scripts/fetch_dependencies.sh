#!/bin/bash
# Downloads ScenarioRunner and CARLA PythonAPI into dependencies/
# Run once from the repo root: bash scripts/fetch_dependencies.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
DEPS_DIR="$REPO_ROOT/dependencies"

SCENARIO_RUNNER_VERSION="v1.0"
CARLA_VERSION="1.0"
RAW_VERSION="${1:-310}"
PYTHON_VERSION="${RAW_VERSION//./}"

mkdir -p "$DEPS_DIR"

echo "[1/4] Downloading ScenarioRunner ${SCENARIO_RUNNER_VERSION}..."
if [[ -d "$DEPS_DIR/scenario_runner" || -f "$DEPS_DIR/scenario_runner.tar.gz" ]]; then
    echo "ScenarioRunner already exists in $DEPS_DIR/scenario_runner. Skipping download."
else
    echo "ScenarioRunner not found. Downloading..."
    # --- ScenarioRunner ---
    curl -L "https://github.com/carla-simulator/scenario_runner/archive/refs/tags/${SCENARIO_RUNNER_VERSION}.tar.gz" \
        -o "$DEPS_DIR/scenario_runner.tar.gz"
fi

echo "[2/4] Extracting ScenarioRunner..."
if [ ! -f "$DEPS_DIR/scenario_runner.tar.gz" ]; then
    echo "ScenarioRunner already extracted in $DEPS_DIR/scenario_runner. Skipping extraction."
else
    echo "Extracting ScenarioRunner..."
    tar -xzf "$DEPS_DIR/scenario_runner.tar.gz" -C "$DEPS_DIR"
    mv "$DEPS_DIR/scenario_runner-${SCENARIO_RUNNER_VERSION#v}" "$DEPS_DIR/scenario_runner"
    rm "$DEPS_DIR/scenario_runner.tar.gz"
fi

# --- CARLA PythonAPI ---
echo "[3/4] Downloading CARLA ${CARLA_VERSION} source..."
if [[ -d "$DEPS_DIR/PythonAPI" || -f "$DEPS_DIR/carla.tar.gz" ]]; then
    echo "CARLA PythonAPI already exists in $DEPS_DIR/PythonAPI. Skipping download."
else
    echo "CARLA PythonAPI not found. Downloading..."
    curl -L "https://downloads.carlasim.com/Linux/CARLA_${CARLA_VERSION}.tar.gz" \
        -o "$DEPS_DIR/carla.tar.gz"
fi

echo "[4/4] Extracting CARLA PythonAPI only..."
# Extract only the PythonAPI/carla subdirectory to avoid pulling the entire CARLA source
if [ ! -f "$DEPS_DIR/carla.tar.gz" ]; then
    echo "CARLA PythonAPI already extracted in $DEPS_DIR/PythonAPI. Skipping extraction."
else
    echo "Extracting CARLA PythonAPI..."
    tar -xzf "$DEPS_DIR/carla.tar.gz" -C "$DEPS_DIR" \
        --strip-components=1 \
        "Carla-${CARLA_VERSION}-Linux-Shipping/PythonAPI"

    mkdir -p "$DEPS_DIR/wheels"
    if [[ -f "$DEPS_DIR/PythonAPI/carla/dist/carla-${CARLA_VERSION}-cp${PYTHON_VERSION}-cp${PYTHON_VERSION}-linux_x86_64.whl" ]]; then
        echo "Copying CARLA wheel to $DEPS_DIR/wheels..."
        cp "$DEPS_DIR/PythonAPI/carla/dist/carla-${CARLA_VERSION}-cp${PYTHON_VERSION}-cp${PYTHON_VERSION}-linux_x86_64.whl" "$DEPS_DIR/wheels/"
    else
        echo "CARLA wheel of version ${CARLA_VERSION} for Python ${PYTHON_VERSION} not found in $DEPS_DIR/PythonAPI/carla/dist/. Please check the CARLA version and Python version compatibility."
    fi

    # Result: dependencies/PythonAPI/carla/
    rm "$DEPS_DIR/carla.tar.gz"
fi

echo ""
echo "Done. Verify:"
echo "  dependencies/scenario_runner/scenario_runner.py"
echo "  dependencies/PythonAPI/carla/agents/"
echo "  dependencies/wheels/carla-*.whl"