#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to carla_msgs__srv__SpawnObject_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnObject_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub id: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub attributes: Vec<diagnostic_msgs::msg::KeyValue>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub transform: geometry_msgs::msg::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub attach_to: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub random_pose: bool,

}



impl Default for SpawnObject_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpawnObject_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SpawnObject_Request {
  type RmwMsg = super::srv::rmw::SpawnObject_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_.as_str().into(),
        id: msg.id.as_str().into(),
        attributes: msg.attributes
          .into_iter()
          .map(|elem| diagnostic_msgs::msg::KeyValue::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
        transform: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Owned(msg.transform)).into_owned(),
        attach_to: msg.attach_to,
        random_pose: msg.random_pose,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        type_: msg.type_.as_str().into(),
        id: msg.id.as_str().into(),
        attributes: msg.attributes
          .iter()
          .map(|elem| diagnostic_msgs::msg::KeyValue::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
        transform: geometry_msgs::msg::Pose::into_rmw_message(std::borrow::Cow::Borrowed(&msg.transform)).into_owned(),
      attach_to: msg.attach_to,
      random_pose: msg.random_pose,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      type_: msg.type_.to_string(),
      id: msg.id.to_string(),
      attributes: msg.attributes
          .into_iter()
          .map(diagnostic_msgs::msg::KeyValue::from_rmw_message)
          .collect(),
      transform: geometry_msgs::msg::Pose::from_rmw_message(msg.transform),
      attach_to: msg.attach_to,
      random_pose: msg.random_pose,
    }
  }
}


// Corresponds to carla_msgs__srv__SpawnObject_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnObject_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_string: std::string::String,

}



impl Default for SpawnObject_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SpawnObject_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SpawnObject_Response {
  type RmwMsg = super::srv::rmw::SpawnObject_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
        error_string: msg.error_string.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
        error_string: msg.error_string.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
      error_string: msg.error_string.to_string(),
    }
  }
}


// Corresponds to carla_msgs__srv__DestroyObject_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DestroyObject_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,

}



impl Default for DestroyObject_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DestroyObject_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DestroyObject_Request {
  type RmwMsg = super::srv::rmw::DestroyObject_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        id: msg.id,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      id: msg.id,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      id: msg.id,
    }
  }
}


// Corresponds to carla_msgs__srv__DestroyObject_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DestroyObject_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for DestroyObject_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DestroyObject_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DestroyObject_Response {
  type RmwMsg = super::srv::rmw::DestroyObject_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
  }
}


// Corresponds to carla_msgs__srv__GetBlueprints_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetBlueprints_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filter: std::string::String,

}



impl Default for GetBlueprints_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetBlueprints_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GetBlueprints_Request {
  type RmwMsg = super::srv::rmw::GetBlueprints_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filter: msg.filter.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filter: msg.filter.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filter: msg.filter.to_string(),
    }
  }
}


// Corresponds to carla_msgs__srv__GetBlueprints_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetBlueprints_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub blueprints: Vec<std::string::String>,

}



impl Default for GetBlueprints_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GetBlueprints_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GetBlueprints_Response {
  type RmwMsg = super::srv::rmw::GetBlueprints_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        blueprints: msg.blueprints
          .into_iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        blueprints: msg.blueprints
          .iter()
          .map(|elem| elem.as_str().into())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      blueprints: msg.blueprints
          .into_iter()
          .map(|elem| elem.to_string())
          .collect(),
    }
  }
}






#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__carla_msgs__srv__SpawnObject() -> *const std::ffi::c_void;
}

// Corresponds to carla_msgs__srv__SpawnObject
#[allow(missing_docs, non_camel_case_types)]
pub struct SpawnObject;

impl rosidl_runtime_rs::Service for SpawnObject {
    type Request = SpawnObject_Request;
    type Response = SpawnObject_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__carla_msgs__srv__SpawnObject() }
    }
}




#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__carla_msgs__srv__DestroyObject() -> *const std::ffi::c_void;
}

// Corresponds to carla_msgs__srv__DestroyObject
#[allow(missing_docs, non_camel_case_types)]
pub struct DestroyObject;

impl rosidl_runtime_rs::Service for DestroyObject {
    type Request = DestroyObject_Request;
    type Response = DestroyObject_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__carla_msgs__srv__DestroyObject() }
    }
}




#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__carla_msgs__srv__GetBlueprints() -> *const std::ffi::c_void;
}

// Corresponds to carla_msgs__srv__GetBlueprints
#[allow(missing_docs, non_camel_case_types)]
pub struct GetBlueprints;

impl rosidl_runtime_rs::Service for GetBlueprints {
    type Request = GetBlueprints_Request;
    type Response = GetBlueprints_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__carla_msgs__srv__GetBlueprints() }
    }
}


