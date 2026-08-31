// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023 Yafang Shao <laoar.shao@gmail.com> */

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_tracing.h>

extern "C" {
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

extern "C" {
    static BPF_RAW_TRACEPOINT_OPEN: i32;
}

#[repr(C)]
pub union bpf_attr {
    pub raw_tracepoint: bpf_attr__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attr__bindgen_ty_1 {
    pub name: u64,
}

#[no_mangle]
pub static mut tp_name: [u8; 128] = [0; 128];

// SEC("lsm.s/bpf")
#[no_mangle]
pub unsafe extern "C" fn lsm_run(
    cmd: i32,
    attr: *mut bpf_attr,
    size: u32,
    kernel: bool,
) -> i32 {
    match cmd {
        x if x == BPF_RAW_TRACEPOINT_OPEN => {
            bpf_copy_from_user(
                tp_name.as_mut_ptr() as *mut core::ffi::c_void,
                (core::mem::size_of_val(&tp_name) - 1) as u32,
                (*attr).raw_tracepoint.name as *const core::ffi::c_void,
            );
        }
        _ => {}
    }
    0
}

// SEC("raw_tracepoint")
#[no_mangle]
pub unsafe extern "C" fn raw_tp_run() -> i32 {
    0
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
