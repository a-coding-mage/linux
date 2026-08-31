// SPDX-License-Identifier: GPL-2.0
#![no_std]

// Dependencies from the original includes:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[repr(C)]
pub struct jmp_table {
    pub r#type: __u32,
    pub max_entries: __u32,
    pub key_size: usize,
    pub value_size: usize,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut jmp_table: jmp_table = jmp_table {
    r#type: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 2,
    key_size: core::mem::size_of::<__u32>(),
    value_size: core::mem::size_of::<__u32>(),
};

// Translation of TAIL_FUNC(0)
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    0
}

// Translation of TAIL_FUNC(1)
#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn classifier_1(skb: *mut __sk_buff) -> i32 {
    1
}

#[inline(never)]
unsafe fn subprog_tail(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, &raw mut jmp_table, 0);

    ((*skb).len).wrapping_mul(2) as i32
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    bpf_tail_call_static(skb, &raw mut jmp_table, 1);

    subprog_tail(skb)
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";
