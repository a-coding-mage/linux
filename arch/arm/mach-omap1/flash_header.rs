/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Flash support for OMAP1
 */

// Dependency: declarations from <linux/mtd/map.h> are supplied externally.

/// Opaque forward declaration of `struct platform_device`.
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn omap1_set_vpp(pdev: *mut platform_device, enable: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
