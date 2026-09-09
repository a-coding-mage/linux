/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  i2c_pxa.h
 *
 *  Copyright (C) 2002 Intrinsyc Software Inc.
 */

use core::ffi::{c_uchar, c_uint, c_ulong};

#[repr(C)]
pub struct i2c_pxa_platform_data {
    pub class: c_uint,
    // C bit-fields of unsigned int; each field occupies one bit in its
    // declaration unit. Rust has no native bit-field syntax.
    pub use_pio: c_uint,
    pub fast_mode: c_uint,
    pub high_mode: c_uint,
    pub master_code: c_uchar,
    pub rate: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
