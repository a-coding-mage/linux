// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

// Map definition originally used __uint(...) BPF helper macros and SEC(".maps").
#[repr(C)]
pub struct jmp_table {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: usize,
    pub value_size: usize,
}

#[link_section = ".maps"]
#[no_mangle]
pub static jmp_table: jmp_table = jmp_table {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>(),
    value_size: core::mem::size_of::<__u32>(),
};

#[no_mangle]
pub static mut done: i32 = 0;

extern "C" {
    static BPF_MAP_TYPE_PROG_ARRAY: __u32;

    fn bpf_tail_call_static(skb: *mut __sk_buff, map: *const jmp_table, index: __u32);
    fn __sink(arg: i8);
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    let _ = skb;

    done = 1;
    0
}

#[inline(never)]
unsafe fn subprog_tail(skb: *mut __sk_buff) -> i32 {
    // Don't propagate the constant to the caller
    let mut ret: i32 = 1;
    core::ptr::write_volatile(&mut ret, 1);

    bpf_tail_call_static(skb, &jmp_table, 0);
    core::ptr::read_volatile(&ret)
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    // Have data on stack which size is not a multiple of 8
    let mut arr: [i8; 1] = [0; 1];

    __sink(core::ptr::read_volatile(arr.as_ptr()));

    subprog_tail(skb)
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
