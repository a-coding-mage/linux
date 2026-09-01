// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, and "bpf_legacy.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;

pub const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    _bindgen_opaque_blob: [u8; 0],
}

#[repr(C)]
pub struct jmp_table_def {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut jmp_table: jmp_table_def = jmp_table_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

extern "C" {
    fn load_byte(skb: *mut __sk_buff, off: i32) -> i32;
    fn bpf_tail_call_static(skb: *mut __sk_buff, map: *mut jmp_table_def, index: __u32);
}

#[inline(never)]
unsafe fn subprog_tail(skb: *mut __sk_buff) -> i32 {
    let ret: i32 = 1;

    if load_byte(skb, 0) != 0 {
        bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 1);
    } else {
        bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 0);
    }
    core::hint::black_box(ret);
    ret
}

#[no_mangle]
pub static mut count: i32 = 0;

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    count += 1;
    subprog_tail(skb)
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, core::ptr::addr_of_mut!(jmp_table), 0);

    0
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
