/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap_device headers
 *
 * Copyright (C) 2009 Nokia Corporation
 * Paul Walmsley
 *
 * Developed in collaboration with (alphabetical order): Benoit
 * Cousson, Kevin Hilman, Tony Lindgren, Rajendra Nayak, Vikram
 * Pandita, Sakari Poussa, Anand Sawant, Santosh Shilimkar, Richard
 * Woodruff
 *
 * This type of functionality should be implemented as a proper
 * omap_bus/omap_device in Linux.
 *
 * omap_device differs from omap_hwmod in that it includes external
 * (e.g., board- and system-level) integration details.  omap_hwmod
 * stores hardware data that is invariant for a given OMAP chip.
 */

use core::ffi::{c_char, c_int, c_ulong};

// C dependencies supplied by the surrounding translation unit.
#[repr(C)]
pub struct platform_device {
    pub archdata: platform_device_archdata,
}

#[repr(C)]
pub struct platform_device_archdata {
    pub od: *mut omap_device,
}

#[repr(C)]
pub struct omap_hwmod {
    _private: [u8; 0],
}

/* omap_device._state values */
pub const OMAP_DEVICE_STATE_UNKNOWN: u8 = 0;
pub const OMAP_DEVICE_STATE_ENABLED: u8 = 1;
pub const OMAP_DEVICE_STATE_IDLE: u8 = 2;
pub const OMAP_DEVICE_STATE_SHUTDOWN: u8 = 3;

/* omap_device.flags values */
pub const OMAP_DEVICE_SUSPENDED: usize = 1usize << 0;

/**
 * struct omap_device - omap_device wrapper for platform_devices
 * @pdev: platform_device
 * @hwmods_cnt: ARRAY_SIZE() of @hwmods
 * @_state: one of OMAP_DEVICE_STATE_* (see above)
 * @flags: device flags
 * @_driver_status: one of BUS_NOTIFY_*_DRIVER from <linux/device.h>
 * @hwmods: (one .. many per omap_device)
 *
 * Integrates omap_hwmod data into Linux platform_device.
 *
 * Field names beginning with underscores are for the internal use of
 * the omap_device code.
 *
 */
#[repr(C)]
pub struct omap_device {
    pub pdev: *mut platform_device,
    pub _driver_status: c_ulong,
    pub hwmods_cnt: u8,
    pub _state: u8,
    pub flags: u8,
    // __counted_by(hwmods_cnt); flexible array member
    pub hwmods: [*mut omap_hwmod; 0],
}

/* Device driver interface (call via platform_data fn ptrs) */

unsafe extern "C" {
    pub fn omap_device_enable(pdev: *mut platform_device) -> c_int;
    pub fn omap_device_idle(pdev: *mut platform_device) -> c_int;

    /* Other */
    pub fn omap_device_assert_hardreset(
        pdev: *mut platform_device,
        name: *const c_char,
    ) -> c_int;
    pub fn omap_device_deassert_hardreset(
        pdev: *mut platform_device,
        name: *const c_char,
    ) -> c_int;
}

/* Get omap_device pointer from platform_device pointer */
#[inline]
pub unsafe fn to_omap_device(pdev: *mut platform_device) -> *mut omap_device {
    if !pdev.is_null() {
        (*pdev).archdata.od
    } else {
        core::ptr::null_mut()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
