/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * powercap.h: Data types and headers for sysfs power capping interface
 * Copyright (c) 2013, Intel Corporation.
 *
 * C dependencies (<linux/device.h> and <linux/idr.h>) are supplied by other
 * translated headers.
 */

pub struct powercap_control_type;
pub struct powercap_zone;
pub struct powercap_zone_constraint;

#[repr(C)]
pub struct powercap_control_type_ops {
    pub set_enable: Option<unsafe extern "C" fn(*mut powercap_control_type, bool) -> i32>,
    pub get_enable: Option<unsafe extern "C" fn(*mut powercap_control_type, *mut bool) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut powercap_control_type) -> i32>,
}

#[repr(C)]
pub struct powercap_control_type {
    pub dev: device,
    pub idr: idr,
    pub nr_zones: i32,
    pub ops: *const powercap_control_type_ops,
    pub lock: mutex,
    pub allocated: bool,
    pub node: list_head,
}

#[repr(C)]
pub struct powercap_zone_ops {
    pub get_max_energy_range_uj: Option<unsafe extern "C" fn(*mut powercap_zone, *mut u64) -> i32>,
    pub get_energy_uj: Option<unsafe extern "C" fn(*mut powercap_zone, *mut u64) -> i32>,
    pub reset_energy_uj: Option<unsafe extern "C" fn(*mut powercap_zone) -> i32>,
    pub get_max_power_range_uw: Option<unsafe extern "C" fn(*mut powercap_zone, *mut u64) -> i32>,
    pub get_power_uw: Option<unsafe extern "C" fn(*mut powercap_zone, *mut u64) -> i32>,
    pub set_enable: Option<unsafe extern "C" fn(*mut powercap_zone, bool) -> i32>,
    pub get_enable: Option<unsafe extern "C" fn(*mut powercap_zone, *mut bool) -> i32>,
    pub release: Option<unsafe extern "C" fn(*mut powercap_zone) -> i32>,
}

pub const POWERCAP_ZONE_MAX_ATTRS: i32 = 6;
pub const POWERCAP_CONSTRAINTS_ATTRS: i32 = 8;
pub const MAX_CONSTRAINTS_PER_ZONE: i32 = 10;

#[repr(C)]
pub struct powercap_zone {
    pub id: i32,
    pub name: *mut i8,
    pub control_type_inst: *mut core::ffi::c_void,
    pub ops: *const powercap_zone_ops,
    pub dev: device,
    pub const_id_cnt: i32,
    pub idr: idr,
    pub parent_idr: *mut idr,
    pub private_data: *mut core::ffi::c_void,
    pub zone_dev_attrs: *mut *mut attribute,
    pub zone_attr_count: i32,
    pub dev_zone_attr_group: attribute_group,
    pub dev_attr_groups: [*const attribute_group; 2],
    pub allocated: bool,
    pub constraints: *mut powercap_zone_constraint,
}

#[repr(C)]
pub struct powercap_zone_constraint_ops {
    pub set_power_limit_uw: Option<unsafe extern "C" fn(*mut powercap_zone, i32, u64) -> i32>,
    pub get_power_limit_uw: Option<unsafe extern "C" fn(*mut powercap_zone, i32, *mut u64) -> i32>,
    pub set_time_window_us: Option<unsafe extern "C" fn(*mut powercap_zone, i32, u64) -> i32>,
    pub get_time_window_us: Option<unsafe extern "C" fn(*mut powercap_zone, i32, *mut u64) -> i32>,
    pub get_max_power_uw: Option<unsafe extern "C" fn(*mut powercap_zone, i32, *mut u64) -> i32>,
    pub get_min_power_uw: Option<unsafe extern "C" fn(*mut powercap_zone, i32, *mut u64) -> i32>,
    pub get_max_time_window_us: Option<unsafe extern "C" fn(*mut powercap_zone, i32, *mut u64) -> i32>,
    pub get_min_time_window_us: Option<unsafe extern "C" fn(*mut powercap_zone, i32, *mut u64) -> i32>,
    pub get_name: Option<unsafe extern "C" fn(*mut powercap_zone, i32) -> *const i8>,
}

#[repr(C)]
pub struct powercap_zone_constraint {
    pub id: i32,
    pub power_zone: *mut powercap_zone,
    pub ops: *const powercap_zone_constraint_ops,
}

#[inline]
pub unsafe fn POWERCAP_GET_DEV(power_zone: *mut powercap_zone) -> *mut device {
    &mut (*power_zone).dev
}

#[inline]
pub unsafe fn powercap_set_zone_data(power_zone: *mut powercap_zone, pdata: *mut core::ffi::c_void) {
    if !power_zone.is_null() {
        (*power_zone).private_data = pdata;
    }
}

#[inline]
pub unsafe fn powercap_get_zone_data(power_zone: *mut powercap_zone) -> *mut core::ffi::c_void {
    if !power_zone.is_null() { (*power_zone).private_data } else { core::ptr::null_mut() }
}

extern "C" {
    pub fn powercap_register_control_type(control_type: *mut powercap_control_type, name: *const i8, ops: *const powercap_control_type_ops) -> *mut powercap_control_type;
    pub fn powercap_unregister_control_type(instance: *mut powercap_control_type) -> i32;
    pub fn powercap_register_zone(power_zone: *mut powercap_zone, control_type: *mut powercap_control_type, name: *const i8, parent: *mut powercap_zone, ops: *const powercap_zone_ops, nr_constraints: i32, const_ops: *const powercap_zone_constraint_ops) -> *mut powercap_zone;
    pub fn powercap_unregister_zone(control_type: *mut powercap_control_type, power_zone: *mut powercap_zone) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
