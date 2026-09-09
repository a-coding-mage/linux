/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct xt_helper_info {
    pub invert: ::core::ffi::c_int,
    pub name: [::core::ffi::c_char; 30],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
