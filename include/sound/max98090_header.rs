/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Platform data for MAX98090
 *
 * Copyright 2011-2012 Maxim Integrated Products
 */

/* codec platform data */
#[repr(C)]
pub struct max98090_pdata {
    /*
     * Analog/digital microphone configuration:
     * 0 = analog microphone input (normal setting)
     * 1 = digital microphone input
     *
     * The C declaration stores these four one-bit fields in one unsigned int.
     */
    pub bits: ::core::ffi::c_uint,
}

impl max98090_pdata {
    pub const DIGMIC_LEFT_MODE: ::core::ffi::c_uint = 1 << 0;
    pub const DIGMIC_RIGHT_MODE: ::core::ffi::c_uint = 1 << 1;
    pub const DIGMIC_3_MODE: ::core::ffi::c_uint = 1 << 2;
    pub const DIGMIC_4_MODE: ::core::ffi::c_uint = 1 << 3;

    #[inline]
    pub unsafe fn digmic_left_mode(&self) -> ::core::ffi::c_uint {
        (self.bits >> 0) & 1
    }

    #[inline]
    pub unsafe fn digmic_right_mode(&self) -> ::core::ffi::c_uint {
        (self.bits >> 1) & 1
    }

    #[inline]
    pub unsafe fn digmic_3_mode(&self) -> ::core::ffi::c_uint {
        (self.bits >> 2) & 1
    }

    #[inline]
    pub unsafe fn digmic_4_mode(&self) -> ::core::ffi::c_uint {
        (self.bits >> 3) & 1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
