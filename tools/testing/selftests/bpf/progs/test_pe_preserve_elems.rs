// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020 Facebook
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type c_int = i32;
type c_uint = u32;
type c_ulong = u64;

extern "C" {
    static BPF_MAP_TYPE_PERF_EVENT_ARRAY: c_uint;
    static BPF_F_PRESERVE_ELEMS: c_uint;

    fn bpf_perf_event_read_value(
        map: *mut core::ffi::c_void,
        flags: c_ulong,
        buf: *mut bpf_perf_event_value,
        buf_size: c_uint,
    ) -> c_int;
}

#[repr(C)]
pub struct bpf_perf_event_value {
    pub counter: u64,
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: c_uint,
    pub max_entries: c_uint,
    pub key_size: c_uint,
    pub value_size: c_uint,
    pub map_flags: c_uint,
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut array_1: bpf_map_def = bpf_map_def {
    type_: unsafe { BPF_MAP_TYPE_PERF_EVENT_ARRAY },
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as c_uint,
    value_size: core::mem::size_of::<c_int>() as c_uint,
    map_flags: 0,
};

#[link_section = ".maps"]
#[no_mangle]
pub static mut array_2: bpf_map_def = bpf_map_def {
    type_: unsafe { BPF_MAP_TYPE_PERF_EVENT_ARRAY },
    max_entries: 1,
    key_size: core::mem::size_of::<c_int>() as c_uint,
    value_size: core::mem::size_of::<c_int>() as c_uint,
    map_flags: unsafe { BPF_F_PRESERVE_ELEMS },
};

#[link_section = "raw_tp/sched_switch"]
#[no_mangle]
pub unsafe extern "C" fn read_array_1() -> c_int {
    let mut val: bpf_perf_event_value = core::mem::zeroed();

    bpf_perf_event_read_value(
        core::ptr::addr_of_mut!(array_1).cast::<core::ffi::c_void>(),
        0,
        core::ptr::addr_of_mut!(val),
        core::mem::size_of::<bpf_perf_event_value>() as c_uint,
    )
}

#[link_section = "raw_tp/task_rename"]
#[no_mangle]
pub unsafe extern "C" fn read_array_2() -> c_int {
    let mut val: bpf_perf_event_value = core::mem::zeroed();

    bpf_perf_event_read_value(
        core::ptr::addr_of_mut!(array_2).cast::<core::ffi::c_void>(),
        0,
        core::ptr::addr_of_mut!(val),
        core::mem::size_of::<bpf_perf_event_value>() as c_uint,
    )
}

#[link_section = "license"]
#[no_mangle]
pub static LICENSE: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
