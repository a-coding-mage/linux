// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

// C includes translated as dependency intent:
// <stdbool.h>, <linux/bpf.h>, <bpf/bpf_helpers.h>

pub type __u64 = u64;

#[no_mangle]
pub static mut out__bpf_link_fops: __u64 = -1i64 as __u64;
#[no_mangle]
pub static mut out__bpf_link_fops1: __u64 = -1i64 as __u64;
#[no_mangle]
pub static mut out__btf_size: __u64 = -1i64 as __u64;
#[no_mangle]
pub static mut out__per_cpu_start: __u64 = -1i64 as __u64;

extern "C" {
    #[link_name = "bpf_link_fops"]
    pub static bpf_link_fops: core::ffi::c_void;
    #[link_name = "__start_BTF"]
    pub static __start_BTF: core::ffi::c_void;
    #[link_name = "__stop_BTF"]
    pub static __stop_BTF: core::ffi::c_void;
    #[link_name = "__per_cpu_start"]
    pub static __per_cpu_start: core::ffi::c_void;
    /* non-existing symbol, weak, default to zero */
    #[link_name = "bpf_link_fops1"]
    pub static bpf_link_fops1: core::ffi::c_void;
}

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn handler(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;

    out__bpf_link_fops = (&bpf_link_fops as *const core::ffi::c_void) as __u64;
    out__btf_size = ((&__stop_BTF as *const core::ffi::c_void as isize)
        .wrapping_sub(&__start_BTF as *const core::ffi::c_void as isize)) as __u64;
    out__per_cpu_start = (&__per_cpu_start as *const core::ffi::c_void) as __u64;

    out__bpf_link_fops1 = (&bpf_link_fops1 as *const core::ffi::c_void) as __u64;

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
