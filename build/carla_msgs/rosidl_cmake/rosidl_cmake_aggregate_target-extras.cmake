# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target carla_msgs::carla_msgs
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${carla_msgs_TARGETS}.
if(carla_msgs_TARGETS AND NOT TARGET carla_msgs::carla_msgs)
  add_library(carla_msgs::carla_msgs INTERFACE IMPORTED)
  set_target_properties(carla_msgs::carla_msgs PROPERTIES
    INTERFACE_LINK_LIBRARIES "${carla_msgs_TARGETS}")
endif()
