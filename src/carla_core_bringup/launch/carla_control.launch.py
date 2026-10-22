from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration, PathJoinSubstitution
from launch_ros.actions import Node
from launch_ros.descriptions import ComposableNode
from ament_index_python.packages import get_package_share_directory
import os

"""Launch file for the CARLA control, including keyboard and G29 steering wheel manual control, as well as route logging.
"""

def generate_launch_description():
    config_dir = os.path.join(
        get_package_share_directory('carla_core_bringup'), 'config')
    g29_control_config = PathJoinSubstitution([config_dir, 'carla_control.yaml'])

    joy_node = ComposableNode(
        package='joy',
        executable='joy_node',
        name='joy_node',
        parameters=[{
            g29_control_config
        }],
        output='screen'
    )

    # Maps G29 raw axes/buttons to CarlaEgoVehicleControl.
    g29_mapping_node = ComposableNode(
        package='g29_control',
        executable='g29_to_carla',
        name='g29_to_carla_node',
        parameters=[{
            g29_control_config
        }],
        output='screen'
    )

    # Optional: logs odometry to a CSV while driving.
    route_logger_node = ComposableNode(
        package='g29_control',
        executable='route_logger',
        name='route_logger_node',
        parameters=[{
            g29_control_config
        }],
        output='screen'
    )

    return LaunchDescription([
        joy_node,
        g29_mapping_node,
        route_logger_node
    ])