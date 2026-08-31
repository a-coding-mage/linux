// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Cloudflare
// C dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>

#[repr(C)]
pub struct bpf_sock_ops {
    pub sk: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct map {
    pub type_: u32,
    pub max_entries: u32,
    pub key: u32,
    pub value: u64,
}

extern "C" {
    fn bpf_map_update_elem(
        map: *mut map,
        key: *const u32,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

// Original BPF map declaration:
// struct {
//     __uint(type, BPF_MAP_TYPE_SOCKMAP);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, __u64);
// } map SEC(".maps");
#[link_section = ".maps"]
#[no_mangle]
pub static mut map: map = map {
    type_: BPF_MAP_TYPE_SOCKMAP,
    max_entries: 1,
    key: 0,
    value: 0,
};

extern "C" {
    static BPF_MAP_TYPE_SOCKMAP: u32;
}

#[link_section = "sockops"]
#[no_mangle]
pub unsafe extern "C" fn bpf_sockmap(skops: *mut bpf_sock_ops) -> i32 {
    let key: u32 = 0;

    if !(*skops).sk.is_null() {
        bpf_map_update_elem(&raw mut map, &key, (*skops).sk, 0);
    }
    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
