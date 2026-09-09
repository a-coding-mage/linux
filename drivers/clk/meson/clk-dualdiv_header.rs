/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2019 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependency supplied by the Linux clock-provider interface.
// Dependency supplied by the local parm header.

#[repr(C)]
pub struct MesonClkDualdivParam {
    pub n1: ::core::ffi::c_uint,
    pub n2: ::core::ffi::c_uint,
    pub m1: ::core::ffi::c_uint,
    pub m2: ::core::ffi::c_uint,
    pub dual: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct MesonClkDualdivData {
    pub n1: Parm,
    pub n2: Parm,
    pub m1: Parm,
    pub m2: Parm,
    pub dual: Parm,
    pub table: *const MesonClkDualdivParam,
}

unsafe extern "C" {
    pub static meson_clk_dualdiv_ops: ClkOps;
    pub static meson_clk_dualdiv_ro_ops: ClkOps;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
