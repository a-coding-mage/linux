/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2015 Jiri Pirko <jiri@resnulli.us>
 */

// Translated from the C header.  The following types are supplied by the
// corresponding Linux headers: tc_action, bpf_prog, and sock_filter.

use core::ffi::c_char;

#[repr(C)]
pub union tcf_bpf_bpf_fd_or_num_ops {
    pub bpf_fd: u32,
    pub bpf_num_ops: u16,
}

#[repr(C)]
pub struct tcf_bpf {
    pub common: tc_action,
    // __rcu annotation preserved as a comment; bpf_prog is supplied externally.
    pub filter: *mut bpf_prog,
    pub bpf_fd_or_num_ops: tcf_bpf_bpf_fd_or_num_ops,
    pub bpf_ops: *mut sock_filter,
    pub bpf_name: *const c_char,
}

#[inline]
pub unsafe fn to_bpf(a: *mut tc_action) -> *mut tcf_bpf {
    a as *mut tcf_bpf
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
