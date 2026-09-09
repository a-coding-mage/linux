/* SPDX-License-Identifier: GPL-2.0
 *
 * Arch specific extensions to struct device
 */

// Dependency supplied by asm-generic/device.h.

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    /* allocate contiguous memory chunk and fill in struct resource */
    pub fn platform_resource_setup_memory(
        pdev: *mut platform_device,
        name: *mut c_char,
        memsize: c_ulong,
    ) -> ::core::ffi::c_int;

    pub fn plat_early_device_setup();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
