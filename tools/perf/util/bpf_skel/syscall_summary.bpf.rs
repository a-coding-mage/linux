// SPDX-License-Identifier: GPL-2.0
/*
 * Trace raw_syscalls tracepoints to collect system call statistics.
 */

// C dependencies translated as external Rust dependencies:
// "vmlinux.h", "syscall_summary.h", <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, and <bpf/bpf_core_read.h>.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

type __u64 = u64;
type u64 = u64;
type s64 = i64;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

#[repr(C)]
pub struct syscall_key {
    pub cpu_or_tid: i32,
    pub cgroup: u64,
    pub nr: i32,
}

#[repr(C)]
pub struct syscall_stats {
    pub count: u64,
    pub error: u64,
    pub total_time: s64,
    pub squared_sum: s64,
    pub max_time: s64,
    pub min_time: s64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum syscall_aggr_mode {
    SYSCALL_AGGR_THREAD,
    SYSCALL_AGGR_CGROUP,
}

#[repr(C)]
pub enum cgroup_subsys_id {
    perf_event_cgrp_id,
}

/* This is to calculate a delta between sys-enter and sys-exit for each thread */
#[repr(C)]
pub struct syscall_trace {
    pub nr: i32, /* syscall number is only available at sys-enter */
    pub unused: i32,
    pub timestamp: u64,
}

pub const MAX_ENTRIES: u32 = 128 * 1024;

#[repr(C)]
pub struct syscall_trace_map {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, int); /* tid */
    // __type(value, struct syscall_trace);
    // __uint(max_entries, MAX_ENTRIES);
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut syscall_trace_map: syscall_trace_map = syscall_trace_map { _private: [] };

#[repr(C)]
pub struct syscall_stats_map {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, struct syscall_key);
    // __type(value, struct syscall_stats);
    // __uint(max_entries, MAX_ENTRIES);
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut syscall_stats_map: syscall_stats_map = syscall_stats_map { _private: [] };

#[no_mangle]
pub static mut enabled: i32 = 0; /* controlled from userspace */

#[no_mangle]
pub static mut aggr_mode: syscall_aggr_mode = syscall_aggr_mode::SYSCALL_AGGR_THREAD;
#[no_mangle]
pub static mut use_cgroup_v2: i32 = 0;

#[no_mangle]
pub static mut perf_subsys_id: i32 = -1;

pub const BPF_ANY: u64 = 0;
pub const BPF_NOEXIST: u64 = 1;

extern "C" {
    fn bpf_get_current_cgroup_id() -> __u64;
    fn bpf_get_current_task_btf() -> *mut task_struct;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
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
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_core_enum_value(enum_type: cgroup_subsys_id, value: cgroup_subsys_id) -> i32;
    fn BPF_CORE_READ_cgroup_id(cgrp: *mut cgroup) -> __u64;
    fn BPF_CORE_READ_task_perf_cgroup(
        task: *mut task_struct,
        perf_subsys_id: i32,
    ) -> *mut cgroup;
}

#[inline]
unsafe fn get_current_cgroup_id() -> __u64 {
    let task: *mut task_struct;
    let cgrp: *mut cgroup;

    if use_cgroup_v2 != 0 {
        return bpf_get_current_cgroup_id();
    }

    task = bpf_get_current_task_btf();

    if perf_subsys_id == -1 {
        // C used __has_builtin(__builtin_preserve_enum_value) to prefer
        // bpf_core_enum_value(enum cgroup_subsys_id, perf_event_cgrp_id).
        perf_subsys_id = bpf_core_enum_value(
            cgroup_subsys_id::perf_event_cgrp_id,
            cgroup_subsys_id::perf_event_cgrp_id,
        );
    }

    cgrp = BPF_CORE_READ_task_perf_cgroup(task, perf_subsys_id);
    BPF_CORE_READ_cgroup_id(cgrp)
}

unsafe fn update_stats(cpu_or_tid: i32, cgroup_id: u64, nr: i32, duration: s64, ret: isize) {
    let key = syscall_key {
        cpu_or_tid,
        cgroup: cgroup_id,
        nr,
    };
    let mut stats: *mut syscall_stats;

    stats = bpf_map_lookup_elem(
        &mut syscall_stats_map as *mut _ as *mut core::ffi::c_void,
        &key as *const _ as *const core::ffi::c_void,
    ) as *mut syscall_stats;
    if stats.is_null() {
        let zero: syscall_stats = core::mem::zeroed();

        bpf_map_update_elem(
            &mut syscall_stats_map as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
            &zero as *const _ as *const core::ffi::c_void,
            BPF_NOEXIST,
        );
        stats = bpf_map_lookup_elem(
            &mut syscall_stats_map as *mut _ as *mut core::ffi::c_void,
            &key as *const _ as *const core::ffi::c_void,
        ) as *mut syscall_stats;
        if stats.is_null() {
            return;
        }
    }

    (*stats).count = (*stats).count.wrapping_add(1);
    if ret < 0 {
        (*stats).error = (*stats).error.wrapping_add(1);
    }

    if duration > 0 {
        (*stats).total_time = (*stats).total_time.wrapping_add(duration);
        (*stats).squared_sum = (*stats)
            .squared_sum
            .wrapping_add(duration.wrapping_mul(duration));
        if (*stats).max_time < duration {
            (*stats).max_time = duration;
        }
        if (*stats).min_time > duration || (*stats).min_time == 0 {
            (*stats).min_time = duration;
        }
    }

    return;
}

#[link_section = "tp_btf/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn sys_enter(ctx: *mut u64) -> i32 {
    let tid: i32;
    let mut st: syscall_trace = core::mem::zeroed();

    if enabled == 0 {
        return 0;
    }

    st.nr = *ctx.add(1) as i32; /* syscall number */
    st.unused = 0;
    st.timestamp = bpf_ktime_get_ns();

    tid = bpf_get_current_pid_tgid() as i32;
    bpf_map_update_elem(
        &mut syscall_trace_map as *mut _ as *mut core::ffi::c_void,
        &tid as *const _ as *const core::ffi::c_void,
        &st as *const _ as *const core::ffi::c_void,
        BPF_ANY,
    );

    0
}

unsafe fn do_exit(ret: isize) -> i32 {
    let tid: i32;
    let mut key: i32 = 0;
    let mut cgroup: u64 = 0;
    let st: *mut syscall_trace;
    let delta: s64;

    if enabled == 0 {
        return 0;
    }

    tid = bpf_get_current_pid_tgid() as i32;
    st = bpf_map_lookup_elem(
        &mut syscall_trace_map as *mut _ as *mut core::ffi::c_void,
        &tid as *const _ as *const core::ffi::c_void,
    ) as *mut syscall_trace;
    if st.is_null() {
        return 0;
    }

    if aggr_mode == syscall_aggr_mode::SYSCALL_AGGR_THREAD {
        key = tid;
    } else if aggr_mode == syscall_aggr_mode::SYSCALL_AGGR_CGROUP {
        cgroup = get_current_cgroup_id();
    } else {
        key = bpf_get_smp_processor_id() as i32;
    }

    delta = bpf_ktime_get_ns().wrapping_sub((*st).timestamp) as s64;
    update_stats(key, cgroup, (*st).nr, delta, ret);

    bpf_map_delete_elem(
        &mut syscall_trace_map as *mut _ as *mut core::ffi::c_void,
        &tid as *const _ as *const core::ffi::c_void,
    );
    0
}

#[link_section = "tp_btf/sys_exit"]
#[no_mangle]
pub unsafe extern "C" fn sys_exit(ctx: *mut u64) -> i32 {
    let ret: isize = *ctx.add(1) as isize; /* return value of the syscall */

    do_exit(ret)
}

#[link_section = "tp_btf/sched_process_exit"]
#[no_mangle]
pub unsafe extern "C" fn process_exit(_ctx: *mut u64) -> i32 {
    do_exit(0)
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
