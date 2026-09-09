/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Miscellaneous cgroup controller.
 *
 * Copyright 2020 Google LLC
 * Author: Vipin Sharma <vipinsh@google.com>
 */

/**
 * enum misc_res_type - Types of misc cgroup entries supported by the host.
 */
#[repr(C)]
pub enum misc_res_type {
    #[cfg(CONFIG_KVM_AMD_SEV)]
    /// AMD SEV ASIDs resource
    MISC_CG_RES_SEV,
    #[cfg(CONFIG_KVM_AMD_SEV)]
    /// AMD SEV-ES ASIDs resource
    MISC_CG_RES_SEV_ES,
    #[cfg(CONFIG_INTEL_TDX_HOST)]
    /// Intel TDX HKIDs resource
    MISC_CG_RES_TDX,
    /// count of enum misc_res_type constants
    MISC_CG_RES_TYPES,
}

pub struct misc_cg;

#[cfg(CONFIG_CGROUP_MISC)]
pub struct misc_res {
    pub max: u64,
    pub watermark: atomic64_t,
    pub usage: atomic64_t,
    pub events: atomic64_t,
    pub events_local: atomic64_t,
}

#[cfg(CONFIG_CGROUP_MISC)]
#[repr(C)]
pub struct misc_cg {
    pub css: cgroup_subsys_state,
    pub events_file: cgroup_file,
    pub events_local_file: cgroup_file,
    pub res: [misc_res; MISC_CG_RES_TYPES as usize],
}

#[cfg(CONFIG_CGROUP_MISC)]
extern "C" {
    pub fn misc_cg_set_capacity(type_: misc_res_type, capacity: u64) -> i32;
    pub fn misc_cg_try_charge(type_: misc_res_type, cg: *mut misc_cg, amount: u64) -> i32;
    pub fn misc_cg_uncharge(type_: misc_res_type, cg: *mut misc_cg, amount: u64);
}

#[cfg(CONFIG_CGROUP_MISC)]
#[inline]
pub unsafe fn css_misc(css: *mut cgroup_subsys_state) -> *mut misc_cg {
    if !css.is_null() {
        container_of!(css, misc_cg, css)
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(CONFIG_CGROUP_MISC)]
#[inline]
pub unsafe fn get_current_misc_cg() -> *mut misc_cg {
    css_misc(task_get_css(current, misc_cgrp_id))
}

#[cfg(CONFIG_CGROUP_MISC)]
#[inline]
pub unsafe fn put_misc_cg(cg: *mut misc_cg) {
    if !cg.is_null() {
        css_put(&mut (*cg).css);
    }
}

#[cfg(not(CONFIG_CGROUP_MISC))]
#[inline]
pub fn misc_cg_set_capacity(_type_: misc_res_type, _capacity: u64) -> i32 {
    0
}

#[cfg(not(CONFIG_CGROUP_MISC))]
#[inline]
pub fn misc_cg_try_charge(_type_: misc_res_type, _cg: *mut misc_cg, _amount: u64) -> i32 {
    0
}

#[cfg(not(CONFIG_CGROUP_MISC))]
#[inline]
pub fn misc_cg_uncharge(_type_: misc_res_type, _cg: *mut misc_cg, _amount: u64) {}

#[cfg(not(CONFIG_CGROUP_MISC))]
#[inline]
pub fn get_current_misc_cg() -> *mut misc_cg {
    core::ptr::null_mut()
}

#[cfg(not(CONFIG_CGROUP_MISC))]
#[inline]
pub fn put_misc_cg(_cg: *mut misc_cg) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
