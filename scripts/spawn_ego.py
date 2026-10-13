import carla
import time
import os

CARLA_HOST = os.environ.get('CARLA_HOST', '127.0.0.1')
CARLA_PORT = int(os.environ.get('CARLA_PORT', 2000))

client = carla.Client(CARLA_HOST, CARLA_PORT)
client.set_timeout(10.0)
world = client.get_world()
bp_lib = world.get_blueprint_library()

# --- Spawn ego vehicle ---
vehicle_bp = bp_lib.find('vehicle.tesla.model3')
vehicle_bp.set_attribute('role_name', 'ego_vehicle')  # role_name is important —
                                                        # CARLA uses it as the ROS topic namespace

spawn_point = world.get_map().get_spawn_points()[0]
vehicle = world.spawn_actor(vehicle_bp, spawn_point)
vehicle.set_autopilot(True)
print(f"Spawned ego vehicle: {vehicle.id}")

# --- Attach RGB front camera ---
camera_bp = bp_lib.find('sensor.camera.rgb')
camera_bp.set_attribute('image_size_x', '1280')
camera_bp.set_attribute('image_size_y', '720')
camera_bp.set_attribute('fov', '90')

# Transform relative to vehicle (front, slightly elevated)
camera_transform = carla.Transform(
    carla.Location(x=1.5, z=2.4),
    carla.Rotation(pitch=-10)
)
camera = world.spawn_actor(camera_bp, camera_transform, attach_to=vehicle)
print(f"Attached RGB camera: {camera.id}")

print("Running — Ctrl+C to despawn")
try:
    while True:
        time.sleep(1)
except KeyboardInterrupt:
    pass
finally:
    camera.destroy()
    vehicle.destroy()
    print("Cleaned up")