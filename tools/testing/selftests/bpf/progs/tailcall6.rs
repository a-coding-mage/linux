// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_tail_call(ctx: *mut __sk_buff, prog_array_map: *mut core::ffi::c_void, index: u32);
    fn __bpf_unreachable() -> !;
}

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct jmp_table_def {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    pub type_: u32,
    // __uint(max_entries, 1);
    pub max_entries: u32,
    // __uint(key_size, sizeof(__u32));
    pub key_size: u32,
    // __uint(value_size, sizeof(__u32));
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut jmp_table: jmp_table_def = jmp_table_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

#[no_mangle]
pub static mut count: i32 = 0;

#[no_mangle]
pub static mut which: i32 = 0;

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn classifier_0(skb: *mut __sk_buff) -> i32 {
    count = count.wrapping_add(1);
    // C: if (__builtin_constant_p(which))
    // `which` is a mutable global, so this file-local Rust translation preserves the non-constant path.
    bpf_tail_call(
        skb,
        &mut jmp_table as *mut jmp_table_def as *mut core::ffi::c_void,
        which as u32,
    );
    return 1;
}

#[link_section = "tc"]
#[no_mangle]
pub unsafe extern "C" fn entry(skb: *mut __sk_buff) -> i32 {
    // C: if (__builtin_constant_p(which))
    // `which` is a mutable global, so this file-local Rust translation preserves the non-constant path.
    bpf_tail_call(
        skb,
        &mut jmp_table as *mut jmp_table_def as *mut core::ffi::c_void,
        which as u32,
    );
    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static __license: [u8; 4] = *b"GPL\0";
