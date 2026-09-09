/* SPDX-License-Identifier: GPL-2.0 */
/*
 *
 * Generic netlink for energy model.
 *
 * Copyright (c) 2025 Valve Corporation.
 * Author: Changwoo Min <changwoo@igalia.com>
 */

// The declarations below are selected by the C build when both
// CONFIG_ENERGY_MODEL and CONFIG_NET are defined.

#[cfg(all(feature = "CONFIG_ENERGY_MODEL", feature = "CONFIG_NET"))]
extern "C" {
    pub fn for_each_em_perf_domain(
        cb: Option<unsafe extern "C" fn(*mut em_perf_domain, *mut core::ffi::c_void) -> i32>,
        data: *mut core::ffi::c_void,
    ) -> i32;
    pub fn em_perf_domain_get_by_id(id: i32) -> *mut em_perf_domain;
    pub fn em_notify_pd_created(pd: *const em_perf_domain);
    pub fn em_notify_pd_deleted(pd: *const em_perf_domain);
    pub fn em_notify_pd_updated(pd: *const em_perf_domain);
}

// Opaque external type supplied by the energy-model dependency.
#[repr(C)]
pub struct em_perf_domain {
    _private: [u8; 0],
}

// Fallback definitions used when CONFIG_ENERGY_MODEL or CONFIG_NET is absent.
#[cfg(not(all(feature = "CONFIG_ENERGY_MODEL", feature = "CONFIG_NET")))]
#[inline]
pub unsafe fn for_each_em_perf_domain(
    _cb: Option<unsafe extern "C" fn(*mut em_perf_domain, *mut core::ffi::c_void) -> i32>,
    _data: *mut core::ffi::c_void,
) -> i32 {
    -22 /* -EINVAL; supplied by the external errno definitions. */
}

#[cfg(not(all(feature = "CONFIG_ENERGY_MODEL", feature = "CONFIG_NET")))]
#[inline]
pub unsafe fn em_perf_domain_get_by_id(_id: i32) -> *mut em_perf_domain {
    core::ptr::null_mut()
}

#[cfg(not(all(feature = "CONFIG_ENERGY_MODEL", feature = "CONFIG_NET")))]
#[inline]
pub unsafe fn em_notify_pd_created(_pd: *const em_perf_domain) {}

#[cfg(not(all(feature = "CONFIG_ENERGY_MODEL", feature = "CONFIG_NET")))]
#[inline]
pub unsafe fn em_notify_pd_deleted(_pd: *const em_perf_domain) {}

#[cfg(not(all(feature = "CONFIG_ENERGY_MODEL", feature = "CONFIG_NET")))]
#[inline]
pub unsafe fn em_notify_pd_updated(_pd: *const em_perf_domain) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
