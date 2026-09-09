/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/firmware-map.h:
 *  Copyright (C) 2008 SUSE LINUX Products GmbH
 *  by Bernhard Walle <bernhard.walle@gmx.de>
 */

// Dependency supplied by the surrounding kernel translation.

/*
 * Provide a dummy interface if CONFIG_FIRMWARE_MEMMAP is disabled.
 * The original build-time condition is represented here by the
 * `firmware_memmap` configuration feature.
 */
#[cfg(feature = "firmware_memmap")]
extern "C" {
    pub fn firmware_map_add_early(
        start: u64,
        end: u64,
        r#type: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn firmware_map_add_hotplug(
        start: u64,
        end: u64,
        r#type: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn firmware_map_remove(
        start: u64,
        end: u64,
        r#type: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
}

#[cfg(not(feature = "firmware_memmap"))]
#[inline]
pub unsafe fn firmware_map_add_early(
    _start: u64,
    _end: u64,
    _r#type: *const core::ffi::c_char,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "firmware_memmap"))]
#[inline]
pub unsafe fn firmware_map_add_hotplug(
    _start: u64,
    _end: u64,
    _r#type: *const core::ffi::c_char,
) -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "firmware_memmap"))]
#[inline]
pub unsafe fn firmware_map_remove(
    _start: u64,
    _end: u64,
    _r#type: *const core::ffi::c_char,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
