// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type pid_t = i32;
type u64 = u64;

const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_STACK: u32 = 23;
const BPF_NOEXIST: u64 = 1;

#[repr(C)]
pub struct bpf_map_def {
    pub map_type: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = ".maps"]
#[no_mangle]
pub static mut hash_map: bpf_map_def = bpf_map_def {
    map_type: BPF_MAP_TYPE_HASH,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut stack_map: bpf_map_def = bpf_map_def {
    map_type: BPF_MAP_TYPE_STACK,
    max_entries: 1,
    key_size: 0,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut array_map: bpf_map_def = bpf_map_def {
    map_type: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[no_mangle]
pub static mut pid: pid_t = 0;
#[no_mangle]
pub static mut err: i64 = 0;

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_delete_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> i64;
    fn bpf_map_push_elem(
        map: *mut core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_map_pop_elem(
        map: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> i64;
    fn bpf_map_peek_elem(
        map: *mut core::ffi::c_void,
        value: *mut core::ffi::c_void,
    ) -> i64;
    fn bpf_for_each_map_elem(
        map: *mut core::ffi::c_void,
        callback: extern "C" fn(u64, u64, u64, u64, u64) -> u64,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

extern "C" fn callback(_map: u64, _key: u64, _val: u64, _ctx: u64, _flags: u64) -> u64 {
    return 0;
}

#[link_section = "tp/syscalls/sys_enter_getpid"]
#[no_mangle]
pub unsafe extern "C" fn map_update(_ctx: *mut core::ffi::c_void) -> i32 {
    let key: i32 = 0;
    let val: i32 = 1;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    err = bpf_map_update_elem(
        &mut hash_map as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &val as *const _ as *const core::ffi::c_void,
        BPF_NOEXIST,
    );

    return 0;
}

#[link_section = "tp/syscalls/sys_enter_getppid"]
#[no_mangle]
pub unsafe extern "C" fn map_delete(_ctx: *mut core::ffi::c_void) -> i32 {
    let key: i32 = 0;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    err = bpf_map_delete_elem(
        &mut hash_map as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    );

    return 0;
}

#[link_section = "tp/syscalls/sys_enter_getuid"]
#[no_mangle]
pub unsafe extern "C" fn map_push(_ctx: *mut core::ffi::c_void) -> i32 {
    let val: i32 = 1;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    err = bpf_map_push_elem(
        &mut stack_map as *mut _ as *mut core::ffi::c_void,
        &val as *const _ as *const core::ffi::c_void,
        0,
    );

    return 0;
}

#[link_section = "tp/syscalls/sys_enter_geteuid"]
#[no_mangle]
pub unsafe extern "C" fn map_pop(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut val: i32 = 0;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    err = bpf_map_pop_elem(
        &mut stack_map as *mut _ as *mut core::ffi::c_void,
        &mut val as *mut _ as *mut core::ffi::c_void,
    );

    return 0;
}

#[link_section = "tp/syscalls/sys_enter_getgid"]
#[no_mangle]
pub unsafe extern "C" fn map_peek(_ctx: *mut core::ffi::c_void) -> i32 {
    let mut val: i32 = 0;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    err = bpf_map_peek_elem(
        &mut stack_map as *mut _ as *mut core::ffi::c_void,
        &mut val as *mut _ as *mut core::ffi::c_void,
    );

    return 0;
}

#[link_section = "tp/syscalls/sys_enter_gettid"]
#[no_mangle]
pub unsafe extern "C" fn map_for_each_pass(_ctx: *mut core::ffi::c_void) -> i32 {
    let key: i32 = 0;
    let val: i32 = 1;
    let flags: u64 = 0;
    let mut callback_ctx: i32 = 0;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    bpf_map_update_elem(
        &mut array_map as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &val as *const _ as *const core::ffi::c_void,
        flags,
    );

    err = bpf_for_each_map_elem(
        &mut array_map as *mut _ as *mut core::ffi::c_void,
        callback,
        &mut callback_ctx as *mut _ as *mut core::ffi::c_void,
        flags,
    );

    return 0;
}

#[link_section = "tp/syscalls/sys_enter_getpgid"]
#[no_mangle]
pub unsafe extern "C" fn map_for_each_fail(_ctx: *mut core::ffi::c_void) -> i32 {
    let key: i32 = 0;
    let val: i32 = 1;
    let flags: u64 = BPF_NOEXIST;
    let mut callback_ctx: i32 = 0;

    if core::ptr::read_volatile(&pid) != (bpf_get_current_pid_tgid() >> 32) as pid_t {
        return 0;
    }

    bpf_map_update_elem(
        &mut array_map as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
        &val as *const _ as *const core::ffi::c_void,
        flags,
    );

    /* calling for_each with non-zero flags will return error */
    err = bpf_for_each_map_elem(
        &mut array_map as *mut _ as *mut core::ffi::c_void,
        callback,
        &mut callback_ctx as *mut _ as *mut core::ffi::c_void,
        flags,
    );

    return 0;
}
