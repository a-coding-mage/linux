/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 Linaro Ltd
 *
 * Author: Daniel Lezcano <daniel.lezcano@linaro.org>
 */

// Dependency supplied by the Linux powercap interfaces.

pub const MAX_DTPM_DESCR: usize = 8;
pub const MAX_DTPM_CONSTRAINTS: usize = 1;

#[repr(C)]
pub struct dtpm {
    pub zone: powercap_zone,
    pub parent: *mut dtpm,
    pub sibling: list_head,
    pub children: list_head,
    pub ops: *mut dtpm_ops,
    pub flags: ::core::ffi::c_ulong,
    pub power_limit: u64,
    pub power_max: u64,
    pub power_min: u64,
    pub weight: ::core::ffi::c_int,
}

#[repr(C)]
pub struct dtpm_ops {
    pub set_power_uw: Option<unsafe extern "C" fn(*mut dtpm, u64) -> u64>,
    pub get_power_uw: Option<unsafe extern "C" fn(*mut dtpm) -> u64>,
    pub update_power_uw: Option<unsafe extern "C" fn(*mut dtpm) -> ::core::ffi::c_int>,
    pub release: Option<unsafe extern "C" fn(*mut dtpm)>,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dtpm_subsys_ops {
    pub name: *const ::core::ffi::c_char,
    pub init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    pub exit: Option<unsafe extern "C" fn()>,
    pub setup: Option<unsafe extern "C" fn(*mut dtpm, *mut device_node) -> ::core::ffi::c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DTPM_NODE_TYPE {
    DTPM_NODE_VIRTUAL = 0,
    DTPM_NODE_DT,
}

#[repr(C)]
pub struct dtpm_node {
    pub type_: DTPM_NODE_TYPE,
    pub name: *const ::core::ffi::c_char,
    pub parent: *mut dtpm_node,
}

#[inline]
pub unsafe fn to_dtpm(zone: *mut powercap_zone) -> *mut dtpm {
    // Equivalent to Linux's container_of(zone, struct dtpm, zone).
    zone.cast::<u8>().sub(core::mem::offset_of!(dtpm, zone)).cast::<dtpm>()
}

unsafe extern "C" {
    pub fn dtpm_update_power(dtpm: *mut dtpm) -> ::core::ffi::c_int;
    pub fn dtpm_release_zone(pcz: *mut powercap_zone) -> ::core::ffi::c_int;
    pub fn dtpm_init(dtpm: *mut dtpm, ops: *mut dtpm_ops);
    pub fn dtpm_unregister(dtpm: *mut dtpm);
    pub fn dtpm_register(
        name: *const ::core::ffi::c_char,
        dtpm: *mut dtpm,
        parent: *mut dtpm,
    ) -> ::core::ffi::c_int;
    pub fn dtpm_create_hierarchy(dtpm_match_table: *mut of_device_id) -> ::core::ffi::c_int;
    pub fn dtpm_destroy_hierarchy();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
