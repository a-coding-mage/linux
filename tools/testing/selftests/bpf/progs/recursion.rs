// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

// C dependencies translated from:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

pub const BPF_MAP_TYPE_HASH: u32 = 1;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct HashMapDef {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut hash1: HashMapDef = HashMapDef {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut hash2: HashMapDef = HashMapDef {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i64>() as u32,
};

#[unsafe(no_mangle)]
pub static mut pass1: i32 = 0;

#[unsafe(no_mangle)]
pub static mut pass2: i32 = 0;

#[unsafe(link_section = "fentry/htab_map_delete_elem")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_delete(map: *mut bpf_map) -> i32 {
    let key: i32 = 0;

    if map == (&raw mut hash1).cast::<core::ffi::c_void>().cast::<bpf_map>() {
        pass1 += 1;
        return 0;
    }
    if map == (&raw mut hash2).cast::<core::ffi::c_void>().cast::<bpf_map>() {
        pass2 += 1;
        bpf_map_delete_elem(
            (&raw mut hash2).cast::<core::ffi::c_void>(),
            (&key as *const i32).cast::<core::ffi::c_void>(),
        );
        return 0;
    }

    return 0;
}
