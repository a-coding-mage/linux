// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */
// C includes translated as external dependency expectations:
// <vmlinux.h>
// <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SEC(".maps")
// Original BPF map declaration:
// __uint(type, BPF_MAP_TYPE_RHASH);
// __uint(map_flags, BPF_F_NO_PREALLOC);
// __uint(max_entries, 64);
// __type(key, __u32);
// __type(value, __u64);
#[repr(C)]
pub struct rhashmap {
    pub type_: u32,
    pub map_flags: u32,
    pub max_entries: u32,
    pub key: u32,
    pub value: u64,
}

extern "C" {
    pub static mut rhashmap: rhashmap;
}

#[no_mangle]
pub static mut key_sum: u32 = 0;
#[no_mangle]
pub static mut val_sum: u64 = 0;
#[no_mangle]
pub static mut elem_count: u32 = 0;
#[no_mangle]
pub static mut err: u32 = 0;

#[repr(C)]
pub struct bpf_iter__bpf_map_elem {
    pub key: *mut u32,
    pub value: *mut u64,
}

#[no_mangle]
#[link_section = "iter/bpf_map_elem"]
pub unsafe extern "C" fn dump_bpf_rhash_map(ctx: *mut bpf_iter__bpf_map_elem) -> i32 {
    let key: *mut u32 = (*ctx).key;
    let val: *mut u64 = (*ctx).value;

    if key.is_null() || val.is_null() {
        return 0;
    }

    key_sum = key_sum.wrapping_add(*key);
    val_sum = val_sum.wrapping_add(*val);
    elem_count = elem_count.wrapping_add(1);
    return 0;
}
