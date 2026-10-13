#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__SpawnObject_Request() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__srv__SpawnObject_Request__init(msg: *mut SpawnObject_Request) -> bool;
    fn carla_msgs__srv__SpawnObject_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpawnObject_Request>, size: usize) -> bool;
    fn carla_msgs__srv__SpawnObject_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpawnObject_Request>);
    fn carla_msgs__srv__SpawnObject_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpawnObject_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SpawnObject_Request>) -> bool;
}

// Corresponds to carla_msgs__srv__SpawnObject_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnObject_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub type_: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub id: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub attributes: rosidl_runtime_rs::Sequence<diagnostic_msgs::msg::rmw::KeyValue>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub transform: geometry_msgs::msg::rmw::Pose,


    // This member is not documented.
    #[allow(missing_docs)]
    pub attach_to: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub random_pose: bool,

}



impl Default for SpawnObject_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__srv__SpawnObject_Request__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__srv__SpawnObject_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpawnObject_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__SpawnObject_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__SpawnObject_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__SpawnObject_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpawnObject_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpawnObject_Request where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/srv/SpawnObject_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__SpawnObject_Request() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__SpawnObject_Response() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__srv__SpawnObject_Response__init(msg: *mut SpawnObject_Response) -> bool;
    fn carla_msgs__srv__SpawnObject_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SpawnObject_Response>, size: usize) -> bool;
    fn carla_msgs__srv__SpawnObject_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SpawnObject_Response>);
    fn carla_msgs__srv__SpawnObject_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SpawnObject_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SpawnObject_Response>) -> bool;
}

// Corresponds to carla_msgs__srv__SpawnObject_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpawnObject_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub error_string: rosidl_runtime_rs::String,

}



impl Default for SpawnObject_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__srv__SpawnObject_Response__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__srv__SpawnObject_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SpawnObject_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__SpawnObject_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__SpawnObject_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__SpawnObject_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SpawnObject_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SpawnObject_Response where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/srv/SpawnObject_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__SpawnObject_Response() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__DestroyObject_Request() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__srv__DestroyObject_Request__init(msg: *mut DestroyObject_Request) -> bool;
    fn carla_msgs__srv__DestroyObject_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DestroyObject_Request>, size: usize) -> bool;
    fn carla_msgs__srv__DestroyObject_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DestroyObject_Request>);
    fn carla_msgs__srv__DestroyObject_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DestroyObject_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DestroyObject_Request>) -> bool;
}

// Corresponds to carla_msgs__srv__DestroyObject_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DestroyObject_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub id: i32,

}



impl Default for DestroyObject_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__srv__DestroyObject_Request__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__srv__DestroyObject_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DestroyObject_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__DestroyObject_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__DestroyObject_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__DestroyObject_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DestroyObject_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DestroyObject_Request where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/srv/DestroyObject_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__DestroyObject_Request() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__DestroyObject_Response() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__srv__DestroyObject_Response__init(msg: *mut DestroyObject_Response) -> bool;
    fn carla_msgs__srv__DestroyObject_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DestroyObject_Response>, size: usize) -> bool;
    fn carla_msgs__srv__DestroyObject_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DestroyObject_Response>);
    fn carla_msgs__srv__DestroyObject_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DestroyObject_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DestroyObject_Response>) -> bool;
}

// Corresponds to carla_msgs__srv__DestroyObject_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DestroyObject_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for DestroyObject_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__srv__DestroyObject_Response__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__srv__DestroyObject_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DestroyObject_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__DestroyObject_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__DestroyObject_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__DestroyObject_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DestroyObject_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DestroyObject_Response where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/srv/DestroyObject_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__DestroyObject_Response() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__GetBlueprints_Request() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__srv__GetBlueprints_Request__init(msg: *mut GetBlueprints_Request) -> bool;
    fn carla_msgs__srv__GetBlueprints_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetBlueprints_Request>, size: usize) -> bool;
    fn carla_msgs__srv__GetBlueprints_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetBlueprints_Request>);
    fn carla_msgs__srv__GetBlueprints_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetBlueprints_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GetBlueprints_Request>) -> bool;
}

// Corresponds to carla_msgs__srv__GetBlueprints_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetBlueprints_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filter: rosidl_runtime_rs::String,

}



impl Default for GetBlueprints_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__srv__GetBlueprints_Request__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__srv__GetBlueprints_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetBlueprints_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__GetBlueprints_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__GetBlueprints_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__GetBlueprints_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetBlueprints_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetBlueprints_Request where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/srv/GetBlueprints_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__GetBlueprints_Request() }
  }
}


#[link(name = "carla_msgs__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__GetBlueprints_Response() -> *const std::ffi::c_void;
}

#[link(name = "carla_msgs__rosidl_generator_c")]
extern "C" {
    fn carla_msgs__srv__GetBlueprints_Response__init(msg: *mut GetBlueprints_Response) -> bool;
    fn carla_msgs__srv__GetBlueprints_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GetBlueprints_Response>, size: usize) -> bool;
    fn carla_msgs__srv__GetBlueprints_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GetBlueprints_Response>);
    fn carla_msgs__srv__GetBlueprints_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GetBlueprints_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GetBlueprints_Response>) -> bool;
}

// Corresponds to carla_msgs__srv__GetBlueprints_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GetBlueprints_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub blueprints: rosidl_runtime_rs::Sequence<rosidl_runtime_rs::String>,

}



impl Default for GetBlueprints_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !carla_msgs__srv__GetBlueprints_Response__init(&mut msg as *mut _) {
        panic!("Call to carla_msgs__srv__GetBlueprints_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GetBlueprints_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__GetBlueprints_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__GetBlueprints_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { carla_msgs__srv__GetBlueprints_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GetBlueprints_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GetBlueprints_Response where Self: Sized {
  const TYPE_NAME: &'static str = "carla_msgs/srv/GetBlueprints_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__carla_msgs__srv__GetBlueprints_Response() }
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


