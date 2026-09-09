/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency supplied by linux/netfilter/nfnetlink_acct.h.

use core::ffi::c_char;

pub struct nf_acct;

#[repr(C)]
pub struct xt_nfacct_match_info {
    pub name: [c_char; NFACCT_NAME_MAX],
    pub nfacct: *mut nf_acct,
}

// The nfacct member is declared with __attribute__((aligned(8))) in C.
#[repr(C, align(8))]
pub struct xt_nfacct_match_info_v1 {
    pub name: [c_char; NFACCT_NAME_MAX],
    pub nfacct: *mut nf_acct,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
