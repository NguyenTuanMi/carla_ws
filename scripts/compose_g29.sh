#!/usr/bin/env bash
set -euo pipefail

by_id_dir=/dev/input/by-id
js_link=$(find "$by_id_dir" -maxdepth 1 -type l \
    -iname '*g29*-joystick' ! -iname '*event*' -print -quit)
event_link=$(find "$by_id_dir" -maxdepth 1 -type l \
    -iname '*g29*-event-joystick' -print -quit)

if [[ -z "$js_link" || -z "$event_link" ]]; then
    echo 'Could not find both G29 joystick and event devices.' >&2
    echo 'Expected links under /dev/input/by-id/*G29*.' >&2
    exit 1
fi

export G29_JS_DEVICE="$(readlink -f "$js_link")"
export G29_EVENT_DEVICE="$(readlink -f "$event_link")"

echo "G29 joystick: $G29_JS_DEVICE"
echo "G29 event:    $G29_EVENT_DEVICE"

docker compose "$@"
