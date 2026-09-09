/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * RJ54N1CB0C Private data
 *
 * Copyright (C) 2009, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

#[repr(C)]
pub struct rj54n1_pdata {
    pub mclk_freq: ::core::ffi::c_uint,
    pub ioctl_high: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
