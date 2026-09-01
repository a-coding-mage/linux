// SPDX-License-Identifier: GPL-2.0
// Dependencies from the C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct jmp_table_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static jmp_table: jmp_table_def = jmp_table_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 3,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u32>() as u32,
};

extern "C" {
    pub fn bpf_tail_call_static(ctx: *mut __sk_buff, prog_array_map: *const jmp_table_def, index: u32);
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn classifier_0(_skb: *mut __sk_buff) -> i32 {
    0
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn classifier_1(_skb: *mut __sk_buff) -> i32 {
    1
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn classifier_2(_skb: *mut __sk_buff) -> i32 {
    2
}

#[link_section = "tc"]
#[no_mangle]
pub extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    /* Multiple locations to make sure we patch
     * all of them.
     */
    unsafe {
        bpf_tail_call_static(skb, &jmp_table, 0);
        bpf_tail_call_static(skb, &jmp_table, 0);
        bpf_tail_call_static(skb, &jmp_table, 0);
        bpf_tail_call_static(skb, &jmp_table, 0);

        bpf_tail_call_static(skb, &jmp_table, 1);
        bpf_tail_call_static(skb, &jmp_table, 1);
        bpf_tail_call_static(skb, &jmp_table, 1);
        bpf_tail_call_static(skb, &jmp_table, 1);

        bpf_tail_call_static(skb, &jmp_table, 2);
        bpf_tail_call_static(skb, &jmp_table, 2);
        bpf_tail_call_static(skb, &jmp_table, 2);
        bpf_tail_call_static(skb, &jmp_table, 2);
    }

    3
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
