// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies translated as external Rust dependencies:
// linux/ptrace.h, linux/bpf.h, bpf/bpf_helpers.h, bpf/bpf_tracing.h

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type c_int = i32;
type c_long = i64;

// Constants supplied by linux/bpf.h in the original C translation unit.
const BPF_MAP_TYPE_ARRAY: u32 = 2;
const BPF_MAP_TYPE_PERF_EVENT_ARRAY: u32 = 4;
const BPF_F_CURRENT_CPU: u64 = 0xffff_ffff;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub map_flags: u32,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut my_pid_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<c_int>() as u32,
    max_entries: 1,
    map_flags: 0,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut perf_buf_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERF_EVENT_ARRAY,
    key_size: core::mem::size_of::<c_int>() as u32,
    value_size: core::mem::size_of::<c_int>() as u32,
    max_entries: 0,
    map_flags: 0,
};

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_perf_event_output(
        ctx: *mut c_void,
        map: *mut c_void,
        flags: u64,
        data: *const c_void,
        size: u64,
    ) -> c_long;
}

#[link_section = "tp/raw_syscalls/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn handle_sys_enter(ctx: *mut c_void) -> c_int {
    let mut zero: c_int = 0;
    let my_pid: *mut c_int;
    let cur_pid: c_int;
    let cpu: c_int = bpf_get_smp_processor_id() as c_int;

    my_pid = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(my_pid_map).cast::<c_void>(),
        core::ptr::addr_of_mut!(zero).cast::<c_void>(),
    )
    .cast::<c_int>();
    if my_pid.is_null() {
        return 1;
    }

    cur_pid = (bpf_get_current_pid_tgid() >> 32) as c_int;
    if cur_pid != *my_pid {
        return 1;
    }

    bpf_perf_event_output(
        ctx,
        core::ptr::addr_of_mut!(perf_buf_map).cast::<c_void>(),
        BPF_F_CURRENT_CPU,
        core::ptr::addr_of!(cpu).cast::<c_void>(),
        core::mem::size_of_val(&cpu) as u64,
    );
    return 1;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
