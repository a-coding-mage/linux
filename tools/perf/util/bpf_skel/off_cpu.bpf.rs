// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2022 Google
//
// Rust translation of perf/util/bpf_skel/off_cpu.bpf.c.
// Original includes: "vmlinux.h", <bpf/bpf_helpers.h>,
// <bpf/bpf_tracing.h>, <bpf/bpf_core_read.h>.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

/* task->flags for off-cpu analysis */
const PF_KTHREAD: core::ffi::c_long = 0x00200000; /* I am a kernel thread */

/* task->state for off-cpu analysis */
const TASK_INTERRUPTIBLE: core::ffi::c_int = 0x0001;
const TASK_UNINTERRUPTIBLE: core::ffi::c_int = 0x0002;

/* create a new thread */
const CLONE_THREAD: u64 = 0x10000;

const MAX_STACKS: usize = 32;
const MAX_ENTRIES: u32 = 102400;

const MAX_CPUS: u32 = 4096;
const MAX_OFFCPU_LEN: usize = 37;

const BPF_MAP_TYPE_STACK_TRACE: u32 = 7;
const BPF_MAP_TYPE_PERF_EVENT_ARRAY: u32 = 4;
const BPF_MAP_TYPE_PERCPU_ARRAY: u32 = 6;
const BPF_MAP_TYPE_TASK_STORAGE: u32 = 29;
const BPF_MAP_TYPE_HASH: u32 = 1;
const BPF_F_NO_PREALLOC: u32 = 1;
const BPF_F_CURRENT_CPU: u64 = 0xffffffff;
const BPF_F_FAST_STACK_CMP: u64 = 1 << 9;
const BPF_F_USER_STACK: u64 = 1 << 8;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 1;
const BPF_ANY: u64 = 0;
const BPF_NOEXIST: u64 = 1;
const PERF_CONTEXT_USER: u64 = (-512i64) as u64;

// We have a 'struct stack' in vmlinux.h when building with GEN_VMLINUX_H=1
#[repr(C)]
pub struct __stack {
    pub array: [u64; MAX_STACKS],
}

#[repr(C)]
pub struct tstamp_data {
    pub stack_id: __u32,
    pub state: __u32,
    pub timestamp: __u64,
    pub stack: __stack,
}

#[repr(C)]
pub struct offcpu_key {
    pub pid: __u32,
    pub tgid: __u32,
    pub stack_id: __u32,
    pub state: __u32,
    pub cgroup_id: __u64,
}

#[repr(C)]
pub struct offcpu_data {
    pub array: [u64; MAX_OFFCPU_LEN],
}

#[repr(C)]
pub struct task_struct {
    pub flags: core::ffi::c_long,
    pub pid: __u32,
    pub tgid: __u32,
}

#[repr(C)]
pub struct cgroup {
    _private: [u8; 0],
}

/* new kernel task_struct definition */
#[repr(C)]
pub struct task_struct___new {
    pub __state: core::ffi::c_long,
}

/* old kernel task_struct definition */
#[repr(C)]
pub struct task_struct___old {
    pub state: core::ffi::c_long,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub map_flags: u32,
}

// SEC(".maps")
#[no_mangle]
pub static mut stacks: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_STACK_TRACE,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: (MAX_STACKS * core::mem::size_of::<__u64>()) as u32,
    max_entries: MAX_ENTRIES,
    map_flags: 0,
};

// SEC(".maps")
#[no_mangle]
pub static mut offcpu_output: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERF_EVENT_ARRAY,
    key_size: core::mem::size_of::<core::ffi::c_int>() as u32,
    value_size: core::mem::size_of::<core::ffi::c_int>() as u32,
    max_entries: MAX_CPUS,
    map_flags: 0,
};

// SEC(".maps")
#[no_mangle]
pub static mut offcpu_payload: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<offcpu_data>() as u32,
    max_entries: 1,
    map_flags: 0,
};

// SEC(".maps"), key type int, value type struct tstamp_data.
#[no_mangle]
pub static mut tstamp: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    key_size: core::mem::size_of::<core::ffi::c_int>() as u32,
    value_size: core::mem::size_of::<tstamp_data>() as u32,
    max_entries: 0,
    map_flags: BPF_F_NO_PREALLOC,
};

// SEC(".maps")
#[no_mangle]
pub static mut off_cpu: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<offcpu_key>() as u32,
    value_size: core::mem::size_of::<__u64>() as u32,
    max_entries: MAX_ENTRIES,
    map_flags: 0,
};

// SEC(".maps")
#[no_mangle]
pub static mut cpu_filter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u8>() as u32,
    max_entries: 1,
    map_flags: 0,
};

// SEC(".maps")
#[no_mangle]
pub static mut task_filter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u8>() as u32,
    max_entries: 1,
    map_flags: 0,
};

// SEC(".maps")
#[no_mangle]
pub static mut cgroup_filter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u64>() as u32,
    value_size: core::mem::size_of::<__u8>() as u32,
    max_entries: 1,
    map_flags: 0,
};

#[no_mangle]
pub static mut enabled: core::ffi::c_int = 0;

#[no_mangle]
pub static has_cpu: core::ffi::c_int = 0;
#[no_mangle]
pub static has_task: core::ffi::c_int = 0;
#[no_mangle]
pub static has_cgroup: core::ffi::c_int = 0;
#[no_mangle]
pub static uses_tgid: core::ffi::c_int = 0;

#[no_mangle]
pub static has_prev_state: bool = false;
#[no_mangle]
pub static needs_cgroup: bool = false;
#[no_mangle]
pub static uses_cgroup_v1: bool = false;

#[no_mangle]
pub static mut perf_subsys_id: core::ffi::c_int = -1;

#[no_mangle]
pub static mut offcpu_thresh_ns: __u64 = 0;

extern "C" {
    fn bpf_core_field_exists<T>(field: T) -> bool;
    fn BPF_CORE_READ<T, R>(ptr: *const T, field: *const core::ffi::c_void) -> R;
    fn bpf_core_enum_value(enum_type: core::ffi::c_int, value: core::ffi::c_int) -> core::ffi::c_int;
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_map_lookup_elem(map: *mut bpf_map_def, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> core::ffi::c_int;
    fn bpf_perf_event_output(
        ctx: *mut core::ffi::c_void,
        map: *mut bpf_map_def,
        flags: u64,
        data: *mut core::ffi::c_void,
        size: u64,
    ) -> core::ffi::c_int;
    fn bpf_ktime_get_ns() -> __u64;
    fn bpf_get_stackid(ctx: *mut u64, map: *mut bpf_map_def, flags: u64) -> __u32;
    fn bpf_task_storage_get(
        map: *mut bpf_map_def,
        task: *mut task_struct,
        value: *mut core::ffi::c_void,
        flags: u64,
    ) -> *mut tstamp_data;
    fn bpf_get_stack(
        ctx: *mut u64,
        buf: *mut core::ffi::c_void,
        size: u64,
        flags: u64,
    ) -> core::ffi::c_int;
    fn bpf_get_current_task() -> *mut core::ffi::c_void;
}

/*
 * Old kernel used to call it task_struct->state and now it's '__state'.
 * Use BPF CO-RE "ignored suffix rule" to deal with it like below:
 *
 * https://nakryiko.com/posts/bpf-core-reference-guide/#handling-incompatible-field-and-type-changes
 */
unsafe fn get_task_state(t: *mut task_struct) -> core::ffi::c_int {
    /* recast pointer to capture new type for compiler */
    let t_new: *mut task_struct___new = t as *mut core::ffi::c_void as *mut task_struct___new;

    if bpf_core_field_exists((*t_new).__state) {
        BPF_CORE_READ(t_new, core::ptr::addr_of!((*t_new).__state) as *const core::ffi::c_void)
    } else {
        /* recast pointer to capture old type for compiler */
        let t_old: *mut task_struct___old = t as *mut core::ffi::c_void as *mut task_struct___old;

        BPF_CORE_READ(t_old, core::ptr::addr_of!((*t_old).state) as *const core::ffi::c_void)
    }
}

unsafe fn get_cgroup_id(t: *mut task_struct) -> __u64 {
    let cgrp: *mut cgroup;

    if !uses_cgroup_v1 {
        return BPF_CORE_READ(t, core::ptr::null());
    }

    if perf_subsys_id == -1 {
        /*
         * C condition:
         * #if __has_builtin(__builtin_preserve_enum_value)
         *     perf_subsys_id = bpf_core_enum_value(enum cgroup_subsys_id,
         *                                          perf_event_cgrp_id);
         * #else
         *     perf_subsys_id = perf_event_cgrp_id;
         * #endif
         */
        perf_subsys_id = bpf_core_enum_value(0, 0);
    }

    cgrp = BPF_CORE_READ(t, core::ptr::null());
    BPF_CORE_READ(cgrp, core::ptr::null())
}

unsafe fn can_record(t: *mut task_struct, state: core::ffi::c_int) -> core::ffi::c_int {
    /* kernel threads don't have user stack */
    if ((*t).flags & PF_KTHREAD) != 0 {
        return 0;
    }

    if state != TASK_INTERRUPTIBLE && state != TASK_UNINTERRUPTIBLE {
        return 0;
    }

    if has_cpu != 0 {
        let cpu: __u32 = bpf_get_smp_processor_id();
        let ok: *mut __u8;

        ok = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(cpu_filter),
            core::ptr::addr_of!(cpu) as *const core::ffi::c_void,
        ) as *mut __u8;
        if ok.is_null() {
            return 0;
        }
    }

    if has_task != 0 {
        let ok: *mut __u8;
        let pid: __u32;

        if uses_tgid != 0 {
            pid = (*t).tgid;
        } else {
            pid = (*t).pid;
        }

        ok = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(task_filter),
            core::ptr::addr_of!(pid) as *const core::ffi::c_void,
        ) as *mut __u8;
        if ok.is_null() {
            return 0;
        }
    }

    if has_cgroup != 0 {
        let ok: *mut __u8;
        let cgrp_id: __u64 = get_cgroup_id(t);

        ok = bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(cgroup_filter),
            core::ptr::addr_of!(cgrp_id) as *const core::ffi::c_void,
        ) as *mut __u8;
        if ok.is_null() {
            return 0;
        }
    }

    1
}

unsafe fn copy_stack(from: *mut __stack, to: *mut offcpu_data, n: core::ffi::c_int) -> core::ffi::c_int {
    let mut len: core::ffi::c_int = 0;
    let mut i: core::ffi::c_int = 0;

    while i < MAX_STACKS as core::ffi::c_int && (*from).array[i as usize] != 0 {
        (*to).array[(n + 2 + i) as usize] = (*from).array[i as usize];
        i += 1;
        len += 1;
    }

    len
}

/**
 * off_cpu_dump - dump off-cpu samples to ring buffer
 * @data: payload for dumping off-cpu samples
 * @key: off-cpu data
 * @stack: stack trace of the task before being scheduled out
 *
 * If the threshold of off-cpu time is reached, acquire tid, period, callchain, and cgroup id
 * information of the task, and dump it as a raw sample to perf ring buffer
 */
unsafe fn off_cpu_dump(
    ctx: *mut core::ffi::c_void,
    data: *mut offcpu_data,
    key: *mut offcpu_key,
    stack: *mut __stack,
    delta: __u64,
) -> core::ffi::c_int {
    let mut n: core::ffi::c_int = 0;
    let mut len: core::ffi::c_int = 0;

    (*data).array[n as usize] = ((*key).tgid as u64) << 32 | (*key).pid as u64;
    n += 1;
    (*data).array[n as usize] = delta;
    n += 1;

    /* data->array[n] is callchain->nr (updated later) */
    (*data).array[(n + 1) as usize] = PERF_CONTEXT_USER;
    (*data).array[(n + 2) as usize] = 0;
    len = copy_stack(stack, data, n);

    /* update length of callchain */
    (*data).array[n as usize] = (len + 1) as u64;
    n += len + 2;

    (*data).array[n as usize] = (*key).cgroup_id;
    n += 1;

    bpf_perf_event_output(
        ctx,
        core::ptr::addr_of_mut!(offcpu_output),
        BPF_F_CURRENT_CPU,
        data as *mut core::ffi::c_void,
        (n as usize * core::mem::size_of::<u64>()) as u64,
    )
}

unsafe fn off_cpu_stat(
    ctx: *mut u64,
    prev: *mut task_struct,
    next: *mut task_struct,
    state: core::ffi::c_int,
) -> core::ffi::c_int {
    let ts: __u64;
    let stack_id: __u32;
    let mut pelem: *mut tstamp_data;

    ts = bpf_ktime_get_ns();

    if can_record(prev, state) == 0 {
        pelem = bpf_task_storage_get(core::ptr::addr_of_mut!(tstamp), next, core::ptr::null_mut(), 0);
    } else {
        stack_id = bpf_get_stackid(
            ctx,
            core::ptr::addr_of_mut!(stacks),
            BPF_F_FAST_STACK_CMP | BPF_F_USER_STACK,
        );

        pelem = bpf_task_storage_get(
            core::ptr::addr_of_mut!(tstamp),
            prev,
            core::ptr::null_mut(),
            BPF_LOCAL_STORAGE_GET_F_CREATE,
        );
        if !pelem.is_null() {
            (*pelem).timestamp = ts;
            (*pelem).state = state as __u32;
            (*pelem).stack_id = stack_id;

            /*
             * If stacks are successfully collected by bpf_get_stackid(), collect them once more
             * in task_storage for direct off-cpu sample dumping
             */
            if stack_id > 0
                && bpf_get_stack(
                    ctx,
                    core::ptr::addr_of_mut!((*pelem).stack) as *mut core::ffi::c_void,
                    (MAX_STACKS * core::mem::size_of::<u64>()) as u64,
                    BPF_F_USER_STACK,
                ) != 0
            {
                /*
                 * This empty if block is used to avoid 'result unused warning' from bpf_get_stack().
                 * If the collection fails, continue with the logic for the next task.
                 */
            }
        }

        pelem = bpf_task_storage_get(core::ptr::addr_of_mut!(tstamp), next, core::ptr::null_mut(), 0);
    }

    if !pelem.is_null() && (*pelem).timestamp != 0 {
        let mut key: offcpu_key = offcpu_key {
            pid: (*next).pid,
            tgid: (*next).tgid,
            stack_id: (*pelem).stack_id,
            state: (*pelem).state,
            cgroup_id: if needs_cgroup { get_cgroup_id(next) } else { 0 },
        };
        let delta: __u64 = ts.wrapping_sub((*pelem).timestamp);
        let total: *mut __u64;

        if delta >= offcpu_thresh_ns {
            let zero: core::ffi::c_int = 0;
            let data: *mut offcpu_data = bpf_map_lookup_elem(
                core::ptr::addr_of_mut!(offcpu_payload),
                core::ptr::addr_of!(zero) as *const core::ffi::c_void,
            ) as *mut offcpu_data;

            if !data.is_null() {
                off_cpu_dump(
                    ctx as *mut core::ffi::c_void,
                    data,
                    core::ptr::addr_of_mut!(key),
                    core::ptr::addr_of_mut!((*pelem).stack),
                    delta,
                );
            }
        } else {
            total = bpf_map_lookup_elem(
                core::ptr::addr_of_mut!(off_cpu),
                core::ptr::addr_of!(key) as *const core::ffi::c_void,
            ) as *mut __u64;
            if !total.is_null() {
                *total = (*total).wrapping_add(delta);
            } else {
                bpf_map_update_elem(
                    core::ptr::addr_of_mut!(off_cpu),
                    core::ptr::addr_of!(key) as *const core::ffi::c_void,
                    core::ptr::addr_of!(delta) as *const core::ffi::c_void,
                    BPF_ANY,
                );
            }
        }

        /* prevent to reuse the timestamp later */
        (*pelem).timestamp = 0;
    }

    0
}

// SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn on_newtask(ctx: *mut u64) -> core::ffi::c_int {
    let mut task: *mut task_struct;
    let clone_flags: u64;
    let mut pid: u32;
    let val: u8 = 1;

    if uses_tgid == 0 {
        return 0;
    }

    task = bpf_get_current_task() as *mut task_struct;

    pid = BPF_CORE_READ(task, core::ptr::addr_of!((*task).tgid) as *const core::ffi::c_void);
    if bpf_map_lookup_elem(
        core::ptr::addr_of_mut!(task_filter),
        core::ptr::addr_of!(pid) as *const core::ffi::c_void,
    )
    .is_null()
    {
        return 0;
    }

    task = *ctx.add(0) as *mut task_struct;
    clone_flags = *ctx.add(1);

    pid = (*task).tgid;
    if (clone_flags & CLONE_THREAD) == 0 {
        bpf_map_update_elem(
            core::ptr::addr_of_mut!(task_filter),
            core::ptr::addr_of!(pid) as *const core::ffi::c_void,
            core::ptr::addr_of!(val) as *const core::ffi::c_void,
            BPF_NOEXIST,
        );
    }

    0
}

// SEC("tp_btf/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn on_switch(ctx: *mut u64) -> core::ffi::c_int {
    let prev: *mut task_struct;
    let next: *mut task_struct;
    let prev_state: core::ffi::c_int;

    if enabled == 0 {
        return 0;
    }

    prev = *ctx.add(1) as *mut task_struct;
    next = *ctx.add(2) as *mut task_struct;

    if has_prev_state {
        prev_state = *ctx.add(3) as core::ffi::c_int;
    } else {
        prev_state = get_task_state(prev);
    }

    off_cpu_stat(ctx, prev, next, prev_state & 0xff)
}

// SEC("license")
#[no_mangle]
pub static mut LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
