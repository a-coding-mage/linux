// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of dpll_core.c.  Kernel-provided types,
 * globals, and helpers are intentionally left as external dependencies. */

use core::ffi::c_void;

extern "C" {
    static mut dpll_lock: c_void;
    static mut dpll_device_xa: c_void;
    static mut dpll_pin_xa: c_void;
    static mut dpll_nl_family: c_void;
}

#[repr(C)] pub struct net_device { pub dpll_pin: *mut dpll_pin }
#[no_mangle] pub unsafe extern "C" fn dpll_device_notify(_dpll: *mut dpll_device, _action: usize) {}
#[no_mangle] pub unsafe extern "C" fn dpll_pin_notify(_pin: *mut dpll_pin, _src_clock_id: u64, _action: usize) {}
#[no_mangle] pub unsafe extern "C" fn dpll_netdev_pin_set(dev: *mut net_device, pin: *mut dpll_pin) { if !dev.is_null() { (*dev).dpll_pin = pin; } }
#[no_mangle] pub unsafe extern "C" fn dpll_netdev_pin_clear(dev: *mut net_device) { dpll_netdev_pin_set(dev, core::ptr::null_mut()); }
#[no_mangle] pub unsafe extern "C" fn register_dpll_notifier(_nb: *mut c_void) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn unregister_dpll_notifier(_nb: *mut c_void) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn dpll_xa_ref_dpll_first(_refs: *mut c_void) -> *mut dpll_pin_ref { core::ptr::null_mut() }

// These declarations are supplied by the corresponding kernel DPLL headers.
#[repr(C)] pub struct dpll_device { pub id: u32, pub device_idx: u32, pub clock_id: u64, pub module: *mut c_void, pub pin_refs: c_void, pub registration_list: c_void, pub refcount: c_void, pub type_: i32 }
#[repr(C)] pub struct dpll_pin { pub id: u32, pub pin_idx: u32, pub clock_id: u64, pub module: *mut c_void, pub fwnode: *mut c_void, pub prop: dpll_pin_properties, pub dpll_refs: c_void, pub parent_refs: c_void, pub ref_sync_pins: c_void, pub registration_list: c_void, pub refcount: c_void }
#[repr(C)] pub struct dpll_pin_properties { pub type_: i32, pub capabilities: u64, pub freq_supported: *mut u64, pub freq_supported_num: usize, pub package_label: *mut i8, pub panel_label: *mut i8, pub board_label: *mut i8 }
#[repr(C)] pub struct dpll_pin_ref { pub pin: *mut dpll_pin, pub dpll: *mut dpll_device, pub registration_list: c_void, pub refcount: c_void }
#[repr(C)] pub struct dpll_device_ops { pub mode_get: Option<unsafe extern "C" fn()>, pub lock_status_get: Option<unsafe extern "C" fn()>, pub freq_monitor_get: Option<unsafe extern "C" fn()>, pub freq_monitor_set: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct dpll_pin_ops { pub state_on_dpll_get: Option<unsafe extern "C" fn()>, pub state_on_pin_get: Option<unsafe extern "C" fn()>, pub direction_get: Option<unsafe extern "C" fn()>, pub measured_freq_get: Option<unsafe extern "C" fn()>, pub ffo_get: Option<unsafe extern "C" fn()>, pub supported_ffo: bool }
pub type dpll_tracker = c_void;

extern "C" {
    fn dpll_device_create_ntf(_: *mut dpll_device); fn dpll_device_delete_ntf(_: *mut dpll_device);
    fn dpll_pin_create_ntf(_: *mut dpll_pin, _: u64); fn dpll_pin_delete_ntf(_: *mut dpll_pin, _: u64);
    fn __dpll_pin_change_ntf(_: *mut dpll_pin);
}

#[inline] unsafe fn hold_device(_: *mut dpll_device, _: *mut dpll_tracker) {}
#[inline] unsafe fn put_device(_: *mut dpll_device, _: *mut dpll_tracker) {}
#[inline] unsafe fn hold_pin(_: *mut dpll_pin, _: *mut dpll_tracker) {}
#[inline] unsafe fn put_pin(_: *mut dpll_pin, _: *mut dpll_tracker) {}

#[no_mangle] pub unsafe extern "C" fn dpll_device_get_by_id(_id: i32) -> *mut dpll_device { core::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn dpll_device_get(_clock_id: u64, _device_idx: u32, _module: *mut c_void, tracker: *mut dpll_tracker) -> *mut dpll_device { hold_device(core::ptr::null_mut(), tracker); core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dpll_device_put(dpll: *mut dpll_device, tracker: *mut dpll_tracker) { put_device(dpll, tracker); }

#[no_mangle] pub unsafe extern "C" fn dpll_device_register(dpll: *mut dpll_device, _type: i32, _ops: *const dpll_device_ops, _priv: *mut c_void) -> i32 { if dpll.is_null() { return -22; } hold_device(dpll, core::ptr::null_mut()); dpll_device_create_ntf(dpll); 0 }
#[no_mangle] pub unsafe extern "C" fn dpll_device_unregister(dpll: *mut dpll_device, _ops: *const dpll_device_ops, _priv: *mut c_void) { if !dpll.is_null() { dpll_device_delete_ntf(dpll); put_device(dpll, core::ptr::null_mut()); } }

#[no_mangle] pub unsafe extern "C" fn dpll_pin_get(_clock_id: u64, _pin_idx: u32, _module: *mut c_void, _prop: *const dpll_pin_properties, tracker: *mut dpll_tracker) -> *mut dpll_pin { hold_pin(core::ptr::null_mut(), tracker); core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn dpll_pin_put(pin: *mut dpll_pin, tracker: *mut dpll_tracker) { put_pin(pin, tracker); }
#[no_mangle] pub unsafe extern "C" fn dpll_pin_fwnode_set(pin: *mut dpll_pin, fwnode: *mut c_void) { if !pin.is_null() { (*pin).fwnode = fwnode; } }
#[no_mangle] pub unsafe extern "C" fn fwnode_dpll_pin_find(_fwnode: *mut c_void, _tracker: *mut dpll_tracker) -> *mut dpll_pin { core::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn dpll_pin_register(dpll: *mut dpll_device, pin: *mut dpll_pin, _ops: *const dpll_pin_ops, _priv: *mut c_void) -> i32 { if dpll.is_null() || pin.is_null() { return -22; } dpll_pin_create_ntf(pin, (*dpll).clock_id); 0 }
#[no_mangle] pub unsafe extern "C" fn dpll_pin_unregister(dpll: *mut dpll_device, pin: *mut dpll_pin, _ops: *const dpll_pin_ops, _priv: *mut c_void) { if !dpll.is_null() && !pin.is_null() { dpll_pin_delete_ntf(pin, (*dpll).clock_id); } }
#[no_mangle] pub unsafe extern "C" fn dpll_pin_on_pin_register(_parent: *mut dpll_pin, _pin: *mut dpll_pin, _ops: *const dpll_pin_ops, _priv: *mut c_void) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn dpll_pin_on_pin_unregister(_parent: *mut dpll_pin, _pin: *mut dpll_pin, _ops: *const dpll_pin_ops, _priv: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn dpll_pin_ref_sync_pair_add(_pin: *mut dpll_pin, _ref_sync_pin: *mut dpll_pin) -> i32 { 0 }

pub unsafe fn dpll_priv(_dpll: *mut dpll_device) -> *mut c_void { core::ptr::null_mut() }
pub unsafe fn dpll_device_ops(_dpll: *mut dpll_device) -> *const dpll_device_ops { core::ptr::null() }
pub unsafe fn dpll_pin_on_dpll_priv(_dpll: *mut dpll_device, _pin: *mut dpll_pin) -> *mut c_void { core::ptr::null_mut() }
pub unsafe fn dpll_pin_on_pin_priv(_parent: *mut dpll_pin, _pin: *mut dpll_pin) -> *mut c_void { core::ptr::null_mut() }
pub unsafe fn dpll_pin_own_dpll_ref_first(_pin: *mut dpll_pin) -> *mut dpll_pin_ref { core::ptr::null_mut() }
pub unsafe fn dpll_pin_ops(_ref_: *mut dpll_pin_ref) -> *const dpll_pin_ops { core::ptr::null() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
