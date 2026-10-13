#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to carla_msgs__msg__CarlaBoundingBox
/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaBoundingBox {

    // This member is not documented.
    #[allow(missing_docs)]
    pub center: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub size: geometry_msgs::msg::Vector3,

}



impl Default for CarlaBoundingBox {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaBoundingBox::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaBoundingBox {
  type RmwMsg = super::msg::rmw::CarlaBoundingBox;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        center: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.center)).into_owned(),
        size: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.size)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        center: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.center)).into_owned(),
        size: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.size)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      center: geometry_msgs::msg::Vector3::from_rmw_message(msg.center),
      size: geometry_msgs::msg::Vector3::from_rmw_message(msg.size),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaEgoVehicleControl
/// Copyright (c) 2018-2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// This represents a vehicle control message sent to CARLA simulator

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaEgoVehicleControl {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaEgoVehicleControl::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleControl {
  type RmwMsg = super::msg::rmw::CarlaEgoVehicleControl;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        throttle: msg.throttle,
        steer: msg.steer,
        brake: msg.brake,
        hand_brake: msg.hand_brake,
        reverse: msg.reverse,
        gear: msg.gear,
        manual_gear_shift: msg.manual_gear_shift,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      throttle: msg.throttle,
      steer: msg.steer,
      brake: msg.brake,
      hand_brake: msg.hand_brake,
      reverse: msg.reverse,
      gear: msg.gear,
      manual_gear_shift: msg.manual_gear_shift,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      throttle: msg.throttle,
      steer: msg.steer,
      brake: msg.brake,
      hand_brake: msg.hand_brake,
      reverse: msg.reverse,
      gear: msg.gear,
      manual_gear_shift: msg.manual_gear_shift,
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaEgoVehicleStatus
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaEgoVehicleStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub velocity: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub acceleration: geometry_msgs::msg::Accel,


    // This member is not documented.
    #[allow(missing_docs)]
    pub orientation: geometry_msgs::msg::Quaternion,

    /// the current control values, as reported by Carla
    pub control: super::msg::CarlaEgoVehicleControl,

}



impl Default for CarlaEgoVehicleStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaEgoVehicleStatus::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleStatus {
  type RmwMsg = super::msg::rmw::CarlaEgoVehicleStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        velocity: msg.velocity,
        acceleration: geometry_msgs::msg::Accel::into_rmw_message(std::borrow::Cow::Owned(msg.acceleration)).into_owned(),
        orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Owned(msg.orientation)).into_owned(),
        control: super::msg::CarlaEgoVehicleControl::into_rmw_message(std::borrow::Cow::Owned(msg.control)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      velocity: msg.velocity,
        acceleration: geometry_msgs::msg::Accel::into_rmw_message(std::borrow::Cow::Borrowed(&msg.acceleration)).into_owned(),
        orientation: geometry_msgs::msg::Quaternion::into_rmw_message(std::borrow::Cow::Borrowed(&msg.orientation)).into_owned(),
        control: super::msg::CarlaEgoVehicleControl::into_rmw_message(std::borrow::Cow::Borrowed(&msg.control)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      velocity: msg.velocity,
      acceleration: geometry_msgs::msg::Accel::from_rmw_message(msg.acceleration),
      orientation: geometry_msgs::msg::Quaternion::from_rmw_message(msg.orientation),
      control: super::msg::CarlaEgoVehicleControl::from_rmw_message(msg.control),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaEgoVehicleInfoWheel
/// Copyright (c) 2019-2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    pub position: geometry_msgs::msg::Vector3,

}



impl Default for CarlaEgoVehicleInfoWheel {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaEgoVehicleInfoWheel::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleInfoWheel {
  type RmwMsg = super::msg::rmw::CarlaEgoVehicleInfoWheel;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        tire_friction: msg.tire_friction,
        damping_rate: msg.damping_rate,
        max_steer_angle: msg.max_steer_angle,
        radius: msg.radius,
        max_brake_torque: msg.max_brake_torque,
        max_handbrake_torque: msg.max_handbrake_torque,
        position: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.position)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      tire_friction: msg.tire_friction,
      damping_rate: msg.damping_rate,
      max_steer_angle: msg.max_steer_angle,
      radius: msg.radius,
      max_brake_torque: msg.max_brake_torque,
      max_handbrake_torque: msg.max_handbrake_torque,
        position: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.position)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      tire_friction: msg.tire_friction,
      damping_rate: msg.damping_rate,
      max_steer_angle: msg.max_steer_angle,
      radius: msg.radius,
      max_brake_torque: msg.max_brake_torque,
      max_handbrake_torque: msg.max_handbrake_torque,
      position: geometry_msgs::msg::Vector3::from_rmw_message(msg.position),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaEgoVehicleInfo
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaEgoVehicleInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rolename: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub wheels: Vec<super::msg::CarlaEgoVehicleInfoWheel>,


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
    pub center_of_mass: geometry_msgs::msg::Vector3,

}



impl Default for CarlaEgoVehicleInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaEgoVehicleInfo::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaEgoVehicleInfo {
  type RmwMsg = super::msg::rmw::CarlaEgoVehicleInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        type_: msg.type_.as_str().into(),
        rolename: msg.rolename.as_str().into(),
        wheels: msg.wheels
          .into_iter()
          .map(|elem| super::msg::CarlaEgoVehicleInfoWheel::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        max_rpm: msg.max_rpm,
        moi: msg.moi,
        damping_rate_full_throttle: msg.damping_rate_full_throttle,
        damping_rate_zero_throttle_clutch_engaged: msg.damping_rate_zero_throttle_clutch_engaged,
        damping_rate_zero_throttle_clutch_disengaged: msg.damping_rate_zero_throttle_clutch_disengaged,
        use_gear_autobox: msg.use_gear_autobox,
        gear_switch_time: msg.gear_switch_time,
        clutch_strength: msg.clutch_strength,
        mass: msg.mass,
        drag_coefficient: msg.drag_coefficient,
        center_of_mass: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.center_of_mass)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
        type_: msg.type_.as_str().into(),
        rolename: msg.rolename.as_str().into(),
        wheels: msg.wheels
          .iter()
          .map(|elem| super::msg::CarlaEgoVehicleInfoWheel::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      max_rpm: msg.max_rpm,
      moi: msg.moi,
      damping_rate_full_throttle: msg.damping_rate_full_throttle,
      damping_rate_zero_throttle_clutch_engaged: msg.damping_rate_zero_throttle_clutch_engaged,
      damping_rate_zero_throttle_clutch_disengaged: msg.damping_rate_zero_throttle_clutch_disengaged,
      use_gear_autobox: msg.use_gear_autobox,
      gear_switch_time: msg.gear_switch_time,
      clutch_strength: msg.clutch_strength,
      mass: msg.mass,
      drag_coefficient: msg.drag_coefficient,
        center_of_mass: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.center_of_mass)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      type_: msg.type_.to_string(),
      rolename: msg.rolename.to_string(),
      wheels: msg.wheels
          .into_iter()
          .map(super::msg::CarlaEgoVehicleInfoWheel::from_rmw_message)
          .collect(),
      max_rpm: msg.max_rpm,
      moi: msg.moi,
      damping_rate_full_throttle: msg.damping_rate_full_throttle,
      damping_rate_zero_throttle_clutch_engaged: msg.damping_rate_zero_throttle_clutch_engaged,
      damping_rate_zero_throttle_clutch_disengaged: msg.damping_rate_zero_throttle_clutch_disengaged,
      use_gear_autobox: msg.use_gear_autobox,
      gear_switch_time: msg.gear_switch_time,
      clutch_strength: msg.clutch_strength,
      mass: msg.mass,
      drag_coefficient: msg.drag_coefficient,
      center_of_mass: geometry_msgs::msg::Vector3::from_rmw_message(msg.center_of_mass),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaCollisionEvent
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// A collision event

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaCollisionEvent {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub other_actor_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub normal_impulse: geometry_msgs::msg::Vector3,

}



impl Default for CarlaCollisionEvent {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaCollisionEvent::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaCollisionEvent {
  type RmwMsg = super::msg::rmw::CarlaCollisionEvent;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        other_actor_id: msg.other_actor_id,
        normal_impulse: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.normal_impulse)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      other_actor_id: msg.other_actor_id,
        normal_impulse: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.normal_impulse)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      other_actor_id: msg.other_actor_id,
      normal_impulse: geometry_msgs::msg::Vector3::from_rmw_message(msg.normal_impulse),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaLaneInvasionEvent
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// A lane invasion event

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaLaneInvasionEvent {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub crossed_lane_markings: Vec<i32>,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaLaneInvasionEvent::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaLaneInvasionEvent {
  type RmwMsg = super::msg::rmw::CarlaLaneInvasionEvent;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        crossed_lane_markings: msg.crossed_lane_markings.as_slice().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        crossed_lane_markings: msg.crossed_lane_markings.as_slice().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      crossed_lane_markings: msg.crossed_lane_markings.into(),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaWorldInfo
/// Copyright (c) 2018-2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaWorldInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub map_name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub opendrive: std::string::String,

}



impl Default for CarlaWorldInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaWorldInfo::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaWorldInfo {
  type RmwMsg = super::msg::rmw::CarlaWorldInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        map_name: msg.map_name.as_str().into(),
        opendrive: msg.opendrive.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        map_name: msg.map_name.as_str().into(),
        opendrive: msg.opendrive.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      map_name: msg.map_name.to_string(),
      opendrive: msg.opendrive.to_string(),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaActorInfo
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaActorInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,

    /// 0 if no parent available
    pub parent_id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub rolename: std::string::String,

}



impl Default for CarlaActorInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaActorInfo::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaActorInfo {
  type RmwMsg = super::msg::rmw::CarlaActorInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        parent_id: msg.parent_id,
        type_: msg.type_.as_str().into(),
        rolename: msg.rolename.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      parent_id: msg.parent_id,
        type_: msg.type_.as_str().into(),
        rolename: msg.rolename.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      parent_id: msg.parent_id,
      type_: msg.type_.to_string(),
      rolename: msg.rolename.to_string(),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaActorList
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaActorList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub actors: Vec<super::msg::CarlaActorInfo>,

}



impl Default for CarlaActorList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaActorList::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaActorList {
  type RmwMsg = super::msg::rmw::CarlaActorList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        actors: msg.actors
          .into_iter()
          .map(|elem| super::msg::CarlaActorInfo::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        actors: msg.actors
          .iter()
          .map(|elem| super::msg::CarlaActorInfo::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      actors: msg.actors
          .into_iter()
          .map(super::msg::CarlaActorInfo::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaControl
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaControl::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaControl {
  type RmwMsg = super::msg::rmw::CarlaControl;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        command: msg.command,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      command: msg.command,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      command: msg.command,
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaStatus
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaStatus::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaStatus {
  type RmwMsg = super::msg::rmw::CarlaStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        frame: msg.frame,
        fixed_delta_seconds: msg.fixed_delta_seconds,
        synchronous_mode: msg.synchronous_mode,
        synchronous_mode_running: msg.synchronous_mode_running,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      frame: msg.frame,
      fixed_delta_seconds: msg.fixed_delta_seconds,
      synchronous_mode: msg.synchronous_mode,
      synchronous_mode_running: msg.synchronous_mode_running,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      frame: msg.frame,
      fixed_delta_seconds: msg.fixed_delta_seconds,
      synchronous_mode: msg.synchronous_mode,
      synchronous_mode_running: msg.synchronous_mode_running,
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaTrafficLightInfo
/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaTrafficLightInfo {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub transform: geometry_msgs::msg::Pose,

    /// position is relative to transform
    pub trigger_volume: super::msg::CarlaBoundingBox,

}



impl Default for CarlaTrafficLightInfo {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaTrafficLightInfo::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightInfo {
  type RmwMsg = super::msg::rmw::CarlaTrafficLightInfo;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        transform: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.transform)).into_owned(),
        trigger_volume: super::msg::CarlaBoundingBox::into_rmw_message(std::borrow::Cow::Owned(msg.trigger_volume)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
        transform: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.transform)).into_owned(),
        trigger_volume: super::msg::CarlaBoundingBox::into_rmw_message(std::borrow::Cow::Borrowed(&msg.trigger_volume)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      transform: geometry_msgs::msg::Pose::from_rmw_message(msg.transform),
      trigger_volume: super::msg::CarlaBoundingBox::from_rmw_message(msg.trigger_volume),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaTrafficLightInfoList
/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaTrafficLightInfoList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub traffic_lights: Vec<super::msg::CarlaTrafficLightInfo>,

}



impl Default for CarlaTrafficLightInfoList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaTrafficLightInfoList::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightInfoList {
  type RmwMsg = super::msg::rmw::CarlaTrafficLightInfoList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        traffic_lights: msg.traffic_lights
          .into_iter()
          .map(|elem| super::msg::CarlaTrafficLightInfo::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        traffic_lights: msg.traffic_lights
          .iter()
          .map(|elem| super::msg::CarlaTrafficLightInfo::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      traffic_lights: msg.traffic_lights
          .into_iter()
          .map(super::msg::CarlaTrafficLightInfo::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaTrafficLightStatus
/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaTrafficLightStatus::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightStatus {
  type RmwMsg = super::msg::rmw::CarlaTrafficLightStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        state: msg.state,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      state: msg.state,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      state: msg.state,
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaTrafficLightStatusList
/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaTrafficLightStatusList {

    // This member is not documented.
    #[allow(missing_docs)]
    pub traffic_lights: Vec<super::msg::CarlaTrafficLightStatus>,

}



impl Default for CarlaTrafficLightStatusList {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaTrafficLightStatusList::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaTrafficLightStatusList {
  type RmwMsg = super::msg::rmw::CarlaTrafficLightStatusList;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        traffic_lights: msg.traffic_lights
          .into_iter()
          .map(|elem| super::msg::CarlaTrafficLightStatus::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        traffic_lights: msg.traffic_lights
          .iter()
          .map(|elem| super::msg::CarlaTrafficLightStatus::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      traffic_lights: msg.traffic_lights
          .into_iter()
          .map(super::msg::CarlaTrafficLightStatus::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaWalkerControl
/// Copyright (c) 2019 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.
///
/// This represents a walker control message sent to CARLA simulator

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CarlaWalkerControl {

    // This member is not documented.
    #[allow(missing_docs)]
    pub direction: geometry_msgs::msg::Vector3,


    // This member is not documented.
    #[allow(missing_docs)]
    pub speed: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub jump: bool,

}



impl Default for CarlaWalkerControl {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaWalkerControl::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaWalkerControl {
  type RmwMsg = super::msg::rmw::CarlaWalkerControl;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        direction: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Owned(msg.direction)).into_owned(),
        speed: msg.speed,
        jump: msg.jump,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        direction: geometry_msgs::msg::Vector3::into_rmw_message(std::borrow::Cow::Borrowed(&msg.direction)).into_owned(),
      speed: msg.speed,
      jump: msg.jump,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      direction: geometry_msgs::msg::Vector3::from_rmw_message(msg.direction),
      speed: msg.speed,
      jump: msg.jump,
    }
  }
}


// Corresponds to carla_msgs__msg__CarlaWeatherParameters
/// Copyright (c) 2020 Intel Corporation.
///
/// This work is licensed under the terms of the MIT license.
/// For a copy, see <https://opensource.org/licenses/MIT>.

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::CarlaWeatherParameters::default())
  }
}

impl rosidl_runtime_rs::Message for CarlaWeatherParameters {
  type RmwMsg = super::msg::rmw::CarlaWeatherParameters;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        cloudiness: msg.cloudiness,
        precipitation: msg.precipitation,
        precipitation_deposits: msg.precipitation_deposits,
        wind_intensity: msg.wind_intensity,
        fog_density: msg.fog_density,
        fog_distance: msg.fog_distance,
        wetness: msg.wetness,
        sun_azimuth_angle: msg.sun_azimuth_angle,
        sun_altitude_angle: msg.sun_altitude_angle,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      cloudiness: msg.cloudiness,
      precipitation: msg.precipitation,
      precipitation_deposits: msg.precipitation_deposits,
      wind_intensity: msg.wind_intensity,
      fog_density: msg.fog_density,
      fog_distance: msg.fog_distance,
      wetness: msg.wetness,
      sun_azimuth_angle: msg.sun_azimuth_angle,
      sun_altitude_angle: msg.sun_altitude_angle,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      cloudiness: msg.cloudiness,
      precipitation: msg.precipitation,
      precipitation_deposits: msg.precipitation_deposits,
      wind_intensity: msg.wind_intensity,
      fog_density: msg.fog_density,
      fog_distance: msg.fog_distance,
      wetness: msg.wetness,
      sun_azimuth_angle: msg.sun_azimuth_angle,
      sun_altitude_angle: msg.sun_altitude_angle,
    }
  }
}


