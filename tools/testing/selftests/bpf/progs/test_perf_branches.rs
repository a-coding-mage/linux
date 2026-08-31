// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// Dependencies from the original C source:
// <stddef.h>
// <linux/ptrace.h>
// <linux/bpf.h>
// <bpf/bpf_helpers.h>
// <bpf/bpf_tracing.h>

pub type __u64 = u64;

pub const BPF_F_GET_BRANCH_RECORDS_SIZE: u64 = 1 << 0;

unsafe extern "C" {
    fn bpf_read_branch_records(
        ctx: *mut core::ffi::c_void,
        buf: *mut core::ffi::c_void,
        size: __u64,
        flags: __u64,
    ) -> i32;
}

#[no_mangle]
pub static mut valid: i32 = 0;
#[no_mangle]
pub static mut run_cnt: i32 = 0;
#[no_mangle]
pub static mut required_size_out: i32 = 0;
#[no_mangle]
pub static mut written_stack_out: i32 = 0;
#[no_mangle]
pub static mut written_global_out: i32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpbe_t {
    pub _a: __u64,
    pub _b: __u64,
    pub _c: __u64,
}

#[no_mangle]
pub static mut fpbe: [fpbe_t; 30] = [fpbe_t {
    _a: 0,
    _b: 0,
    _c: 0,
}; 30];

// SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn perf_branches(ctx: *mut core::ffi::c_void) -> i32 {
    let mut entries: [__u64; 4 * 3] = [0; 4 * 3];
    let required_size: i32;
    let written_stack: i32;
    let written_global: i32;

    run_cnt += 1;

    /* write to stack */
    written_stack = bpf_read_branch_records(
        ctx,
        entries.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&entries) as __u64,
        0,
    );
    /* ignore spurious events */
    if written_stack == 0 {
        return 1;
    }

    /* get required size */
    required_size = bpf_read_branch_records(
        ctx,
        core::ptr::null_mut(),
        0,
        BPF_F_GET_BRANCH_RECORDS_SIZE,
    );

    written_global = bpf_read_branch_records(
        ctx,
        fpbe.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&fpbe) as __u64,
        0,
    );
    /* ignore spurious events */
    if written_global == 0 {
        return 1;
    }

    required_size_out = required_size;
    written_stack_out = written_stack;
    written_global_out = written_global;
    valid = 1;

    return 0;
}

// SEC("license")
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
