// SPDX-License-Identifier: GPL-2.0
/* Copyright Leon Hwang */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_test_utils.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

pub type __u32 = u32;

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

// Translated from the libbpf map-definition struct using __uint(...) fields.
#[repr(C)]
pub struct jmp_table_def {
    pub type_: *mut [i32; BPF_MAP_TYPE_PROG_ARRAY as usize],
    pub max_entries: *mut [i32; 1],
    pub key_size: *mut [i32; core::mem::size_of::<__u32>()],
    pub value_size: *mut [i32; core::mem::size_of::<__u32>()],
}

extern "C" {
    static BPF_MAP_TYPE_PROG_ARRAY: u32;

    fn bpf_tail_call_static(ctx: *mut c_void, map: *const jmp_table_def, index: u32);
    fn barrier_var(ret: i32);
    fn clobber_regs_stack();
    fn __sink(value: i32);
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: jmp_table_def = jmp_table_def {
    type_: core::ptr::null_mut(),
    max_entries: core::ptr::null_mut(),
    key_size: core::ptr::null_mut(),
    value_size: core::ptr::null_mut(),
};

#[no_mangle]
pub static mut count: i32 = 0;

#[inline(never)]
unsafe fn subprog_tail(ctx: *mut c_void) -> i32 {
    let mut ret: i32 = 0;

    bpf_tail_call_static(ctx, core::ptr::addr_of!(jmp_table), 0);
    barrier_var(ret);
    return ret;
}

#[no_mangle]
#[link_section = "fentry/dummy"]
pub unsafe extern "C" fn fentry(ctx: *mut c_void) -> i32 {
    let _skb: *mut sk_buff = ctx as *mut sk_buff;
    let ret1: i32;
    let ret2: i32;

    clobber_regs_stack();

    count += 1;
    ret1 = subprog_tail(ctx);
    ret2 = subprog_tail(ctx);
    __sink(ret1);
    __sink(ret2);

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
