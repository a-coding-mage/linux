/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/sound/wm2000.h -- Platform data for WM2000
 *
 * Copyright 2010 Wolfson Microelectronics. PLC.
 */

#[repr(C)]
pub struct wm2000_platform_data {
    /// Filename for system-specific image to download to device.
    pub download_file: *const core::ffi::c_char,

    /// Disable speech clarity enhancement, for use when an external algorithm
    /// is used. This corresponds to the C unsigned int 1-bit bit-field.
    pub speech_enh_disable: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
