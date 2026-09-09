/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Copyright (c) 2010, 2011 Fabien Marteau <fabien.marteau@armadeus.com>
 */

#[repr(C)]
pub struct as5011_platform_data {
    pub axis_irq: ::core::ffi::c_uint, /* irq number */
    pub axis_irqflags: ::core::ffi::c_ulong,
    pub xp: ::core::ffi::c_char,
    pub xn: ::core::ffi::c_char, /* threshold for x axis */
    pub yp: ::core::ffi::c_char,
    pub yn: ::core::ffi::c_char, /* threshold for y axis */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
