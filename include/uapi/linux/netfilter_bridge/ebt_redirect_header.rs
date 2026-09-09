/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct ebt_redirect_info {
    /* EBT_ACCEPT, EBT_DROP, EBT_CONTINUE or EBT_RETURN */
    pub target: ::core::ffi::c_int,
}

pub const EBT_REDIRECT_TARGET: &str = "redirect";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
