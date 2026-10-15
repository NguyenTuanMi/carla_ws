#!/bin/bash
# Downloads ScenarioRunner and CARLA PythonAPI into dependencies/
# Run once from the repo root: bash scripts/fetch_dependencies.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
DEPS_DIR="$REPO_ROOT/dependencies"

SCENARIO_RUNNER_VERSION="v0.9.16"
CARLA_VERSION="0.9.16"

mkdir -p "$DEPS_DIR"

# --- ScenarioRunner ---
echo "[1/4] Downloading ScenarioRunner ${SCENARIO_RUNNER_VERSION}..."
curl -L "https://github.com/carla-simulator/scenario_runner/archive/refs/tags/${SCENARIO_RUNNER_VERSION}.tar.gz" \
    -o "$DEPS_DIR/scenario_runner.tar.gz"

echo "[2/4] Extracting ScenarioRunner..."
tar -xzf "$DEPS_DIR/scenario_runner.tar.gz" -C "$DEPS_DIR"
# GitHub extracts as scenario_runner-0.9.15 (strips the 'v' prefix)
mv "$DEPS_DIR/scenario_runner-${SCENARIO_RUNNER_VERSION#v}" "$DEPS_DIR/scenario_runner"
rm "$DEPS_DIR/scenario_runner.tar.gz"

# --- CARLA PythonAPI ---
echo "[3/4] Downloading CARLA ${CARLA_VERSION} source..."
curl -L "https://github.com/carla-simulator/carla/archive/refs/tags/${CARLA_VERSION}.tar.gz" \
    -o "$DEPS_DIR/carla.tar.gz"

echo "[4/4] Extracting CARLA PythonAPI only..."
# Extract only the PythonAPI/carla subdirectory to avoid pulling the entire CARLA source
tar -xzf "$DEPS_DIR/carla.tar.gz" -C "$DEPS_DIR" \
    --strip-components=1 \
    "carla-${CARLA_VERSION}/PythonAPI"
# Result: dependencies/PythonAPI/carla/
rm "$DEPS_DIR/carla.tar.gz"

echo ""
echo "Done. Verify:"
echo "  dependencies/scenario_runner/scenario_runner.py"
echo "  dependencies/PythonAPI/carla/agents/"