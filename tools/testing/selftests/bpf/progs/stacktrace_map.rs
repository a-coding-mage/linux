// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2018 Facebook

// Rust translation of dependencies from:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>

pub const PERF_MAX_STACK_DEPTH: usize = 127;

pub type stack_trace_t = [__u64; PERF_MAX_STACK_DEPTH];

#[link_section = ".maps"]
pub static mut control_map: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 1,
    key_size: ::core::mem::size_of::<__u32>() as __u32,
    value_size: ::core::mem::size_of::<__u32>() as __u32,
};

#[link_section = ".maps"]
pub static mut stackid_hmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    max_entries: 16384,
    key_size: ::core::mem::size_of::<__u32>() as __u32,
    value_size: ::core::mem::size_of::<__u32>() as __u32,
};

#[link_section = ".maps"]
pub static mut stackmap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_STACK_TRACE,
    max_entries: 16384,
    key_size: ::core::mem::size_of::<__u32>() as __u32,
    value_size: ::core::mem::size_of::<stack_trace_t>() as __u32,
};

#[link_section = ".maps"]
pub static mut stack_amap: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    max_entries: 16384,
    key_size: ::core::mem::size_of::<__u32>() as __u32,
    value_size: ::core::mem::size_of::<stack_trace_t>() as __u32,
};

/* taken from /sys/kernel/tracing/events/sched/sched_switch/format */
#[repr(C)]
pub struct sched_switch_args {
    pub pad: ::core::ffi::c_ulonglong,
    pub prev_comm: [::core::ffi::c_char; TASK_COMM_LEN],
    pub prev_pid: ::core::ffi::c_int,
    pub prev_prio: ::core::ffi::c_int,
    pub prev_state: ::core::ffi::c_longlong,
    pub next_comm: [::core::ffi::c_char; TASK_COMM_LEN],
    pub next_pid: ::core::ffi::c_int,
    pub next_prio: ::core::ffi::c_int,
}

pub static mut stack_id: __u32 = 0;

extern "C" {
    pub fn bpf_map_lookup_elem(map: *mut bpf_map_def, key: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn bpf_get_stackid(
        ctx: *mut sched_switch_args,
        map: *mut bpf_map_def,
        flags: __u64,
    ) -> ::core::ffi::c_long;
    pub fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const ::core::ffi::c_void,
        value: *const ::core::ffi::c_void,
        flags: __u64,
    ) -> ::core::ffi::c_long;
    pub fn bpf_get_stack(
        ctx: *mut sched_switch_args,
        buf: *mut ::core::ffi::c_void,
        size: __u32,
        flags: __u64,
    ) -> ::core::ffi::c_long;
}

#[link_section = "tracepoint/sched/sched_switch"]
pub unsafe extern "C" fn oncpu(ctx: *mut sched_switch_args) -> ::core::ffi::c_int {
    let max_len: __u32 = (PERF_MAX_STACK_DEPTH * ::core::mem::size_of::<__u64>()) as __u32;
    let mut key: __u32 = 0;
    let val: __u32 = 0;
    let mut value_p: *mut __u32;
    let mut stack_p: *mut ::core::ffi::c_void;

    value_p = bpf_map_lookup_elem(
        &mut control_map,
        &key as *const __u32 as *const ::core::ffi::c_void,
    ) as *mut __u32;
    if !value_p.is_null() && *value_p != 0 {
        return 0; /* skip if non-zero *value_p */
    }

    /* The size of stackmap and stackid_hmap should be the same */
    key = bpf_get_stackid(ctx, &mut stackmap, 0) as __u32;
    if (key as ::core::ffi::c_int) >= 0 {
        stack_id = key;
        bpf_map_update_elem(
            &mut stackid_hmap,
            &key as *const __u32 as *const ::core::ffi::c_void,
            &val as *const __u32 as *const ::core::ffi::c_void,
            0,
        );
        stack_p = bpf_map_lookup_elem(
            &mut stack_amap,
            &key as *const __u32 as *const ::core::ffi::c_void,
        );
        if !stack_p.is_null() {
            bpf_get_stack(ctx, stack_p, max_len, 0);
        }
    }

    return 0;
}

#[link_section = "license"]
pub static mut _license: [::core::ffi::c_char; 4] = [
    b'G' as ::core::ffi::c_char,
    b'P' as ::core::ffi::c_char,
    b'L' as ::core::ffi::c_char,
    0,
];
