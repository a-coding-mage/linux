/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (c) 2023 Meta Platforms, Inc. and affiliates
 *  Copyright (c) 2023 Intel and affiliates
 */

// Translated from dpll_core.h. Types and constants supplied by the included
// kernel headers and dpll_nl.h are intentionally left as external dependencies.

use core::ffi::{c_char, c_int, c_ulong, c_void};

pub const DPLL_REGISTERED: u32 = XA_MARK_1;

#[repr(C)]
pub struct dpll_device {
    pub id: u32,
    pub device_idx: u32,
    pub clock_id: u64,
    pub module: *mut module,
    pub type_: dpll_type,
    pub pin_refs: xarray,
    pub refcount: refcount_t,
    pub refcnt_tracker: ref_tracker_dir,
    pub registration_list: list_head,
}

#[repr(C)]
pub struct dpll_pin {
    pub id: u32,
    pub pin_idx: u32,
    pub clock_id: u64,
    pub module: *mut module,
    pub module_name: [c_char; MODULE_NAME_LEN],
    pub fwnode: *mut fwnode_handle,
    pub dpll_refs: xarray,
    pub parent_refs: xarray,
    pub ref_sync_pins: xarray,
    pub prop: dpll_pin_properties,
    pub refcount: refcount_t,
    pub refcnt_tracker: ref_tracker_dir,
    pub rcu: rcu_head,
}

#[repr(C)]
pub union dpll_pin_ref_target {
    pub dpll: *mut dpll_device,
    pub pin: *mut dpll_pin,
}

#[repr(C)]
pub struct dpll_pin_ref {
    pub target: dpll_pin_ref_target,
    pub registration_list: list_head,
    pub refcount: refcount_t,
}

unsafe extern "C" {
    pub fn dpll_priv(dpll: *mut dpll_device) -> *mut c_void;
    pub fn dpll_pin_on_dpll_priv(
        dpll: *mut dpll_device,
        pin: *mut dpll_pin,
    ) -> *mut c_void;
    pub fn dpll_pin_on_pin_priv(
        parent: *mut dpll_pin,
        pin: *mut dpll_pin,
    ) -> *mut c_void;

    pub fn dpll_device_ops(dpll: *mut dpll_device) -> *const dpll_device_ops;
    pub fn dpll_device_get_by_id(id: c_int) -> *mut dpll_device;
    pub fn dpll_pin_own_dpll_ref_first(pin: *mut dpll_pin) -> *mut dpll_pin_ref;
    pub fn dpll_pin_ops(r#ref: *mut dpll_pin_ref) -> *const dpll_pin_ops;
    pub fn dpll_xa_ref_dpll_first(xa_refs: *mut xarray) -> *mut dpll_pin_ref;

    pub static mut dpll_device_xa: xarray;
    pub static mut dpll_pin_xa: xarray;
    pub static mut dpll_lock: mutex;

    pub fn dpll_device_notify(dpll: *mut dpll_device, action: c_ulong);
    pub fn dpll_pin_notify(pin: *mut dpll_pin, src_clock_id: u64, action: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
