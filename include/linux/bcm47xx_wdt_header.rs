/* SPDX-License-Identifier: GPL-2.0 */

// Translated from linux/bcm47xx_wdt.h.
// External Linux types are supplied by other translated dependencies.

use core::ffi::c_void;

#[repr(C)]
pub struct bcm47xx_wdt {
    pub timer_set:
        Option<unsafe extern "C" fn(wdt: *mut bcm47xx_wdt, ms: u32) -> u32>,
    pub timer_set_ms:
        Option<unsafe extern "C" fn(wdt: *mut bcm47xx_wdt, ms: u32) -> u32>,
    pub max_timer_ms: u32,

    pub driver_data: *mut c_void,

    pub wdd: watchdog_device,

    pub soft_timer: timer_list,
    pub soft_ticks: atomic_t,
}

/// Equivalent to the C inline function `bcm47xx_wdt_get_drvdata`.
#[inline]
pub unsafe fn bcm47xx_wdt_get_drvdata(wdt: *mut bcm47xx_wdt) -> *mut c_void {
    (*wdt).driver_data
}

// Dependencies supplied by other translated Linux headers.
extern "C" {
    pub type watchdog_device;
    pub type timer_list;
    pub type atomic_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
