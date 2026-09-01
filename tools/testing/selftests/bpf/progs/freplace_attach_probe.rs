// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook

// C dependencies:
// #include <linux/ptrace.h>
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

pub const VAR_NUM: usize = 2;
pub const BPF_MAP_TYPE_HASH: u32 = 1;

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

pub type __u32 = u32;

#[repr(C)]
pub struct hmap_elem {
    pub lock: bpf_spin_lock,
    pub var: [i32; VAR_NUM],
}

#[repr(C)]
pub struct hash_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

// Original C used BPF map declaration macros:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(max_entries, 1);
//     __type(key, __u32);
//     __type(value, struct hmap_elem);
// } hash_map SEC(".maps");
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static hash_map: hash_map_def = hash_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<hmap_elem>() as u32,
};

unsafe extern "C" {
    pub fn bpf_map_lookup_elem(
        map: *const core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    pub fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    pub fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
}

#[unsafe(link_section = "freplace/handle_kprobe")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn new_handle_kprobe(ctx: *mut pt_regs) -> i32 {
    let mut val: *mut hmap_elem;
    let key: i32 = 0;

    let _ = ctx;
    val = unsafe {
        bpf_map_lookup_elem(
            &hash_map as *const hash_map_def as *const core::ffi::c_void,
            &key as *const i32 as *const core::ffi::c_void,
        ) as *mut hmap_elem
    };
    if val.is_null() {
        return 1;
    }
    /* spin_lock in hash map */
    unsafe {
        bpf_spin_lock(&mut (*val).lock);
        (*val).var[0] = 99;
        bpf_spin_unlock(&mut (*val).lock);
    }

    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
