// SPDX-License-Identifier: GPL-2.0

// Dependencies in the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

extern "C" {
    fn bpf_tail_call(ctx: *mut __sk_buff, prog_array_map: *const ProgArrayMap, index: u32);
}

#[repr(C)]
pub struct __sk_buff {
    _unused: [u8; 0],
}

pub const BPF_MAP_TYPE_PROG_ARRAY: u32 = 3;

#[repr(C)]
pub struct ProgArrayMap {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static prog_array: ProgArrayMap = ProgArrayMap {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
};

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn caller_prog(skb: *mut __sk_buff) -> i32 {
    unsafe {
        bpf_tail_call(skb, &prog_array, 0);
    }
    1
}

#[link_section = "cgroup_skb/egress"]
#[no_mangle]
pub unsafe extern "C" fn leaf_prog(_skb: *mut __sk_buff) -> i32 {
    1
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
