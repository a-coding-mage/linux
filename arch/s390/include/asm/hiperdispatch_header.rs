/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2024
 */

// Declarations translated from _ASM_HIPERDISPATCH_H.

unsafe extern "C" {
    pub fn hd_reset_state();
    pub fn hd_add_core(cpu: ::core::ffi::c_int);
    pub fn hd_disable_hiperdispatch();
    pub fn hd_enable_hiperdispatch() -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
