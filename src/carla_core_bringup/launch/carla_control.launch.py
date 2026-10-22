from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node
from ament_index_python.packages import get_package_share_directory
import os

"""Launch file for the CARLA control, including keyboard and G29 steering wheel manual control, as well as route logging.
"""

def generate_launch_description():
    drone_ns_arg = DeclareLaunchArgument(
        'drone_ns', default_value='tello1',
        description='Namespace of the drone (e.g. tello1)')

    drone_ns = LaunchConfiguration('drone_ns')

    # Load mission configuration
    config_dir = os.path.join(
        get_package_share_directory('carla_core_bringup'), 'config')
    carla_config = os.path.join(config_dir, 'control_config.yaml')

    joy_node = Node(
        package='joy',
        executable='joy_node',
        name='joy_node',
        parameters=[{
            'device_id': 0
        }],
        output='screen'
    )

    # Maps G29 raw axes/buttons to CarlaEgoVehicleControl.
    g29_mapping_node = Node(
        package='g29_control',
        executable='g29_to_carla',
        name='g29_to_carla_node',
        parameters=[{
            'role_name': LaunchConfiguration('role_name'),
            'steer_axis': LaunchConfiguration('steer_axis'),
            'throttle_axis': LaunchConfiguration('throttle_axis'),
            'brake_axis': LaunchConfiguration('brake_axis'),
        }],
        output='screen'
    )

    # Optional: logs odometry to a CSV while driving.
    route_logger_node = Node(
        package='g29_control',
        executable='route_logger',
        name='route_logger_node',
        parameters=[{
            'role_name': LaunchConfiguration('role_name'),
            'output_file': LaunchConfiguration('output_file')
        }],
        output='screen'
    )

    return LaunchDescription([
        drone_ns_arg,
        mission_control_node
    ])