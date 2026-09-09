/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * display.h - OMAP2+ integration-specific DSS header
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 */

// Dependency supplied by the Linux kernel environment; the C header's
// declarations are represented using Rust's built-in u8 and bool types.

#[repr(C)]
pub struct omap_dss_dispc_dev_attr {
    pub manager_count: u8,
    pub has_framedonetv_irq: bool,
}

extern "C" {
    pub fn omap_init_vrfb() -> ::core::ffi::c_int;
    pub fn omap_init_fb() -> ::core::ffi::c_int;
    pub fn omap_init_vout() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
