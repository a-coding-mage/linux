// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

/* Dependencies from the original C source:
 * - vmlinux.h
 * - bpf/bpf_helpers.h
 * - bpf_misc.h
 *
 * BPF section annotations from SEC(...) are represented with link_section
 * attributes where Rust can express them directly.
 */

type __u32 = u32;
type __u64 = u64;
type u32 = __u32;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_HASH_OF_MAPS: u32 = 12;
const BPF_F_MMAPABLE: u32 = 1024;

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct inner_array_type {
    /* __uint(type, BPF_MAP_TYPE_ARRAY); */
    pub type_: u32,
    /* __uint(map_flags, BPF_F_MMAPABLE); */
    pub map_flags: u32,
    /* __type(key, __u32); */
    pub key_size: u32,
    /* __type(value, __u64); */
    pub value_size: u32,
    /* __uint(max_entries, 1); */
    pub max_entries: u32,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut inner_array: inner_array_type = inner_array_type {
    type_: BPF_MAP_TYPE_ARRAY,
    map_flags: BPF_F_MMAPABLE,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
    max_entries: 1,
};

#[repr(C)]
pub struct outer_map_type {
    /* __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS); */
    pub type_: u32,
    /* __uint(key_size, 4); */
    pub key_size: u32,
    /* __uint(value_size, 4); */
    pub value_size: u32,
    /* __uint(max_entries, 1); */
    pub max_entries: u32,
    /* __array(values, struct inner_array_type); */
    pub values: *mut inner_array_type,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut outer_map: outer_map_type = outer_map_type {
    type_: BPF_MAP_TYPE_HASH_OF_MAPS,
    key_size: 4,
    value_size: 4,
    max_entries: 1,
    values: unsafe { &raw mut inner_array },
};

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;
#[unsafe(no_mangle)]
pub static mut match_value: __u64 = 0x13572468;
#[unsafe(no_mangle)]
pub static mut done: bool = false;
#[unsafe(no_mangle)]
pub static mut pid_match: bool = false;
#[unsafe(no_mangle)]
pub static mut outer_map_match: bool = false;

/* Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep") */
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/sys_nanosleep")]
pub unsafe extern "C" fn add_to_list_in_inner_array(ctx: *mut core::ffi::c_void) -> i32 {
    let mut curr_pid: __u32;
    let zero: __u32 = 0;
    let mut map: *mut bpf_map;
    let mut value: *mut __u64;

    let _ = ctx;

    curr_pid = bpf_get_current_pid_tgid() as u32;
    if done || curr_pid != pid as __u32 {
        return 0;
    }

    pid_match = true;
    map = bpf_map_lookup_elem(
        &raw mut outer_map as *mut core::ffi::c_void,
        &curr_pid as *const __u32 as *const core::ffi::c_void,
    ) as *mut bpf_map;
    if map.is_null() {
        return 0;
    }

    outer_map_match = true;
    value = bpf_map_lookup_elem(
        map as *mut core::ffi::c_void,
        &zero as *const __u32 as *const core::ffi::c_void,
    ) as *mut __u64;
    if value.is_null() {
        return 0;
    }

    *value = match_value;
    done = true;
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
