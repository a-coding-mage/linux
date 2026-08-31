// SPDX-License-Identifier: GPL-2.0

// C dependencies translated as Rust dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

type __u32 = u32;

extern "C" {
    fn bpf_tail_call_static(skb: *mut __sk_buff, map: *mut JmpTable, index: __u32);
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
//     __uint(max_entries, 1);
//     __uint(key_size, sizeof(__u32));
//     __uint(value_size, sizeof(__u32));
// } jmp_table SEC(".maps");
#[repr(C)]
pub struct JmpTable {
    pub type_: __u32,
    pub max_entries: __u32,
    pub key_size: usize,
    pub value_size: usize,
}

const BPF_MAP_TYPE_PROG_ARRAY: __u32 = 3;

#[no_mangle]
#[link_section = ".maps"]
pub static mut jmp_table: JmpTable = JmpTable {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>(),
    value_size: core::mem::size_of::<__u32>(),
};

#[no_mangle]
pub static mut count: i32 = 0;

#[no_mangle]
#[link_section = "freplace"]
pub unsafe extern "C" fn entry_freplace(skb: *mut __sk_buff) -> i32 {
    count += 1;
    bpf_tail_call_static(skb, &mut jmp_table, 0);
    count
}

#[no_mangle]
#[link_section = "license"]
pub static __license: [u8; 4] = *b"GPL\0";
