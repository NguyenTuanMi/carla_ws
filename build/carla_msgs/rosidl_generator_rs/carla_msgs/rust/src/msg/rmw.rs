#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaBoundingBox() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaBoundingBox__init(msg: *mut CarlaBoundingBox) -> bool;
    fn carla_msgs__msg__CarlaBoundingBox__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaBoundingBox>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaBoundingBox__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaBoundingBox>);
    fn carla_msgs__msg__CarlaBoundingBox__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaBoundingBox>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaBoundingBox>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaBoundingBox
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaBoundingBox {

    // This member is not documented.
    #[allow(missing_docs)]
    pub center: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub size: geometry_msgs::msg::rmw::Vector3,

}



impl Default for CarlaBoundingBox {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaBoundingBox__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaBoundingBox__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaBoundingBox {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaBoundingBox__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaBoundingBox__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaBoundingBox__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaBoundingBox {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaBoundingBox where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaBoundingBox";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaBoundingBox() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleControl() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaEgoVehicleControl__init(msg: *mut CarlaEgoVehicleControl) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleControl__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleControl>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleControl__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleControl>);
    fn carla_msgs__msg__CarlaEgoVehicleControl__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaEgoVehicleControl>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleControl>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaEgoVehicleControl
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2018-2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// This represents a vehicle control message sent to CARLA simulator

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaEgoVehicleControl {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// The CARLA vehicle control data
    /// 0. <= throttle <= 1.
    pub throttle: f32,

    /// -1. <= steer <= 1.
    pub steer: f32,

    /// 0. <= brake <= 1.
    pub brake: f32,

    /// hand_brake 0 or 1
    pub hand_brake: bool,

    /// reverse 0 or 1
    pub reverse: bool,

    /// gear
    pub gear: i32,

    /// manual gear shift
    pub manual_gear_shift: bool,

}



impl Default for CarlaEgoVehicleControl {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaEgoVehicleControl__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaEgoVehicleControl__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaEgoVehicleControl {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleControl__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleControl__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleControl__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleControl {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaEgoVehicleControl where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaEgoVehicleControl";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleControl() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleStatus() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaEgoVehicleStatus__init(msg: *mut CarlaEgoVehicleStatus) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleStatus>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleStatus>);
    fn carla_msgs__msg__CarlaEgoVehicleStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaEgoVehicleStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleStatus>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaEgoVehicleStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaEgoVehicleStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acceleration: geometry_msgs::msg::rmw::Accel,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation: geometry_msgs::msg::rmw::Quaternion,

    /// the current control values, as reported by Carla
    pub control: super::super::msg::rmw::CarlaEgoVehicleControl,

}



impl Default for CarlaEgoVehicleStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaEgoVehicleStatus__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaEgoVehicleStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaEgoVehicleStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaEgoVehicleStatus where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaEgoVehicleStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleStatus() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleInfoWheel() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaEgoVehicleInfoWheel__init(msg: *mut CarlaEgoVehicleInfoWheel) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleInfoWheel__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfoWheel>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleInfoWheel__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfoWheel>);
    fn carla_msgs__msg__CarlaEgoVehicleInfoWheel__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfoWheel>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfoWheel>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaEgoVehicleInfoWheel
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019-2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaEgoVehicleInfoWheel {

    // This member is not documented.
    #[allow(missing_docs)]
    pub tire_friction: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping_rate: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_steer_angle: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub radius: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_brake_torque: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_handbrake_torque: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub position: geometry_msgs::msg::rmw::Vector3,

}



impl Default for CarlaEgoVehicleInfoWheel {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaEgoVehicleInfoWheel__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaEgoVehicleInfoWheel__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaEgoVehicleInfoWheel {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleInfoWheel__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleInfoWheel__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleInfoWheel__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleInfoWheel {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaEgoVehicleInfoWheel where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaEgoVehicleInfoWheel";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleInfoWheel() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleInfo() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaEgoVehicleInfo__init(msg: *mut CarlaEgoVehicleInfo) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfo>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaEgoVehicleInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfo>);
    fn carla_msgs__msg__CarlaEgoVehicleInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaEgoVehicleInfo>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaEgoVehicleInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaEgoVehicleInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rolename: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wheels: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CarlaEgoVehicleInfoWheel>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub max_rpm: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub moi: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping_rate_full_throttle: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping_rate_zero_throttle_clutch_engaged: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub damping_rate_zero_throttle_clutch_disengaged: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub use_gear_autobox: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub gear_switch_time: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub clutch_strength: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub mass: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub drag_coefficient: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub center_of_mass: geometry_msgs::msg::rmw::Vector3,

}



impl Default for CarlaEgoVehicleInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaEgoVehicleInfo__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaEgoVehicleInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaEgoVehicleInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaEgoVehicleInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaEgoVehicleInfo where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaEgoVehicleInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaEgoVehicleInfo() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaCollisionEvent() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaCollisionEvent__init(msg: *mut CarlaCollisionEvent) -> bool;
    fn carla_msgs__msg__CarlaCollisionEvent__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaCollisionEvent>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaCollisionEvent__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaCollisionEvent>);
    fn carla_msgs__msg__CarlaCollisionEvent__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaCollisionEvent>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaCollisionEvent>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaCollisionEvent
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// A collision event

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaCollisionEvent {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub other_actor_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub normal_impulse: geometry_msgs::msg::rmw::Vector3,

}



impl Default for CarlaCollisionEvent {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaCollisionEvent__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaCollisionEvent__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaCollisionEvent {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaCollisionEvent__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaCollisionEvent__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaCollisionEvent__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaCollisionEvent {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaCollisionEvent where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaCollisionEvent";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaCollisionEvent() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaLaneInvasionEvent() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaLaneInvasionEvent__init(msg: *mut CarlaLaneInvasionEvent) -> bool;
    fn carla_msgs__msg__CarlaLaneInvasionEvent__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaLaneInvasionEvent>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaLaneInvasionEvent__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaLaneInvasionEvent>);
    fn carla_msgs__msg__CarlaLaneInvasionEvent__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaLaneInvasionEvent>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaLaneInvasionEvent>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaLaneInvasionEvent
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// A lane invasion event

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaLaneInvasionEvent {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub crossed_lane_markings: rosidl_runtime_rs::Sequence<i32>,

}

impl CarlaLaneInvasionEvent {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LANE_MARKING_OTHER: i32 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LANE_MARKING_BROKEN: i32 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LANE_MARKING_SOLID: i32 = 2;

}


impl Default for CarlaLaneInvasionEvent {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaLaneInvasionEvent__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaLaneInvasionEvent__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaLaneInvasionEvent {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaLaneInvasionEvent__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaLaneInvasionEvent__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaLaneInvasionEvent__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaLaneInvasionEvent {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaLaneInvasionEvent where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaLaneInvasionEvent";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaLaneInvasionEvent() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaWorldInfo() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaWorldInfo__init(msg: *mut CarlaWorldInfo) -> bool;
    fn carla_msgs__msg__CarlaWorldInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaWorldInfo>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaWorldInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaWorldInfo>);
    fn carla_msgs__msg__CarlaWorldInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaWorldInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaWorldInfo>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaWorldInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2018-2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaWorldInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub map_name: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub opendrive: rosidl_runtime_rs::String,

}



impl Default for CarlaWorldInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaWorldInfo__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaWorldInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaWorldInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWorldInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWorldInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWorldInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaWorldInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaWorldInfo where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaWorldInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaWorldInfo() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaActorInfo() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaActorInfo__init(msg: *mut CarlaActorInfo) -> bool;
    fn carla_msgs__msg__CarlaActorInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaActorInfo>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaActorInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaActorInfo>);
    fn carla_msgs__msg__CarlaActorInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaActorInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaActorInfo>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaActorInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaActorInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,

    /// 0 if no parent available
    pub parent_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rolename: rosidl_runtime_rs::String,

}



impl Default for CarlaActorInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaActorInfo__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaActorInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaActorInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaActorInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaActorInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaActorInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaActorInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaActorInfo where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaActorInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaActorInfo() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaActorList() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaActorList__init(msg: *mut CarlaActorList) -> bool;
    fn carla_msgs__msg__CarlaActorList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaActorList>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaActorList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaActorList>);
    fn carla_msgs__msg__CarlaActorList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaActorList>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaActorList>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaActorList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaActorList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub actors: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CarlaActorInfo>,

}



impl Default for CarlaActorList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaActorList__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaActorList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaActorList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaActorList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaActorList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaActorList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaActorList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaActorList where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaActorList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaActorList() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaControl() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaControl__init(msg: *mut CarlaControl) -> bool;
    fn carla_msgs__msg__CarlaControl__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaControl>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaControl__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaControl>);
    fn carla_msgs__msg__CarlaControl__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaControl>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaControl>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaControl
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaControl {

    // This member is not documented.
    #[allow(missing_docs)]
    pub command: i8,

}

impl CarlaControl {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PLAY: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const PAUSE: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const STEP_ONCE: i8 = 2;

}


impl Default for CarlaControl {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaControl__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaControl__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaControl {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaControl__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaControl__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaControl__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaControl {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaControl where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaControl";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaControl() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaStatus() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaStatus__init(msg: *mut CarlaStatus) -> bool;
    fn carla_msgs__msg__CarlaStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaStatus>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaStatus>);
    fn carla_msgs__msg__CarlaStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaStatus>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaStatus {
    /// frame number
    pub frame: u64,

    /// duration of one frame
    pub fixed_delta_seconds: f32,

    /// carla is in synchronous mode
    pub synchronous_mode: bool,

    /// true: running, false: paused
    pub synchronous_mode_running: bool,

}



impl Default for CarlaStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaStatus__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaStatus where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaStatus() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightInfo() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaTrafficLightInfo__init(msg: *mut CarlaTrafficLightInfo) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightInfo__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightInfo>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightInfo__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightInfo>);
    fn carla_msgs__msg__CarlaTrafficLightInfo__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaTrafficLightInfo>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightInfo>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaTrafficLightInfo
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaTrafficLightInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub transform: geometry_msgs::msg::rmw::Pose,

    /// position is relative to transform
    pub trigger_volume: super::super::msg::rmw::CarlaBoundingBox,

}



impl Default for CarlaTrafficLightInfo {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaTrafficLightInfo__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaTrafficLightInfo__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaTrafficLightInfo {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightInfo__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightInfo__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightInfo__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightInfo {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaTrafficLightInfo where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaTrafficLightInfo";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightInfo() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightInfoList() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaTrafficLightInfoList__init(msg: *mut CarlaTrafficLightInfoList) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightInfoList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightInfoList>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightInfoList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightInfoList>);
    fn carla_msgs__msg__CarlaTrafficLightInfoList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaTrafficLightInfoList>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightInfoList>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaTrafficLightInfoList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaTrafficLightInfoList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub traffic_lights: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CarlaTrafficLightInfo>,

}



impl Default for CarlaTrafficLightInfoList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaTrafficLightInfoList__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaTrafficLightInfoList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaTrafficLightInfoList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightInfoList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightInfoList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightInfoList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightInfoList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaTrafficLightInfoList where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaTrafficLightInfoList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightInfoList() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightStatus() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaTrafficLightStatus__init(msg: *mut CarlaTrafficLightStatus) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightStatus>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightStatus>);
    fn carla_msgs__msg__CarlaTrafficLightStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaTrafficLightStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightStatus>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaTrafficLightStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaTrafficLightStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub state: u8,

}

impl CarlaTrafficLightStatus {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RED: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const YELLOW: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const GREEN: u8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const OFF: u8 = 3;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNKNOWN: u8 = 4;

}


impl Default for CarlaTrafficLightStatus {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaTrafficLightStatus__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaTrafficLightStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaTrafficLightStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaTrafficLightStatus where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaTrafficLightStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightStatus() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightStatusList() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaTrafficLightStatusList__init(msg: *mut CarlaTrafficLightStatusList) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightStatusList__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightStatusList>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaTrafficLightStatusList__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightStatusList>);
    fn carla_msgs__msg__CarlaTrafficLightStatusList__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaTrafficLightStatusList>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaTrafficLightStatusList>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaTrafficLightStatusList
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaTrafficLightStatusList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub traffic_lights: rosidl_runtime_rs::Sequence<super::super::msg::rmw::CarlaTrafficLightStatus>,

}



impl Default for CarlaTrafficLightStatusList {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaTrafficLightStatusList__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaTrafficLightStatusList__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaTrafficLightStatusList {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightStatusList__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightStatusList__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaTrafficLightStatusList__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightStatusList {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaTrafficLightStatusList where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaTrafficLightStatusList";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaTrafficLightStatusList() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaWalkerControl() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaWalkerControl__init(msg: *mut CarlaWalkerControl) -> bool;
    fn carla_msgs__msg__CarlaWalkerControl__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaWalkerControl>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaWalkerControl__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaWalkerControl>);
    fn carla_msgs__msg__CarlaWalkerControl__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaWalkerControl>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaWalkerControl>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaWalkerControl
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// This represents a walker control message sent to CARLA simulator

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaWalkerControl {

    // This member is not documented.
    #[allow(missing_docs)]
    pub direction: geometry_msgs::msg::rmw::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub jump: bool,

}



impl Default for CarlaWalkerControl {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaWalkerControl__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaWalkerControl__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaWalkerControl {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWalkerControl__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWalkerControl__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWalkerControl__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaWalkerControl {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaWalkerControl where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaWalkerControl";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaWalkerControl() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaWeatherParameters() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__msg__CarlaWeatherParameters__init(msg: *mut CarlaWeatherParameters) -> bool;
    fn carla_msgs__msg__CarlaWeatherParameters__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CarlaWeatherParameters>, size: usize) -> bool;
    fn carla_msgs__msg__CarlaWeatherParameters__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CarlaWeatherParameters>);
    fn carla_msgs__msg__CarlaWeatherParameters__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CarlaWeatherParameters>, out_seq: *mut rosidl_runtime_rs::Sequence<CarlaWeatherParameters>) -> bool;
}

// Corresponds to carla_msgs__msg__CarlaWeatherParameters
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaWeatherParameters {

    // This member is not documented.
    #[allow(missing_docs)]
    pub cloudiness: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub precipitation: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub precipitation_deposits: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wind_intensity: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fog_density: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub fog_distance: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wetness: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sun_azimuth_angle: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub sun_altitude_angle: f32,

}



impl Default for CarlaWeatherParameters {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__msg__CarlaWeatherParameters__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__msg__CarlaWeatherParameters__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CarlaWeatherParameters {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWeatherParameters__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWeatherParameters__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__msg__CarlaWeatherParameters__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CarlaWeatherParameters {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CarlaWeatherParameters where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/msg/CarlaWeatherParameters";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__msg__CarlaWeatherParameters() }
  }
}


