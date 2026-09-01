// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2023. Huawei Technologies Co., Ltd */

/*
 * Translated from C eBPF source. Original dependencies:
 * <linux/bpf.h>, <time.h>, <bpf/bpf_helpers.h>, and "bpf_misc.h".
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u64 = u64;

const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_ARRAY_OF_MAPS: u32 = 12;
const BPF_MAP_TYPE_HASH_OF_MAPS: u32 = 13;
const SYS_PREFIX: &str = "";

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
}

#[repr(C)]
pub struct inner_map_type {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut inner_map: inner_map_type = inner_map_type {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: 4,
    value_size: 4,
    max_entries: 1,
};

#[repr(C)]
pub struct outer_map_type {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub values: [*mut inner_map_type; 1],
}

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut outer_array_map: outer_map_type = outer_map_type {
    type_: BPF_MAP_TYPE_ARRAY_OF_MAPS,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 1,
    values: [core::ptr::addr_of_mut!(inner_map)],
};

#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut outer_htab_map: outer_map_type = outer_map_type {
    type_: BPF_MAP_TYPE_HASH_OF_MAPS,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
    max_entries: 1,
    values: [core::ptr::addr_of_mut!(inner_map)],
};

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut tgid: i32 = 0;

unsafe fn acc_map_in_map(outer_map: *mut core::ffi::c_void) -> i32 {
    let mut i: i32;
    let mut key: i32;
    let value: i32 = 0xdeadbeefu32 as i32;
    let mut inner_map_ptr: *mut core::ffi::c_void;

    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { tgid } as u64 {
        return 0;
    }

    /* Find nonexistent inner map */
    key = 1;
    inner_map_ptr = unsafe {
        bpf_map_lookup_elem(
            outer_map,
            (&raw const key).cast::<core::ffi::c_void>(),
        )
    };
    if !inner_map_ptr.is_null() {
        return 0;
    }

    /* Find the old inner map */
    key = 0;
    inner_map_ptr = unsafe {
        bpf_map_lookup_elem(
            outer_map,
            (&raw const key).cast::<core::ffi::c_void>(),
        )
    };
    if inner_map_ptr.is_null() {
        return 0;
    }

    /* Wait for the old inner map to be replaced */
    i = 0;
    while i < 2048 {
        unsafe {
            bpf_map_update_elem(
                inner_map_ptr,
                (&raw const key).cast::<core::ffi::c_void>(),
                (&raw const value).cast::<core::ffi::c_void>(),
                0,
            );
        }
        i += 1;
    }

    0
}

/* SEC("?kprobe/" SYS_PREFIX "sys_getpgid") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn access_map_in_array(_ctx: *mut core::ffi::c_void) -> i32 {
    unsafe { acc_map_in_map(core::ptr::addr_of_mut!(outer_array_map).cast::<core::ffi::c_void>()) }
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sleepable_access_map_in_array(_ctx: *mut core::ffi::c_void) -> i32 {
    unsafe { acc_map_in_map(core::ptr::addr_of_mut!(outer_array_map).cast::<core::ffi::c_void>()) }
}

/* SEC("?kprobe/" SYS_PREFIX "sys_getpgid") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn access_map_in_htab(_ctx: *mut core::ffi::c_void) -> i32 {
    unsafe { acc_map_in_map(core::ptr::addr_of_mut!(outer_htab_map).cast::<core::ffi::c_void>()) }
}

/* SEC("?fentry.s/" SYS_PREFIX "sys_getpgid") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sleepable_access_map_in_htab(_ctx: *mut core::ffi::c_void) -> i32 {
    unsafe { acc_map_in_map(core::ptr::addr_of_mut!(outer_htab_map).cast::<core::ffi::c_void>()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
