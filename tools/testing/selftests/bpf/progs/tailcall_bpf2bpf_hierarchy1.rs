// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, "bpf_legacy.h",
// "bpf_test_utils.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct jmp_table {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut jmp_table: jmp_table = jmp_table {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

#[no_mangle]
pub static mut count: i32 = 0;

extern "C" {
    fn bpf_tail_call_static(skb: *mut __sk_buff, map: *mut jmp_table, index: u32);
    fn barrier_var(ret: i32);
    fn clobber_regs_stack();
    fn __sink(value: i32);
}

#[inline(never)]
unsafe fn subprog_tail(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 0;

    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 0);
    barrier_var(ret);
    ret
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 1;
    let ret1: i32;
    let ret2: i32;

    clobber_regs_stack();

    count += 1;
    ret1 = subprog_tail(skb);
    ret2 = subprog_tail(skb);
    __sink(ret1);
    __sink(ret2);

    ret
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
