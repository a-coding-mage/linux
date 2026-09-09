// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Sensor Protocol */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::c_void;

pub const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x30001;
pub const SCMI_MAX_NUM_SENSOR_AXIS: usize = 63;
pub const SCMIv2_SENSOR_PROTOCOL: u32 = 0x10000;

pub const SENSOR_DESCRIPTION_GET: u32 = 0x3;
pub const SENSOR_TRIP_POINT_NOTIFY: u32 = 0x4;
pub const SENSOR_TRIP_POINT_CONFIG: u32 = 0x5;
pub const SENSOR_READING_GET: u32 = 0x6;
pub const SENSOR_AXIS_DESCRIPTION_GET: u32 = 0x7;
pub const SENSOR_LIST_UPDATE_INTERVALS: u32 = 0x8;
pub const SENSOR_CONFIG_GET: u32 = 0x9;
pub const SENSOR_CONFIG_SET: u32 = 0xa;
pub const SENSOR_CONTINUOUS_UPDATE_NOTIFY: u32 = 0xb;
pub const SENSOR_NAME_GET: u32 = 0xc;
pub const SENSOR_AXIS_NAME_GET: u32 = 0xd;

pub const SENSOR_NOTIFY_ALL: u32 = 1 << 0;
pub const SENSOR_TP_EVENT_MASK: u32 = 3;
pub const SENSOR_TP_DISABLED: u32 = 0;
pub const SENSOR_TP_POSITIVE: u32 = 1;
pub const SENSOR_TP_NEGATIVE: u32 = 2;
pub const SENSOR_TP_BOTH: u32 = 3;
pub const SENSOR_READ_ASYNC: u32 = 1;

#[inline] pub const fn field_get(mask: u32, x: u32) -> u32 { (x & mask) >> mask.trailing_zeros() }
#[inline] pub const fn SUPPORTS_UPDATE_NOTIFY(x: u32) -> u32 { field_get(1 << 30, x) }
#[inline] pub const fn SENSOR_TSTAMP_EXP(x: u32) -> u32 { field_get(0x7c00, x) }
#[inline] pub const fn SUPPORTS_TIMESTAMP(x: u32) -> u32 { field_get(1 << 9, x) }
#[inline] pub const fn SUPPORTS_EXTEND_ATTRS(x: u32) -> u32 { field_get(1 << 8, x) }
#[inline] pub const fn SENSOR_UPDATE_BASE(x: u32) -> u32 { field_get(0xf8000000, x) }
#[inline] pub const fn SENSOR_UPDATE_SCALE(x: u32) -> u32 { field_get(0x07c00000, x) }
#[inline] pub const fn SENSOR_AXIS_NUMBER(x: u32) -> u32 { field_get(0x003f0000, x) }
#[inline] pub const fn SUPPORTS_AXIS(x: u32) -> u32 { field_get(1 << 8, x) }
#[inline] pub const fn SENSOR_RES(x: u32) -> u32 { x & 0x07ffffff }
#[inline] pub const fn SENSOR_RES_EXP(x: u32) -> u32 { field_get(0xf8000000, x) }
#[inline] pub const fn SUPPORTS_ASYNC_READ(x: u32) -> u32 { x >> 31 }
#[inline] pub const fn SUPPORTS_EXTENDED_NAMES(x: u32) -> u32 { field_get(1 << 29, x) }
#[inline] pub const fn NUM_TRIP_POINTS(x: u32) -> u32 { x & 0xff }
#[inline] pub const fn SENSOR_SCALE(x: u32) -> u32 { field_get(0xf800, x) }
pub const SENSOR_SCALE_SIGN: u32 = 1 << 4;
pub const SENSOR_SCALE_EXTEND: u32 = 0xffffffe0;
#[inline] pub const fn SENSOR_TYPE(x: u32) -> u32 { x & 0xff }
#[inline] pub const fn S32_EXT(v: u32) -> i32 { if v & SENSOR_SCALE_SIGN != 0 { (v | SENSOR_SCALE_EXTEND) as i32 } else { v as i32 } }
#[inline] pub const fn SENSOR_TP_ID(x: u8) -> u32 { (x as u32 & 0xff) << 4 }

#[repr(C)] #[derive(Copy, Clone)] pub struct scmi_msg_resp_sensor_attributes { pub num_sensors: u16, pub max_requests: u8, pub reserved: u8, pub reg_addr_low: u32, pub reg_addr_high: u32, pub reg_size: u32 }
#[repr(C)] #[derive(Copy, Clone)] pub struct scmi_msg_resp_attrs { pub min_range_low:u32, pub min_range_high:u32, pub max_range_low:u32, pub max_range_high:u32 }
#[repr(C)] pub struct scmi_msg_sensor_description { pub desc_index:u32 }
#[repr(C)] pub struct scmi_sensor_descriptor { pub id:u32, pub attributes_low:u32, pub attributes_high:u32, pub name:[u8; SCMI_SHORT_NAME_MAX_SIZE], pub power:u32, pub resolution:u32, pub scalar_attrs:scmi_msg_resp_attrs }
#[repr(C)] pub struct scmi_msg_resp_sensor_description { pub num_returned:u16, pub num_remaining:u16, pub desc:[scmi_sensor_descriptor;0] }
#[repr(C)] pub struct scmi_msg_sensor_axis_description_get { pub id:u32, pub axis_desc_index:u32 }
#[repr(C)] pub struct scmi_axis_descriptor { pub id:u32, pub attributes_low:u32, pub attributes_high:u32, pub name:[u8; SCMI_SHORT_NAME_MAX_SIZE], pub resolution:u32, pub attrs:scmi_msg_resp_attrs }
#[repr(C)] pub struct scmi_msg_resp_sensor_axis_description { pub num_axis_flags:u32, pub desc:[scmi_axis_descriptor;0] }
#[repr(C)] pub struct scmi_sensor_axis_name_descriptor { pub axis_id:u32, pub name:[u8; SCMI_MAX_STR_SIZE] }
#[repr(C)] pub struct scmi_msg_resp_sensor_axis_names_description { pub num_axis_flags:u32, pub desc:[scmi_sensor_axis_name_descriptor;0] }
#[repr(C)] pub struct scmi_msg_sensor_list_update_intervals { pub id:u32, pub index:u32 }
#[repr(C)] pub struct scmi_msg_resp_sensor_list_update_intervals { pub num_intervals_flags:u32, pub intervals:[u32;0] }
#[repr(C)] pub struct scmi_msg_sensor_request_notify { pub id:u32, pub event_control:u32 }
#[repr(C)] pub struct scmi_msg_set_sensor_trip_point { pub id:u32, pub event_control:u32, pub value_low:u32, pub value_high:u32 }
#[repr(C)] pub struct scmi_msg_sensor_config_set { pub id:u32, pub sensor_config:u32 }
#[repr(C)] pub struct scmi_msg_sensor_reading_get { pub id:u32, pub flags:u32 }
#[repr(C)] pub struct scmi_resp_sensor_reading_complete { pub id:u32, pub readings_low:u32, pub readings_high:u32 }
#[repr(C)] pub struct scmi_sensor_reading_resp { pub sensor_value_low:u32, pub sensor_value_high:u32, pub timestamp_low:u32, pub timestamp_high:u32 }
#[repr(C)] pub struct scmi_resp_sensor_reading_complete_v3 { pub id:u32, pub readings:[scmi_sensor_reading_resp;0] }
#[repr(C)] pub struct scmi_sensor_trip_notify_payld { pub agent_id:u32, pub sensor_id:u32, pub trip_point_desc:u32 }
#[repr(C)] pub struct scmi_sensor_update_notify_payld { pub agent_id:u32, pub sensor_id:u32, pub readings:[scmi_sensor_reading_resp;0] }

// Types supplied by protocols.h/notify.h and the SCMI core remain external dependencies.
extern "C" {
    fn scmi_sensor_attributes_get(ph:*const scmi_protocol_handle, si:*mut sensors_info)->i32;
}
#[repr(C)] pub struct sensors_info { pub notify_trip_point_cmd:bool, pub notify_continuos_update_cmd:bool, pub num_sensors:i32, pub max_requests:i32, pub reg_addr:u64, pub reg_size:u32, pub sensors:*mut scmi_sensor_info }
#[repr(C)] pub struct scmi_sensor_info { pub id:u32, pub async_:bool, pub update:bool, pub timestamped:bool, pub num_trip_points:u32, pub tstamp_scale:i32, pub extended_scalar_attrs:bool, pub scale:i32, pub type_:u32, pub num_axis:u32, pub name:[u8; SCMI_MAX_STR_SIZE], pub sensor_power:u32, pub resolution:u32, pub exponent:i32, pub sensor_config:u32 }
#[repr(C)] pub struct scmi_protocol_handle { pub version:u32, pub dev:*mut c_void }
pub const SCMI_SHORT_NAME_MAX_SIZE:usize = 16;
pub const SCMI_MAX_STR_SIZE:usize = 64;
pub const SCMI_MAX_PREALLOC_POOL:usize = 16;
pub const SCMI_MSG_RESP_SENS_DESCR_BASE_SZ:usize = 28;
pub const SCMI_MSG_RESP_AXIS_DESCR_BASE_SZ:usize = 28;

/* The remaining protocol callbacks are intentionally represented as declarations:
 * their implementations and shared SCMI data structures are supplied by the core. */
extern "C" {
    fn scmi_sensor_description_get(ph:*const scmi_protocol_handle, si:*mut sensors_info)->i32;
    fn scmi_sensor_trip_point_config(ph:*const scmi_protocol_handle, sensor_id:u32, trip_id:u8, trip_value:u64)->i32;
    fn scmi_sensor_reading_get(ph:*const scmi_protocol_handle, sensor_id:u32, value:*mut u64)->i32;
    fn scmi_sensor_config_get(ph:*const scmi_protocol_handle, sensor_id:u32, config:*mut u32)->i32;
    fn scmi_sensor_config_set(ph:*const scmi_protocol_handle, sensor_id:u32, config:u32)->i32;
}

// Registration and event tables are populated by the SCMI integration layer.
#[repr(C)] pub struct scmi_sensor_proto_ops { pub count_get:Option<unsafe extern "C" fn(*const scmi_protocol_handle)->i32>, pub info_get:*const c_void, pub trip_point_config:*const c_void, pub reading_get:*const c_void, pub reading_get_timestamped:*const c_void, pub config_get:*const c_void, pub config_set:*const c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
