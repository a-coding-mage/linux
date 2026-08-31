// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2020 Facebook
// Rust translation of perf/util/bpf_skel/bpf_prog_profiler.bpf.c.
// Dependencies originally supplied by vmlinux.h, bpf_helpers.h, and
// bpf_tracing.h are referenced as external BPF ABI items.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __u32 = u32;

const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;

#[repr(C)]
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
}

unsafe extern "C" {
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_map_lookup_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn bpf_perf_event_read_value(
        map: *mut core::ffi::c_void,
        flags: u64,
        buf: *mut core::ffi::c_void,
        buf_size: __u32,
    ) -> i64;
}

/* map of perf event fds, num_cpu * num_metric entries */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut events: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERF_EVENT_ARRAY,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<i32>() as __u32,
    max_entries: 0,
};

/* readings at fentry */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut fentry_readings: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<bpf_perf_event_value>() as __u32,
    max_entries: 1,
};

/* accumulated readings */
#[unsafe(link_section = ".maps")]
#[unsafe(no_mangle)]
pub static mut accum_readings: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<bpf_perf_event_value>() as __u32,
    max_entries: 1,
};

#[unsafe(no_mangle)]
pub static mut num_cpu: __u32 = 1;

#[unsafe(link_section = "fentry/XXX")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fentry_XXX() -> i32 {
    let key: __u32 = unsafe { bpf_get_smp_processor_id() };
    let mut ptr: *mut bpf_perf_event_value;
    let zero: __u32 = 0;
    let err: i64;

    /* look up before reading, to reduce error */
    ptr = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(fentry_readings).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(zero).cast::<core::ffi::c_void>(),
        )
        .cast::<bpf_perf_event_value>()
    };
    if ptr.is_null() {
        return 0;
    }

    err = unsafe {
        bpf_perf_event_read_value(
            core::ptr::addr_of_mut!(events).cast::<core::ffi::c_void>(),
            key as u64,
            ptr.cast::<core::ffi::c_void>(),
            core::mem::size_of::<bpf_perf_event_value>() as __u32,
        )
    };
    if err != 0 {
        return 0;
    }

    0
}

#[inline(always)]
unsafe fn fexit_update_maps(after: *mut bpf_perf_event_value) {
    let mut before: *mut bpf_perf_event_value;
    let mut diff: bpf_perf_event_value = bpf_perf_event_value {
        counter: 0,
        enabled: 0,
        running: 0,
    };
    let zero: __u32 = 0;

    before = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(fentry_readings).cast::<core::ffi::c_void>(),
            core::ptr::addr_of!(zero).cast::<core::ffi::c_void>(),
        )
        .cast::<bpf_perf_event_value>()
    };
    /* only account samples with a valid fentry_reading */
    if !before.is_null() && unsafe { (*before).counter } != 0 {
        let accum: *mut bpf_perf_event_value;

        diff.counter = unsafe { (*after).counter.wrapping_sub((*before).counter) };
        diff.enabled = unsafe { (*after).enabled.wrapping_sub((*before).enabled) };
        diff.running = unsafe { (*after).running.wrapping_sub((*before).running) };

        accum = unsafe {
            bpf_map_lookup_elem(
                core::ptr::addr_of_mut!(accum_readings).cast::<core::ffi::c_void>(),
                core::ptr::addr_of!(zero).cast::<core::ffi::c_void>(),
            )
            .cast::<bpf_perf_event_value>()
        };
        if !accum.is_null() {
            unsafe {
                (*accum).counter = (*accum).counter.wrapping_add(diff.counter);
                (*accum).enabled = (*accum).enabled.wrapping_add(diff.enabled);
                (*accum).running = (*accum).running.wrapping_add(diff.running);
            }
        }
    }
}

#[unsafe(link_section = "fexit/XXX")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fexit_XXX() -> i32 {
    let mut reading: bpf_perf_event_value = bpf_perf_event_value {
        counter: 0,
        enabled: 0,
        running: 0,
    };
    let cpu: __u32 = unsafe { bpf_get_smp_processor_id() };
    let err: i32;

    /* read all events before updating the maps, to reduce error */
    err = unsafe {
        bpf_perf_event_read_value(
            core::ptr::addr_of_mut!(events).cast::<core::ffi::c_void>(),
            cpu as u64,
            core::ptr::addr_of_mut!(reading).cast::<core::ffi::c_void>(),
            core::mem::size_of::<bpf_perf_event_value>() as __u32,
        ) as i32
    };
    if err != 0 {
        return 0;
    }

    unsafe {
        fexit_update_maps(core::ptr::addr_of_mut!(reading));
    }
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";
