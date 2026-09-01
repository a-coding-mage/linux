// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2021 Facebook
// Copyright (c) 2021 Google
//
// Rust translation of bperf_cgroup.bpf.c. C include dependencies:
// "bperf_cgroup.h", "vmlinux.h", <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, and <bpf/bpf_core_read.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::c_void;

type __u32 = u32;
type __u64 = u64;
type u64 = u64;

const BPF_MAP_TYPE_PERF_EVENT_ARRAY: __u32 = 4;
const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 6;
const BPF_ANY: __u64 = 0;

// From bperf_cgroup.h.
extern "C" {
    static BPERF_CGROUP__MAX_LEVELS: i32;
    static BPERF_CGROUP__MAX_EVENTS: __u32;
}

#[repr(C)]
pub struct bpf_perf_event_value {
    pub counter: __u64,
    pub enabled: __u64,
    pub running: __u64,
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kernfs_node {
    pub id: __u64,
}

#[repr(C)]
pub struct cgroup_with_kn {
    pub kn: *mut kernfs_node,
}

#[repr(C)]
pub struct cgroup_subsys_state {
    pub cgroup: *mut cgroup,
}

#[repr(C)]
pub struct css_set {
    pub subsys: [*mut cgroup_subsys_state; 0],
}

#[repr(C)]
pub struct task_struct_core {
    pub cgroups: *mut css_set,
}

#[repr(C)]
pub struct cgroup_core {
    pub level: i32,
}

// BPF map declarations translated from anonymous SEC(".maps") structs.
// The section/name metadata is represented as comments because Rust attributes
// for libbpf map definition macros are supplied externally in the original C.

// NOTE: many of map and global data will be modified before loading
//       from the userspace (perf tool) using the skeleton helpers.

#[repr(C)]
pub struct events_map {
    // __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    // __uint(key_size, sizeof(__u32));
    // __uint(value_size, sizeof(int));
    // __uint(max_entries, 1);
    _private: [u8; 0],
}

// single set of global perf events to measure
#[no_mangle]
pub static mut events: events_map = events_map { _private: [] };

#[repr(C)]
pub struct cgrp_idx_map {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __uint(key_size, sizeof(__u64));
    // __uint(value_size, sizeof(__u32));
    // __uint(max_entries, 1);
    _private: [u8; 0],
}

// from cgroup id to event index
#[no_mangle]
pub static mut cgrp_idx: cgrp_idx_map = cgrp_idx_map { _private: [] };

#[repr(C)]
pub struct prev_readings_map {
    // __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    // __uint(key_size, sizeof(__u32));
    // __uint(value_size, sizeof(struct bpf_perf_event_value));
    _private: [u8; 0],
}

// per-cpu event snapshots to calculate delta
#[no_mangle]
pub static mut prev_readings: prev_readings_map = prev_readings_map { _private: [] };

#[repr(C)]
pub struct cgrp_readings_map {
    // __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    // __uint(key_size, sizeof(__u32));
    // __uint(value_size, sizeof(struct bpf_perf_event_value));
    _private: [u8; 0],
}

// aggregated event values for each cgroup (per-cpu)
// will be read from the user-space
#[no_mangle]
pub static mut cgrp_readings: cgrp_readings_map = cgrp_readings_map { _private: [] };

/* new kernel cgroup definition */
#[repr(C)]
pub struct cgroup___new {
    pub level: i32,
    pub ancestors: [*mut cgroup_with_kn; 0],
}

/* old kernel cgroup definition */
#[repr(C)]
pub struct cgroup___old {
    pub level: i32,
    pub ancestor_ids: [u64; 0],
}

#[no_mangle]
pub static mut num_events: __u32 = 1;
#[no_mangle]
pub static mut num_cpus: __u32 = 1;
#[no_mangle]
pub static mut use_cgroup_v2: i32 = 0;

#[no_mangle]
pub static mut enabled: i32 = 0;
#[no_mangle]
pub static mut perf_subsys_id: i32 = -1;

extern "C" {
    fn bpf_get_current_task() -> *mut c_void;
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_get_current_ancestor_cgroup_id(level: i32) -> __u64;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_perf_event_read_value(
        map: *mut c_void,
        flags: __u64,
        buf: *mut bpf_perf_event_value,
        buf_size: __u32,
    ) -> i64;

    // From enum cgroup_subsys_id / bpf_core_enum_value fallback.
    static perf_event_cgrp_id: i32;
}

unsafe fn bpf_core_field_exists_cgroup_new_ancestors() -> bool {
    // Represents bpf_core_field_exists(cgrp_new->ancestors). The real value is
    // resolved by CO-RE/BPF tooling outside this isolated translation.
    true
}

unsafe fn get_cgroup_v1_ancestor_id(cgrp: *mut cgroup, level: i32) -> __u64 {
    /* recast pointer to capture new type for compiler */
    let cgrp_new = cgrp as *mut cgroup___new;

    if bpf_core_field_exists_cgroup_new_ancestors() {
        let ancestor = *(*cgrp_new).ancestors.as_ptr().offset(level as isize);
        let kn = (*ancestor).kn;
        (*kn).id
    } else {
        /* recast pointer to capture old type for compiler */
        let cgrp_old = cgrp as *mut cgroup___old;

        *(*cgrp_old).ancestor_ids.as_ptr().offset(level as isize)
    }
}

unsafe fn get_cgroup_v1_idx(cgrps: *mut __u32, size: i32) -> i32 {
    let p = bpf_get_current_task() as *mut task_struct;
    let mut cgrp: *mut cgroup;
    let mut i: i32 = 0;
    let mut elem: *mut __u32;
    let level: i32;
    let mut cnt: i32;

    if perf_subsys_id == -1 {
        // C used:
        // #if __has_builtin(__builtin_preserve_enum_value)
        //     perf_subsys_id = bpf_core_enum_value(enum cgroup_subsys_id,
        //                                          perf_event_cgrp_id);
        // #else
        //     perf_subsys_id = perf_event_cgrp_id;
        // #endif
        perf_subsys_id = perf_event_cgrp_id;
    }

    let p_core = p as *mut task_struct_core;
    let css = *(*(*p_core).cgroups)
        .subsys
        .as_ptr()
        .offset(perf_subsys_id as isize);
    cgrp = (*css).cgroup;
    level = (*(cgrp as *mut cgroup_core)).level;

    cnt = 0;
    while i < BPERF_CGROUP__MAX_LEVELS {
        let cgrp_id: __u64;

        if i > level {
            break;
        }

        // convert cgroup-id to a map index
        cgrp_id = get_cgroup_v1_ancestor_id(cgrp, i);
        elem = bpf_map_lookup_elem(
            &mut cgrp_idx as *mut _ as *mut c_void,
            &cgrp_id as *const _ as *const c_void,
        ) as *mut __u32;
        if elem.is_null() {
            i += 1;
            continue;
        }

        *cgrps.offset(cnt as isize) = *elem;
        cnt += 1;
        if cnt == size {
            break;
        }

        i += 1;
    }

    cnt
}

unsafe fn get_cgroup_v2_idx(cgrps: *mut __u32, size: i32) -> i32 {
    let mut i: i32 = 0;
    let mut elem: *mut __u32;
    let mut cnt: i32;

    cnt = 0;
    while i < BPERF_CGROUP__MAX_LEVELS {
        let cgrp_id: __u64 = bpf_get_current_ancestor_cgroup_id(i);

        if cgrp_id == 0 {
            break;
        }

        // convert cgroup-id to a map index
        elem = bpf_map_lookup_elem(
            &mut cgrp_idx as *mut _ as *mut c_void,
            &cgrp_id as *const _ as *const c_void,
        ) as *mut __u32;
        if elem.is_null() {
            i += 1;
            continue;
        }

        *cgrps.offset(cnt as isize) = *elem;
        cnt += 1;
        if cnt == size {
            break;
        }

        i += 1;
    }

    cnt
}

unsafe fn bperf_cgroup_count() -> i32 {
    let mut idx: __u32 = 0; // to have it in a register to pass BPF verifier
    let mut c: i32;
    let mut val = bpf_perf_event_value {
        counter: 0,
        enabled: 0,
        running: 0,
    };
    let mut delta = bpf_perf_event_value {
        counter: 0,
        enabled: 0,
        running: 0,
    };
    let mut prev_val: *mut bpf_perf_event_value;
    let mut cgrp_val: *mut bpf_perf_event_value;
    let cpu: __u32 = bpf_get_smp_processor_id();
    let mut cgrp_idx_local: [__u32; BPERF_CGROUP__MAX_LEVELS as usize] =
        [0; BPERF_CGROUP__MAX_LEVELS as usize];
    let cgrp_cnt: i32;
    let mut key: __u32;
    let mut cgrp: __u32;
    let mut err: i64;

    if use_cgroup_v2 != 0 {
        cgrp_cnt = get_cgroup_v2_idx(
            cgrp_idx_local.as_mut_ptr(),
            BPERF_CGROUP__MAX_LEVELS,
        );
    } else {
        cgrp_cnt = get_cgroup_v1_idx(
            cgrp_idx_local.as_mut_ptr(),
            BPERF_CGROUP__MAX_LEVELS,
        );
    }

    while idx < BPERF_CGROUP__MAX_EVENTS {
        if idx == num_events {
            break;
        }

        // XXX: do not pass idx directly (for verifier)
        key = idx;
        // this is per-cpu array for diff
        prev_val = bpf_map_lookup_elem(
            &mut prev_readings as *mut _ as *mut c_void,
            &key as *const _ as *const c_void,
        ) as *mut bpf_perf_event_value;
        if prev_val.is_null() {
            val.counter = 0;
            val.enabled = 0;
            val.running = 0;
            bpf_map_update_elem(
                &mut prev_readings as *mut _ as *mut c_void,
                &key as *const _ as *const c_void,
                &val as *const _ as *const c_void,
                BPF_ANY,
            );

            prev_val = bpf_map_lookup_elem(
                &mut prev_readings as *mut _ as *mut c_void,
                &key as *const _ as *const c_void,
            ) as *mut bpf_perf_event_value;
            if prev_val.is_null() {
                idx += 1;
                continue;
            }
        }

        // read from global perf_event array
        key = idx.wrapping_mul(num_cpus).wrapping_add(cpu);
        err = bpf_perf_event_read_value(
            &mut events as *mut _ as *mut c_void,
            key as __u64,
            &mut val,
            core::mem::size_of::<bpf_perf_event_value>() as __u32,
        );
        if err != 0 {
            idx += 1;
            continue;
        }

        if enabled != 0 {
            delta.counter = val.counter.wrapping_sub((*prev_val).counter);
            delta.enabled = val.enabled.wrapping_sub((*prev_val).enabled);
            delta.running = val.running.wrapping_sub((*prev_val).running);

            c = 0;
            while c < BPERF_CGROUP__MAX_LEVELS {
                if c == cgrp_cnt {
                    break;
                }

                cgrp = cgrp_idx_local[c as usize];

                // aggregate the result by cgroup
                key = cgrp.wrapping_mul(num_events).wrapping_add(idx);
                cgrp_val = bpf_map_lookup_elem(
                    &mut cgrp_readings as *mut _ as *mut c_void,
                    &key as *const _ as *const c_void,
                ) as *mut bpf_perf_event_value;
                if !cgrp_val.is_null() {
                    (*cgrp_val).counter = (*cgrp_val).counter.wrapping_add(delta.counter);
                    (*cgrp_val).enabled = (*cgrp_val).enabled.wrapping_add(delta.enabled);
                    (*cgrp_val).running = (*cgrp_val).running.wrapping_add(delta.running);
                } else {
                    bpf_map_update_elem(
                        &mut cgrp_readings as *mut _ as *mut c_void,
                        &key as *const _ as *const c_void,
                        &delta as *const _ as *const c_void,
                        BPF_ANY,
                    );
                }

                c += 1;
            }
        }

        *prev_val = val;
        idx += 1;
    }
    0
}

// This will be attached to cgroup-switches event for each cpu
// SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn on_cgrp_switch() -> i32 {
    bperf_cgroup_count()
}

// SEC("raw_tp/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn trigger_read() -> i32 {
    bperf_cgroup_count()
}

// SEC("license")
#[no_mangle]
pub static LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
