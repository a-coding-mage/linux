// SPDX-License-Identifier: GPL-2.0

// Dependencies from the original C source:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub type __u32 = u32;
pub type __u64 = u64;

// External kernel/libbpf-provided types.
#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_cgroup_storage_key {
    _private: [u8; 0],
}

// External BPF map type constants supplied by bpf headers.
extern "C" {
    pub static BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE: __u32;
    pub static BPF_MAP_TYPE_PROG_ARRAY: __u32;
}

#[repr(C)]
pub struct storage_map_def {
    // __uint(type, BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE);
    pub type_: __u32,
    // __type(key, struct bpf_cgroup_storage_key);
    pub key: *mut bpf_cgroup_storage_key,
    // __type(value, __u64);
    pub value: *mut __u64,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut storage_map: storage_map_def = storage_map_def {
    // Initialized at load time from the BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE
    // dependency represented by the original __uint macro.
    type_: 0,
    key: core::ptr::null_mut(),
    value: core::ptr::null_mut(),
};

#[repr(C)]
pub struct prog_array_def {
    // __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    pub type_: __u32,
    // __uint(max_entries, 1);
    pub max_entries: __u32,
    // __uint(key_size, sizeof(__u32));
    pub key_size: __u32,
    // __uint(value_size, sizeof(__u32));
    pub value_size: __u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut prog_array: prog_array_def = prog_array_def {
    // Initialized at load time from the BPF_MAP_TYPE_PROG_ARRAY dependency
    // represented by the original __uint macro.
    type_: 0,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u32>() as __u32,
};

extern "C" {
    pub fn bpf_get_local_storage(map: *mut storage_map_def, flags: __u64) -> *mut core::ffi::c_void;
    pub fn bpf_tail_call(ctx: *mut __sk_buff, prog_array_map: *mut prog_array_def, index: __u32);
}

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn caller_prog(skb: *mut __sk_buff) -> i32 {
    let mut storage: *mut __u64;

    storage = bpf_get_local_storage(&mut storage_map, 0) as *mut __u64;
    if !storage.is_null() {
        *storage = 1;
    }

    bpf_tail_call(skb, &mut prog_array, 0);
    return 1;
}

#[no_mangle]
#[link_section = "cgroup_skb/egress"]
pub unsafe extern "C" fn callee_prog(skb: *mut __sk_buff) -> i32 {
    let mut storage: *mut __u64;

    storage = bpf_get_local_storage(&mut storage_map, 0) as *mut __u64;
    if !storage.is_null() {
        *storage = 1;
    }

    return 1;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
