// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Google */

// Depends on definitions from vmlinux.h, bpf/bpf_helpers.h, and
// bpf/bpf_tracing.h in the original C source.

use core::ffi::c_void;

pub type __u32 = u32;

#[repr(C)]
pub struct path {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kstat {
    _private: [u8; 0],
}

extern "C" {
    #[link_name = "bpf_prog_active"]
    static bpf_prog_active: i32;

    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_per_cpu_ptr(ptr: *const c_void, cpu: __u32) -> *mut c_void;
    fn bpf_d_path(path: *mut path, buf: *mut c_void, sz: u32) -> i64;
}

#[no_mangle]
#[link_section = "fentry/security_inode_getattr"]
pub unsafe extern "C" fn d_path_check_rdonly_mem(
    path: *mut path,
    stat: *mut kstat,
    request_mask: __u32,
    query_flags: u32,
) -> i32 {
    let active: *mut c_void;
    let cpu: __u32;

    let _ = stat;
    let _ = request_mask;
    let _ = query_flags;

    cpu = bpf_get_smp_processor_id();
    active = bpf_per_cpu_ptr(
        (&bpf_prog_active as *const i32).cast::<c_void>(),
        cpu,
    );
    if !active.is_null() {
        /* FAIL here! 'active' points to readonly memory. bpf helpers
         * that update its arguments can not write into it.
         */
        bpf_d_path(path, active, core::mem::size_of::<i32>() as u32);
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
