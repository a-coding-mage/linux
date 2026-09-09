/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 Krzysztof Kozlowski <krzk@kernel.org>
 */

use core::ffi::c_void;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_S3C64XX_COMMON_CLK")]
unsafe extern "C" {
    pub fn s3c64xx_clk_init(
        np: *mut device_node,
        xtal_f: core::ffi::c_ulong,
        xusbxti_f: core::ffi::c_ulong,
        s3c6400: bool,
        base: *mut c_void,
    );
}

#[cfg(not(feature = "CONFIG_S3C64XX_COMMON_CLK"))]
#[inline]
pub unsafe fn s3c64xx_clk_init(
    _np: *mut device_node,
    _xtal_f: core::ffi::c_ulong,
    _xusbxti_f: core::ffi::c_ulong,
    _s3c6400: bool,
    _base: *mut c_void,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
