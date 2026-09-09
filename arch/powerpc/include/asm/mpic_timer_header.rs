/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/powerpc/include/asm/mpic_timer.h
 *
 * Header file for Mpic Global Timer
 *
 * Copyright 2013 Freescale Semiconductor, Inc.
 *
 * Author: Wang Dongsheng <Dongsheng.Wang@freescale.com>
 *	   Li Yang <leoli@freescale.com>
 */

// Dependencies supplied by other headers:
// irq_handler_t, cascade_priv, and time64_t.

#[repr(C)]
pub struct mpic_timer {
    pub dev: *mut core::ffi::c_void,
    pub cascade_handle: *mut cascade_priv,
    pub num: core::ffi::c_uint,
    pub irq: core::ffi::c_uint,
}

// CONFIG_MPIC_TIMER
#[cfg(feature = "CONFIG_MPIC_TIMER")]
extern "C" {
    pub fn mpic_request_timer(
        fn_: irq_handler_t,
        dev: *mut core::ffi::c_void,
        time: time64_t,
    ) -> *mut mpic_timer;
    pub fn mpic_start_timer(handle: *mut mpic_timer);
    pub fn mpic_stop_timer(handle: *mut mpic_timer);
    pub fn mpic_get_remain_time(handle: *mut mpic_timer, time: *mut time64_t);
    pub fn mpic_free_timer(handle: *mut mpic_timer);
}

#[cfg(not(feature = "CONFIG_MPIC_TIMER"))]
pub unsafe fn mpic_request_timer(
    _fn: irq_handler_t,
    _dev: *mut core::ffi::c_void,
    _time: time64_t,
) -> *mut mpic_timer {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_MPIC_TIMER"))]
pub unsafe fn mpic_start_timer(_handle: *mut mpic_timer) {}

#[cfg(not(feature = "CONFIG_MPIC_TIMER"))]
pub unsafe fn mpic_stop_timer(_handle: *mut mpic_timer) {}

#[cfg(not(feature = "CONFIG_MPIC_TIMER"))]
pub unsafe fn mpic_get_remain_time(_handle: *mut mpic_timer, _time: *mut time64_t) {}

#[cfg(not(feature = "CONFIG_MPIC_TIMER"))]
pub unsafe fn mpic_free_timer(_handle: *mut mpic_timer) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
