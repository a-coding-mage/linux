/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010, Intel Corporation.
 *
 * Author: John Fastabend <john.r.fastabend@intel.com>
 */

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub enum dcbevent_notif_type {
    DCB_APP_EVENT = 1,
}

// CONFIG_DCB is a build-time condition from the original header.
#[cfg(feature = "CONFIG_DCB")]
unsafe extern "C" {
    pub fn register_dcbevent_notifier(nb: *mut notifier_block) -> ::std::os::raw::c_int;
    pub fn unregister_dcbevent_notifier(nb: *mut notifier_block) -> ::std::os::raw::c_int;
    pub fn call_dcbevent_notifiers(
        val: ::std::os::raw::c_ulong,
        v: *mut ::std::ffi::c_void,
    ) -> ::std::os::raw::c_int;
}

#[cfg(not(feature = "CONFIG_DCB"))]
#[inline]
pub unsafe fn register_dcbevent_notifier(_nb: *mut notifier_block) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(feature = "CONFIG_DCB"))]
#[inline]
pub unsafe fn unregister_dcbevent_notifier(_nb: *mut notifier_block) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(feature = "CONFIG_DCB"))]
#[inline]
pub unsafe fn call_dcbevent_notifiers(
    _val: ::std::os::raw::c_ulong,
    _v: *mut ::std::ffi::c_void,
) -> ::std::os::raw::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
