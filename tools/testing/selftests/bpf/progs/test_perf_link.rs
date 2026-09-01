// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[no_mangle]
pub static mut run_cnt: i32 = 0;

unsafe extern "C" {
    fn __sync_fetch_and_add(ptr: *mut i32, value: i32) -> i32;
}

#[no_mangle]
#[link_section = "perf_event"]
pub unsafe extern "C" fn handler(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        __sync_fetch_and_add(core::ptr::addr_of_mut!(run_cnt), 1);
    }
    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
