// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_legacy.h"
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    pub len: __u32,
}

#[repr(C)]
pub struct jmp_table_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: jmp_table_def = jmp_table_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

extern "C" {
    fn load_word(skb: *mut __sk_buff, off: i32) -> i32;
    fn load_half(skb: *mut __sk_buff, off: i32) -> i32;
    fn bpf_tail_call_static(ctx: *mut __sk_buff, map: *mut jmp_table_def, index: __u32);
    fn __sink(value: i8);
}

#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn subprog_tail2(skb: *mut __sk_buff) -> i32 {
    let arr: [i8; 64] = [0; 64];

    if load_word(skb, 0) != 0 || load_half(skb, 0) != 0 {
        bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 10);
    } else {
        bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 1);
    }

    __sink(core::ptr::read_volatile(&arr[core::mem::size_of_val(&arr) - 1]));

    (*skb).len as i32
}

#[inline(never)]
unsafe extern "C" fn subprog_tail(skb: *mut __sk_buff) -> i32 {
    let arr: [i8; 64] = [0; 64];

    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 0);

    __sink(core::ptr::read_volatile(&arr[core::mem::size_of_val(&arr) - 1]));

    ((*skb).len).wrapping_mul(2) as i32
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    let arr: [i8; 128] = [0; 128];

    __sink(core::ptr::read_volatile(&arr[core::mem::size_of_val(&arr) - 1]));

    subprog_tail2(skb)
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> i32 {
    let arr: [i8; 128] = [0; 128];

    __sink(core::ptr::read_volatile(&arr[core::mem::size_of_val(&arr) - 1]));

    ((*skb).len).wrapping_mul(3) as i32
}

#[no_mangle]
#[link_section = "tc"]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    let arr: [i8; 128] = [0; 128];

    __sink(core::ptr::read_volatile(&arr[core::mem::size_of_val(&arr) - 1]));

    subprog_tail(skb)
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
