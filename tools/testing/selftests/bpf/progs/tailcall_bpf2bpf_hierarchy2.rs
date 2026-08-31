// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"
// #include "bpf_test_utils.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::c_void;

pub type __u32 = u32;

// From <linux/bpf.h>; used by the BPF map definition below.
pub const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

extern "C" {
    fn bpf_tail_call_static(ctx: *mut __sk_buff, map: *mut c_void, index: __u32);
    fn barrier_var(arg: i32);
    fn __sink(arg: i32);
    fn clobber_regs_stack();
}

#[repr(C)]
pub struct jmp_table_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: usize,
    pub values: [*mut c_void; 2],
}

// SEC(".maps")
#[no_mangle]
pub static mut jmp_table: jmp_table_def = jmp_table_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>(),
    values: [
        classifier_0 as *mut c_void,
        classifier_1 as *mut c_void,
    ],
};

#[no_mangle]
pub static mut count0: i32 = 0;
#[no_mangle]
pub static mut count1: i32 = 0;

// static __noinline
#[inline(never)]
unsafe fn subprog_tail0(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;

    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table).cast::<c_void>(), 0);
    barrier_var(ret);
    return ret;
}

// __auxiliary
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    count0 = count0.wrapping_add(1);
    subprog_tail0(skb);
    return 0;
}

// static __noinline
#[inline(never)]
unsafe fn subprog_tail1(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;

    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table).cast::<c_void>(), 1);
    barrier_var(ret);
    return ret;
}

// __auxiliary
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> i32 {
    let ret: i32;

    count1 = count1.wrapping_add(1);
    ret = subprog_tail1(skb);
    __sink(ret);
    return 0;
}

// __success
// __retval(33)
// SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn tailcall_bpf2bpf_hierarchy_2(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;
    let ret1: i32;
    let ret2: i32;

    clobber_regs_stack();

    ret1 = subprog_tail0(skb);
    ret2 = subprog_tail1(skb);
    __sink(ret1);
    __sink(ret2);
    __sink(ret);
    return (count1.wrapping_shl(16)) | count0;
}

// SEC("license")
#[no_mangle]
pub static mut __license: [u8; 4] = *b"GPL\0";
