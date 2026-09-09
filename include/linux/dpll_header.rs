/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Meta Platforms, Inc. and affiliates
 * Copyright (c) 2023 Intel and affiliates
 */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;
pub type c_ulong = usize;

pub enum dpll_device {}
pub enum dpll_pin {}
pub enum fwnode_handle {}
pub enum ref_tracker {}
pub enum netlink_ext_ack {}
pub enum net_device {}
pub enum sk_buff {}
pub enum module {}
pub enum notifier_block {}

#[repr(C)]
pub struct dpll_device_ops {
    pub mode_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut dpll_mode, *mut netlink_ext_ack) -> i32>,
    pub mode_set: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, dpll_mode, *mut netlink_ext_ack) -> i32>,
    pub supported_modes_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut c_ulong, *mut netlink_ext_ack) -> i32>,
    pub lock_status_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut dpll_lock_status, *mut dpll_lock_status_error, *mut netlink_ext_ack) -> i32>,
    pub temp_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut i32, *mut netlink_ext_ack) -> i32>,
    pub clock_quality_level_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut c_ulong, *mut netlink_ext_ack) -> i32>,
    pub phase_offset_monitor_set: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, dpll_feature_state, *mut netlink_ext_ack) -> i32>,
    pub phase_offset_monitor_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut dpll_feature_state, *mut netlink_ext_ack) -> i32>,
    pub phase_offset_avg_factor_set: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, u32, *mut netlink_ext_ack) -> i32>,
    pub phase_offset_avg_factor_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut u32, *mut netlink_ext_ack) -> i32>,
    pub freq_monitor_set: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, dpll_feature_state, *mut netlink_ext_ack) -> i32>,
    pub freq_monitor_get: Option<unsafe extern "C" fn(*const dpll_device, *mut c_void, *mut dpll_feature_state, *mut netlink_ext_ack) -> i32>,
}

#[repr(C)]
pub enum dpll_ffo_type { DPLL_FFO_PORT_RXTX_RATE, DPLL_FFO_PIN_DEVICE, __DPLL_FFO_TYPE_MAX }

#[repr(C)]
pub struct dpll_ffo_param { pub type_: dpll_ffo_type, pub ffo: i64 }

#[repr(C)]
pub struct dpll_pin_ops {
    pub supported_ffo: c_ulong,
    pub frequency_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, u64, *mut netlink_ext_ack) -> i32>,
    pub frequency_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut u64, *mut netlink_ext_ack) -> i32>,
    pub direction_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, dpll_pin_direction, *mut netlink_ext_ack) -> i32>,
    pub direction_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut dpll_pin_direction, *mut netlink_ext_ack) -> i32>,
    pub state_on_pin_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_pin, *mut c_void, *mut dpll_pin_state, *mut netlink_ext_ack) -> i32>,
    pub state_on_dpll_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut dpll_pin_state, *mut netlink_ext_ack) -> i32>,
    pub operstate_on_dpll_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut dpll_pin_operstate, *mut netlink_ext_ack) -> i32>,
    pub state_on_pin_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_pin, *mut c_void, dpll_pin_state, *mut netlink_ext_ack) -> i32>,
    pub state_on_dpll_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, dpll_pin_state, *mut netlink_ext_ack) -> i32>,
    pub prio_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut u32, *mut netlink_ext_ack) -> i32>,
    pub prio_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, u32, *mut netlink_ext_ack) -> i32>,
    pub phase_offset_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut i64, *mut netlink_ext_ack) -> i32>,
    pub phase_adjust_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut i32, *mut netlink_ext_ack) -> i32>,
    pub phase_adjust_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, i32, *mut netlink_ext_ack) -> i32>,
    pub ffo_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut dpll_ffo_param, *mut netlink_ext_ack) -> i32>,
    pub measured_freq_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut u64, *mut netlink_ext_ack) -> i32>,
    pub esync_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, u64, *mut netlink_ext_ack) -> i32>,
    pub esync_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_device, *mut c_void, *mut dpll_pin_esync, *mut netlink_ext_ack) -> i32>,
    pub ref_sync_set: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_pin, *mut c_void, dpll_pin_state, *mut netlink_ext_ack) -> i32>,
    pub ref_sync_get: Option<unsafe extern "C" fn(*const dpll_pin, *mut c_void, *const dpll_pin, *mut c_void, *mut dpll_pin_state, *mut netlink_ext_ack) -> i32>,
}

#[repr(C)] pub struct dpll_pin_frequency { pub min: u64, pub max: u64 }
#[macro_export] macro_rules! DPLL_PIN_FREQUENCY_RANGE { ($min:expr, $max:expr) => { dpll_pin_frequency { min: $min, max: $max } }; }
#[macro_export] macro_rules! DPLL_PIN_FREQUENCY { ($val:expr) => { DPLL_PIN_FREQUENCY_RANGE!($val, $val) }; }
#[macro_export] macro_rules! DPLL_PIN_FREQUENCY_1PPS { () => { DPLL_PIN_FREQUENCY!(DPLL_PIN_FREQUENCY_1_HZ) }; }
#[macro_export] macro_rules! DPLL_PIN_FREQUENCY_10MHZ { () => { DPLL_PIN_FREQUENCY!(DPLL_PIN_FREQUENCY_10_MHZ) }; }
#[macro_export] macro_rules! DPLL_PIN_FREQUENCY_IRIG_B { () => { DPLL_PIN_FREQUENCY!(DPLL_PIN_FREQUENCY_10_KHZ) }; }
#[macro_export] macro_rules! DPLL_PIN_FREQUENCY_DCF77 { () => { DPLL_PIN_FREQUENCY!(DPLL_PIN_FREQUENCY_77_5_KHZ) }; }

#[repr(C)] pub struct dpll_pin_phase_adjust_range { pub min: i32, pub max: i32 }
#[repr(C)] pub struct dpll_pin_esync { pub freq: u64, pub range: *const dpll_pin_frequency, pub range_num: u8, pub pulse: u8 }
#[repr(C)] pub struct dpll_pin_properties {
    pub board_label: *const u8, pub panel_label: *const u8, pub package_label: *const u8,
    pub type_: dpll_pin_type, pub capabilities: c_ulong, pub freq_supported_num: u32,
    pub freq_supported: *mut dpll_pin_frequency, pub phase_range: dpll_pin_phase_adjust_range, pub phase_gran: u32,
}

#[cfg(feature = "CONFIG_DPLL_REFCNT_TRACKER")]
pub type dpll_tracker = *mut ref_tracker;
#[cfg(not(feature = "CONFIG_DPLL_REFCNT_TRACKER"))]
#[repr(C)] pub struct dpll_tracker;

pub const DPLL_DEVICE_CREATED: u32 = 1;
pub const DPLL_DEVICE_DELETED: u32 = 2;
pub const DPLL_DEVICE_CHANGED: u32 = 3;
pub const DPLL_PIN_CREATED: u32 = 4;
pub const DPLL_PIN_DELETED: u32 = 5;
pub const DPLL_PIN_CHANGED: u32 = 6;

#[repr(C)] pub struct dpll_device_notifier_info { pub dpll: *mut dpll_device, pub id: u32, pub idx: u32, pub clock_id: u64, pub type_: dpll_type }
#[repr(C)] pub struct dpll_pin_notifier_info { pub pin: *mut dpll_pin, pub id: u32, pub idx: u32, pub clock_id: u64, pub fwnode: *const fwnode_handle, pub prop: *const dpll_pin_properties, pub src_clock_id: u64 }

#[cfg(feature = "CONFIG_DPLL")]
extern "C" {
    pub fn dpll_netdev_pin_set(dev: *mut net_device, dpll_pin: *mut dpll_pin);
    pub fn dpll_netdev_pin_clear(dev: *mut net_device);
    pub fn dpll_netdev_pin_handle_size() -> usize;
    pub fn dpll_netdev_add_pin_handle(msg: *mut sk_buff, dev: *const net_device) -> i32;
    pub fn fwnode_dpll_pin_find(fwnode: *mut fwnode_handle, tracker: *mut dpll_tracker) -> *mut dpll_pin;
}
#[cfg(not(feature = "CONFIG_DPLL"))]
pub unsafe fn dpll_netdev_pin_set(_dev: *mut net_device, _dpll_pin: *mut dpll_pin) {}
#[cfg(not(feature = "CONFIG_DPLL"))]
pub unsafe fn dpll_netdev_pin_clear(_dev: *mut net_device) {}
#[cfg(not(feature = "CONFIG_DPLL"))]
pub const fn dpll_netdev_pin_handle_size() -> usize { 0 }
#[cfg(not(feature = "CONFIG_DPLL"))]
pub unsafe fn dpll_netdev_add_pin_handle(_msg: *mut sk_buff, _dev: *const net_device) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_DPLL"))]
pub unsafe fn fwnode_dpll_pin_find(_fwnode: *mut fwnode_handle, _tracker: *mut dpll_tracker) -> *mut dpll_pin { core::ptr::null_mut() }

extern "C" {
    pub fn dpll_device_get(clock_id: u64, dev_driver_id: u32, module: *mut module, tracker: *mut dpll_tracker) -> *mut dpll_device;
    pub fn dpll_device_put(dpll: *mut dpll_device, tracker: *mut dpll_tracker);
    pub fn dpll_device_register(dpll: *mut dpll_device, type_: dpll_type, ops: *const dpll_device_ops, priv_: *mut c_void) -> i32;
    pub fn dpll_device_unregister(dpll: *mut dpll_device, ops: *const dpll_device_ops, priv_: *mut c_void);
    pub fn dpll_pin_get(clock_id: u64, dev_driver_id: u32, module: *mut module, prop: *const dpll_pin_properties, tracker: *mut dpll_tracker) -> *mut dpll_pin;
    pub fn dpll_pin_register(dpll: *mut dpll_device, pin: *mut dpll_pin, ops: *const dpll_pin_ops, priv_: *mut c_void) -> i32;
    pub fn dpll_pin_unregister(dpll: *mut dpll_device, pin: *mut dpll_pin, ops: *const dpll_pin_ops, priv_: *mut c_void);
    pub fn dpll_pin_put(pin: *mut dpll_pin, tracker: *mut dpll_tracker);
    pub fn dpll_pin_fwnode_set(pin: *mut dpll_pin, fwnode: *mut fwnode_handle);
    pub fn dpll_pin_on_pin_register(parent: *mut dpll_pin, pin: *mut dpll_pin, ops: *const dpll_pin_ops, priv_: *mut c_void) -> i32;
    pub fn dpll_pin_on_pin_unregister(parent: *mut dpll_pin, pin: *mut dpll_pin, ops: *const dpll_pin_ops, priv_: *mut c_void);
    pub fn dpll_pin_ref_sync_pair_add(pin: *mut dpll_pin, ref_sync_pin: *mut dpll_pin) -> i32;
    pub fn __dpll_device_change_ntf(dpll: *mut dpll_device) -> i32;
    pub fn dpll_device_change_ntf(dpll: *mut dpll_device) -> i32;
    pub fn __dpll_pin_change_ntf(pin: *mut dpll_pin) -> i32;
    pub fn dpll_pin_change_ntf(pin: *mut dpll_pin) -> i32;
    pub fn register_dpll_notifier(nb: *mut notifier_block) -> i32;
    pub fn unregister_dpll_notifier(nb: *mut notifier_block) -> i32;
}

pub const DPLL_PIN_IDX_UNSPEC: u32 = u32::MAX;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
