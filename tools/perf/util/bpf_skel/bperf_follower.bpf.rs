// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2021 Facebook
//
// Translated from C. Original dependencies:
// - "vmlinux.h"
// - <bpf/bpf_helpers.h>
// - <bpf/bpf_tracing.h>
// - "bperf_u.h"

pub const MAX_ENTRIES: u32 = 102400;

// BPF map declaration equivalent:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
//     __uint(key_size, sizeof(__u32));
//     __uint(value_size, sizeof(struct bpf_perf_event_value));
//     __uint(max_entries, 1);
// } diff_readings SEC(".maps");
#[repr(C)]
pub struct diff_readings {
    _private: [u8; 0],
}

// BPF map declaration equivalent:
// struct {
//     __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
//     __uint(key_size, sizeof(__u32));
//     __uint(value_size, sizeof(struct bpf_perf_event_value));
//     __uint(max_entries, 1);
// } accum_readings SEC(".maps");
#[repr(C)]
pub struct accum_readings {
    _private: [u8; 0],
}

// BPF map declaration equivalent:
// struct {
//     __uint(type, BPF_MAP_TYPE_HASH);
//     __uint(key_size, sizeof(__u32));
//     __uint(value_size, sizeof(struct bperf_filter_value));
//     __uint(max_entries, MAX_ENTRIES);
//     __uint(map_flags, BPF_F_NO_PREALLOC);
// } filter SEC(".maps");
#[repr(C)]
pub struct filter {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut diff_readings: diff_readings;
    pub static mut accum_readings: accum_readings;
    pub static mut filter: filter;

    pub fn bpf_get_smp_processor_id() -> u32;
    pub fn bpf_get_current_pid_tgid() -> u64;
    pub fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i64;
    pub fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

// External definitions supplied by translated headers/dependencies.
pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct bpf_perf_event_value {
    pub counter: __u64,
    pub enabled: __u64,
    pub running: __u64,
}

#[repr(C)]
pub struct bperf_filter_value {
    pub accum_key: __u32,
    pub exited: __u32,
}

#[repr(C)]
pub struct task_struct {
    pub pid: __u32,
    pub tgid: __u32,
}

pub type bperf_filter_type = u32;

pub const BPERF_FILTER_GLOBAL: bperf_filter_type = 0;
pub const BPERF_FILTER_CPU: bperf_filter_type = 1;
pub const BPERF_FILTER_PID: bperf_filter_type = 2;
pub const BPERF_FILTER_TGID: bperf_filter_type = 3;
pub const BPF_NOEXIST: u64 = 1;

pub static mut r#type: bperf_filter_type = 0;
pub static mut enabled: core::ffi::c_int = 0;
pub static mut inherit: core::ffi::c_int = 0;

// SEC("fexit/XXX")
// int BPF_PROG(fexit_XXX)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fexit_XXX() -> core::ffi::c_int {
    let mut diff_val: *mut bpf_perf_event_value;
    let mut accum_val: *mut bpf_perf_event_value;
    let mut filter_key: __u32 = 0;
    let zero: __u32 = 0;
    let accum_key: __u32;
    let mut fval: *mut bperf_filter_value;

    if enabled == 0 {
        return 0;
    }

    match r#type {
        BPERF_FILTER_GLOBAL => {
            accum_key = zero;
        }
        BPERF_FILTER_CPU => {
            filter_key = bpf_get_smp_processor_id();
            fval = bpf_map_lookup_elem(
                core::ptr::addr_of_mut!(filter).cast(),
                core::ptr::addr_of!(filter_key).cast(),
            )
            .cast();
            if fval.is_null() {
                return 0;
            }

            accum_key = (*fval).accum_key;
            if (*fval).exited != 0 {
                bpf_map_delete_elem(
                    core::ptr::addr_of_mut!(filter).cast(),
                    core::ptr::addr_of!(filter_key).cast(),
                );
            }
        }
        BPERF_FILTER_PID => {
            filter_key = (bpf_get_current_pid_tgid() & 0xffffffff) as __u32;
            fval = bpf_map_lookup_elem(
                core::ptr::addr_of_mut!(filter).cast(),
                core::ptr::addr_of!(filter_key).cast(),
            )
            .cast();
            if fval.is_null() {
                return 0;
            }

            accum_key = (*fval).accum_key;
            if (*fval).exited != 0 {
                bpf_map_delete_elem(
                    core::ptr::addr_of_mut!(filter).cast(),
                    core::ptr::addr_of!(filter_key).cast(),
                );
            }
        }
        BPERF_FILTER_TGID => {
            /* Use pid as the filter_key to exclude new task counts
             * when inherit is disabled. Don't worry about the existing
             * children in TGID losing their counts, bpf_counter has
             * already added them to the filter map via perf_thread_map
             * before this bpf prog runs.
             */
            filter_key = if inherit != 0 {
                (bpf_get_current_pid_tgid() >> 32) as __u32
            } else {
                (bpf_get_current_pid_tgid() & 0xffffffff) as __u32
            };
            fval = bpf_map_lookup_elem(
                core::ptr::addr_of_mut!(filter).cast(),
                core::ptr::addr_of!(filter_key).cast(),
            )
            .cast();
            if fval.is_null() {
                return 0;
            }

            accum_key = (*fval).accum_key;
            if (*fval).exited != 0 {
                bpf_map_delete_elem(
                    core::ptr::addr_of_mut!(filter).cast(),
                    core::ptr::addr_of!(filter_key).cast(),
                );
            }
        }
        _ => {
            return 0;
        }
    }

    diff_val = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(diff_readings).cast(),
        core::ptr::addr_of!(zero).cast(),
    )
    .cast();
    if diff_val.is_null() {
        return 0;
    }

    accum_val = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(accum_readings).cast(),
        core::ptr::addr_of!(accum_key).cast(),
    )
    .cast();
    if accum_val.is_null() {
        return 0;
    }

    (*accum_val).counter = (*accum_val).counter.wrapping_add((*diff_val).counter);
    (*accum_val).enabled = (*accum_val).enabled.wrapping_add((*diff_val).enabled);
    (*accum_val).running = (*accum_val).running.wrapping_add((*diff_val).running);

    0
}

/* The program is only used for PID or TGID filter types. */
// SEC("tp_btf/task_newtask")
// int BPF_PROG(on_newtask, struct task_struct *task, __u64 clone_flags)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_newtask(
    task: *mut task_struct,
    _clone_flags: __u64,
) -> core::ffi::c_int {
    let parent_key: __u32;
    let child_key: __u32;
    let mut parent_fval: *mut bperf_filter_value;
    let mut child_fval: bperf_filter_value = bperf_filter_value {
        accum_key: 0,
        exited: 0,
    };

    if enabled == 0 {
        return 0;
    }

    match r#type {
        BPERF_FILTER_PID => {
            parent_key = (bpf_get_current_pid_tgid() & 0xffffffff) as __u32;
            child_key = (*task).pid;
        }
        BPERF_FILTER_TGID => {
            parent_key = (bpf_get_current_pid_tgid() >> 32) as __u32;
            child_key = (*task).tgid;
            if child_key == parent_key {
                return 0;
            }
        }
        _ => {
            return 0;
        }
    }

    /* Check if the current task is one of the target tasks to be counted */
    parent_fval = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(filter).cast(),
        core::ptr::addr_of!(parent_key).cast(),
    )
    .cast();
    if parent_fval.is_null() {
        return 0;
    }

    /* Start counting for the new task by adding it into filter map,
     * inherit the accum key of its parent task so that they can be
     * counted together.
     */
    child_fval.accum_key = (*parent_fval).accum_key;
    child_fval.exited = 0;
    bpf_map_update_elem(
        core::ptr::addr_of_mut!(filter).cast(),
        core::ptr::addr_of!(child_key).cast(),
        core::ptr::addr_of!(child_fval).cast(),
        BPF_NOEXIST,
    );

    0
}

/* The program is only used for PID or TGID filter types. */
// SEC("tp_btf/sched_process_exit")
// int BPF_PROG(on_exittask, struct task_struct *task)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn on_exittask(task: *mut task_struct) -> core::ffi::c_int {
    let pid: __u32;
    let mut fval: *mut bperf_filter_value;

    if enabled == 0 {
        return 0;
    }

    /* Stop counting for this task by removing it from filter map.
     * For TGID type, if the pid can be found in the map, it means that
     * this pid belongs to the leader task. After the task exits, the
     * tgid of its child tasks (if any) will be 1, so the pid can be
     * safely removed.
     */
    pid = (*task).pid;
    fval = bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(filter).cast(),
        core::ptr::addr_of!(pid).cast(),
    )
    .cast();
    if !fval.is_null() {
        (*fval).exited = 1;
    }

    0
}

// char LICENSE[] SEC("license") = "Dual BSD/GPL";
#[unsafe(no_mangle)]
pub static mut LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";
