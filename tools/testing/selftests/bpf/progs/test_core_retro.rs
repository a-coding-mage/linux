// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

const BPF_MAP_TYPE_ARRAY: u32 = 2;

#[repr(C)]
pub struct task_struct {
    pub tgid: i32,
}
// C attribute preserve_access_index applies to CO-RE field access for task_struct.

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut exp_tgid_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut results: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: core::mem::size_of::<i32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

extern "C" {
    fn bpf_get_current_task() -> *mut core::ffi::c_void;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

#[link_section = "tp/raw_syscalls/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn handle_sys_enter(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    let task: *mut task_struct = bpf_get_current_task() as *mut task_struct;
    let tgid: i32 = (*task).tgid;
    let zero: i32 = 0;
    let real_tgid: i32 = (bpf_get_current_pid_tgid() >> 32) as i32;
    let exp_tgid: *mut i32 = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(exp_tgid_map) as *mut core::ffi::c_void,
        core::ptr::addr_of!(zero) as *const core::ffi::c_void,
    ) as *mut i32;

    /* only pass through sys_enters from test process */
    if exp_tgid.is_null() || *exp_tgid != real_tgid {
        return 0;
    }

    bpf_map_update_elem(
        core::ptr::addr_of_mut!(results) as *mut core::ffi::c_void,
        core::ptr::addr_of!(zero) as *const core::ffi::c_void,
        core::ptr::addr_of!(tgid) as *const core::ffi::c_void,
        0,
    );

    return 0;
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
