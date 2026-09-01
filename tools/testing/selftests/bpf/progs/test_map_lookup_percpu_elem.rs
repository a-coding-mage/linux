// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Bytedance */

// Dependencies from vmlinux.h and bpf/bpf_helpers.h are expected to be supplied
// by the surrounding BPF build environment.

pub type __u32 = u32;
pub type __u64 = u64;

pub const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;
pub const BPF_MAP_TYPE_PERCPU_HASH: u32 = 5;
pub const BPF_MAP_TYPE_LRU_PERCPU_HASH: u32 = 10;

#[no_mangle]
pub static mut percpu_array_elem_sum: __u64 = 0;
#[no_mangle]
pub static mut percpu_hash_elem_sum: __u64 = 0;
#[no_mangle]
pub static mut percpu_lru_hash_elem_sum: __u64 = 0;

extern "C" {
    pub static nr_cpus: core::ffi::c_int;
    pub static my_pid: core::ffi::c_int;

    pub fn bpf_map_lookup_percpu_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        cpu: __u32,
    ) -> *mut core::ffi::c_void;

    pub fn bpf_get_current_pid_tgid() -> __u64;

    pub fn bpf_loop(
        nr_loops: core::ffi::c_int,
        callback_fn: Option<
            unsafe extern "C" fn(__u32, *mut read_percpu_elem_ctx) -> core::ffi::c_int,
        >,
        callback_ctx: *mut read_percpu_elem_ctx,
        flags: __u64,
    ) -> core::ffi::c_long;
}

#[repr(C)]
pub struct percpu_array_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: __u32,
    pub value: __u64,
}

#[repr(C)]
pub struct percpu_hash_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: __u64,
    pub value: __u64,
}

#[repr(C)]
pub struct percpu_lru_hash_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key: __u64,
    pub value: __u64,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut percpu_array_map: percpu_array_map_def = percpu_array_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    max_entries: 1,
    key: 0,
    value: 0,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut percpu_hash_map: percpu_hash_map_def = percpu_hash_map_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    max_entries: 1,
    key: 0,
    value: 0,
};

#[no_mangle]
#[link_section = ".maps"]
pub static mut percpu_lru_hash_map: percpu_lru_hash_map_def = percpu_lru_hash_map_def {
    type_: BPF_MAP_TYPE_LRU_PERCPU_HASH,
    max_entries: 1,
    key: 0,
    value: 0,
};

#[repr(C)]
pub struct read_percpu_elem_ctx {
    pub map: *mut core::ffi::c_void,
    pub sum: __u64,
}

unsafe extern "C" fn read_percpu_elem_callback(
    index: __u32,
    ctx: *mut read_percpu_elem_ctx,
) -> core::ffi::c_int {
    let key: __u64 = 0;
    let mut value: *mut __u64;

    value = bpf_map_lookup_percpu_elem(
        (*ctx).map,
        &key as *const __u64 as *const core::ffi::c_void,
        index,
    ) as *mut __u64;
    if !value.is_null() {
        (*ctx).sum = (*ctx).sum.wrapping_add(*value);
    }
    0
}

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_getuid"]
pub unsafe extern "C" fn sysenter_getuid(ctx: *const core::ffi::c_void) -> core::ffi::c_int {
    let mut map_ctx: read_percpu_elem_ctx = core::mem::zeroed();

    let _ = ctx;

    if my_pid != (bpf_get_current_pid_tgid() >> 32) as core::ffi::c_int {
        return 0;
    }

    map_ctx.map = &mut percpu_array_map as *mut percpu_array_map_def as *mut core::ffi::c_void;
    map_ctx.sum = 0;
    bpf_loop(nr_cpus, Some(read_percpu_elem_callback), &mut map_ctx, 0);
    percpu_array_elem_sum = map_ctx.sum;

    map_ctx.map = &mut percpu_hash_map as *mut percpu_hash_map_def as *mut core::ffi::c_void;
    map_ctx.sum = 0;
    bpf_loop(nr_cpus, Some(read_percpu_elem_callback), &mut map_ctx, 0);
    percpu_hash_elem_sum = map_ctx.sum;

    map_ctx.map =
        &mut percpu_lru_hash_map as *mut percpu_lru_hash_map_def as *mut core::ffi::c_void;
    map_ctx.sum = 0;
    bpf_loop(nr_cpus, Some(read_percpu_elem_callback), &mut map_ctx, 0);
    percpu_lru_hash_elem_sum = map_ctx.sum;

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [core::ffi::c_char; 4] = [
    b'G' as core::ffi::c_char,
    b'P' as core::ffi::c_char,
    b'L' as core::ffi::c_char,
    0,
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
