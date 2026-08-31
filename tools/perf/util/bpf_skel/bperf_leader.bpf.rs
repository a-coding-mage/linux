// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2021 Facebook
// Translated from C. Original dependencies: "vmlinux.h",
// <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

type __u32 = u32;

extern "C" {
    static BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32;
    static BPF_MAP_TYPE_PERCPU_ARRAY: __u32;
    static BPF_F_PRESERVE_ELEMS: __u32;

    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_perf_event_read_value(
        map: *mut core::ffi::c_void,
        flags: __u32,
        buf: *mut core::ffi::c_void,
        buf_size: __u32,
    ) -> i64;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_perf_event_value {
    pub counter: u64,
    pub enabled: u64,
    pub running: u64,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
}

// SEC(".maps")
#[no_mangle]
pub static mut events: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_PERF_EVENT_ARRAY
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<i32>() as __u32,
    max_entries: 0,
    map_flags: 0, // BPF_F_PRESERVE_ELEMS
};

// SEC(".maps")
#[no_mangle]
pub static mut prev_readings: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_PERCPU_ARRAY
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<bpf_perf_event_value>() as __u32,
    max_entries: 1,
    map_flags: 0,
};

// SEC(".maps")
#[no_mangle]
pub static mut diff_readings: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_PERCPU_ARRAY
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<bpf_perf_event_value>() as __u32,
    max_entries: 1,
    map_flags: 0,
};

// SEC("raw_tp/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn on_switch() -> i32 {
    let mut val: bpf_perf_event_value = core::mem::zeroed();
    let mut prev_val: *mut bpf_perf_event_value;
    let mut diff_val: *mut bpf_perf_event_value;
    let key: __u32 = bpf_get_smp_processor_id();
    let zero: __u32 = 0;
    let err: i64;

    prev_val = bpf_map_lookup_elem(
        &mut prev_readings as *mut _ as *mut core::ffi::c_void,
        &zero as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_perf_event_value;
    if prev_val.is_null() {
        return 0;
    }

    diff_val = bpf_map_lookup_elem(
        &mut diff_readings as *mut _ as *mut core::ffi::c_void,
        &zero as *const _ as *const core::ffi::c_void,
    ) as *mut bpf_perf_event_value;
    if diff_val.is_null() {
        return 0;
    }

    err = bpf_perf_event_read_value(
        &mut events as *mut _ as *mut core::ffi::c_void,
        key,
        &mut val as *mut _ as *mut core::ffi::c_void,
        core::mem::size_of::<bpf_perf_event_value>() as __u32,
    );
    if err != 0 {
        return 0;
    }

    (*diff_val).counter = val.counter.wrapping_sub((*prev_val).counter);
    (*diff_val).enabled = val.enabled.wrapping_sub((*prev_val).enabled);
    (*diff_val).running = val.running.wrapping_sub((*prev_val).running);
    *prev_val = val;
    0
}

// SEC("license")
#[no_mangle]
pub static mut LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";
