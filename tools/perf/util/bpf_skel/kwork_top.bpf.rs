// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2022, Huawei

// Dependencies in the C source:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <bpf/bpf_core_read.h>

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type u8 = __u8;
type u32 = __u32;
type u64 = __u64;

/*
 * This should be in sync with "util/kwork.h"
 */
#[repr(u32)]
enum kwork_class_type {
    KWORK_CLASS_IRQ,
    KWORK_CLASS_SOFTIRQ,
    KWORK_CLASS_WORKQUEUE,
    KWORK_CLASS_SCHED,
    KWORK_CLASS_MAX,
}

const MAX_ENTRIES: u32 = 102400;
const MAX_NR_CPUS: u32 = 4096;
const PF_KTHREAD: u32 = 0x00200000;
const MAX_COMMAND_LEN: usize = 16;

const BPF_MAP_TYPE_TASK_STORAGE: u32 = 0;
const BPF_MAP_TYPE_PERCPU_HASH: u32 = 0;
const BPF_MAP_TYPE_HASH: u32 = 0;
const BPF_F_NO_PREALLOC: u32 = 0;
const BPF_ANY: u64 = 0;
const BPF_LOCAL_STORAGE_GET_F_CREATE: u64 = 0;

#[repr(C)]
struct task_struct {
    pid: __u32,
    tgid: __u32,
    flags: __u32,
    comm: [::core::ffi::c_char; MAX_COMMAND_LEN],
}

#[repr(C)]
struct time_data {
    timestamp: __u64,
}

#[repr(C)]
struct work_data {
    runtime: __u64,
}

#[repr(C)]
struct task_data {
    tgid: __u32,
    is_kthread: __u32,
    comm: [::core::ffi::c_char; MAX_COMMAND_LEN],
}

#[repr(C)]
struct work_key {
    type_: __u32,
    pid: __u32,
    task_p: __u64,
}

#[repr(C)]
struct task_key {
    pid: __u32,
    cpu: __u32,
}

#[repr(C)]
struct bpf_map_def {
    type_: u32,
    map_flags: u32,
    key_size: u32,
    value_size: u32,
    max_entries: u32,
}

#[link_section = ".maps"]
#[no_mangle]
static mut kwork_top_task_time: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_TASK_STORAGE,
    map_flags: BPF_F_NO_PREALLOC,
    key_size: ::core::mem::size_of::<i32>() as u32,
    value_size: ::core::mem::size_of::<time_data>() as u32,
    max_entries: 0,
};

#[link_section = ".maps"]
#[no_mangle]
static mut kwork_top_irq_time: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    map_flags: 0,
    key_size: ::core::mem::size_of::<work_key>() as u32,
    value_size: ::core::mem::size_of::<time_data>() as u32,
    max_entries: MAX_ENTRIES,
};

#[link_section = ".maps"]
#[no_mangle]
static mut kwork_top_tasks: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: 0,
    key_size: ::core::mem::size_of::<task_key>() as u32,
    value_size: ::core::mem::size_of::<task_data>() as u32,
    max_entries: MAX_ENTRIES,
};

#[link_section = ".maps"]
#[no_mangle]
static mut kwork_top_works: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_HASH,
    map_flags: 0,
    key_size: ::core::mem::size_of::<work_key>() as u32,
    value_size: ::core::mem::size_of::<work_data>() as u32,
    max_entries: MAX_ENTRIES,
};

#[link_section = ".maps"]
#[no_mangle]
static mut kwork_top_cpu_filter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    map_flags: 0,
    key_size: ::core::mem::size_of::<u32>() as u32,
    value_size: ::core::mem::size_of::<u8>() as u32,
    max_entries: MAX_NR_CPUS,
};

#[no_mangle]
static mut enabled: i32 = 0;

#[no_mangle]
static mut has_cpu_filter: i32 = 0;

#[no_mangle]
static mut from_timestamp: __u64 = 0;
#[no_mangle]
static mut to_timestamp: __u64 = 0;

extern "C" {
    fn bpf_map_lookup_elem(map: *mut bpf_map_def, key: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const ::core::ffi::c_void,
        value: *const ::core::ffi::c_void,
        flags: u64,
    ) -> i64;
    fn bpf_task_storage_get(
        map: *mut bpf_map_def,
        task: *mut task_struct,
        value: *mut ::core::ffi::c_void,
        flags: u64,
    ) -> *mut ::core::ffi::c_void;
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_ktime_get_ns() -> __u64;
    fn bpf_get_current_task() -> *mut ::core::ffi::c_void;
    fn bpf_core_read_str_into(dst: *mut ::core::ffi::c_char, src: *const ::core::ffi::c_char) -> i64;
}

#[inline(always)]
unsafe fn cpu_is_filtered(cpu: __u32) -> i32 {
    let mut cpu_val: *mut __u8;

    if has_cpu_filter != 0 {
        cpu_val = bpf_map_lookup_elem(
            &mut kwork_top_cpu_filter,
            &cpu as *const __u32 as *const ::core::ffi::c_void,
        ) as *mut __u8;
        if cpu_val.is_null() {
            return 1;
        }
    }

    0
}

#[inline(always)]
unsafe fn update_task_info(task: *mut task_struct, cpu: __u32) {
    let key = task_key {
        pid: (*task).pid,
        cpu,
    };

    if bpf_map_lookup_elem(
        &mut kwork_top_tasks,
        &key as *const task_key as *const ::core::ffi::c_void,
    )
    .is_null()
    {
        let mut data = task_data {
            tgid: (*task).tgid,
            is_kthread: if ((*task).flags & PF_KTHREAD) != 0 { 1 } else { 0 },
            comm: [0; MAX_COMMAND_LEN],
        };
        bpf_core_read_str_into(data.comm.as_mut_ptr(), (*task).comm.as_ptr());

        bpf_map_update_elem(
            &mut kwork_top_tasks,
            &key as *const task_key as *const ::core::ffi::c_void,
            &data as *const task_data as *const ::core::ffi::c_void,
            BPF_ANY,
        );
    }
}

#[inline(always)]
unsafe fn update_work(key: *mut work_key, delta: __u64) {
    let mut data: *mut work_data;

    data = bpf_map_lookup_elem(
        &mut kwork_top_works,
        key as *const work_key as *const ::core::ffi::c_void,
    ) as *mut work_data;
    if !data.is_null() {
        (*data).runtime = (*data).runtime.wrapping_add(delta);
    } else {
        let new_data = work_data { runtime: delta };

        bpf_map_update_elem(
            &mut kwork_top_works,
            key as *const work_key as *const ::core::ffi::c_void,
            &new_data as *const work_data as *const ::core::ffi::c_void,
            BPF_ANY,
        );
    }
}

unsafe fn on_sched_out(task: *mut task_struct, ts: __u64, cpu: __u32) {
    let delta: __u64;
    let mut pelem: *mut time_data;

    pelem = bpf_task_storage_get(&mut kwork_top_task_time, task, ::core::ptr::null_mut(), 0)
        as *mut time_data;
    if !pelem.is_null() {
        delta = ts.wrapping_sub((*pelem).timestamp);
    } else {
        delta = ts.wrapping_sub(from_timestamp);
    }

    let mut key = work_key {
        type_: kwork_class_type::KWORK_CLASS_SCHED as __u32,
        pid: (*task).pid,
        task_p: task as __u64,
    };

    update_work(&mut key, delta);
    update_task_info(task, cpu);
}

unsafe fn on_sched_in(task: *mut task_struct, ts: __u64) {
    let mut pelem: *mut time_data;

    pelem = bpf_task_storage_get(
        &mut kwork_top_task_time,
        task,
        ::core::ptr::null_mut(),
        BPF_LOCAL_STORAGE_GET_F_CREATE,
    ) as *mut time_data;
    if !pelem.is_null() {
        (*pelem).timestamp = ts;
    }
}

#[link_section = "tp_btf/sched_switch"]
#[no_mangle]
pub unsafe extern "C" fn on_switch(ctx: *mut u64) -> i32 {
    let mut prev: *mut task_struct;
    let mut next: *mut task_struct;

    prev = *ctx.add(1) as *mut task_struct;
    next = *ctx.add(2) as *mut task_struct;

    if enabled == 0 {
        return 0;
    }

    let cpu: __u32 = bpf_get_smp_processor_id();

    if cpu_is_filtered(cpu) != 0 {
        return 0;
    }

    let ts: __u64 = bpf_ktime_get_ns();

    on_sched_out(prev, ts, cpu);
    on_sched_in(next, ts);

    0
}

#[link_section = "tp_btf/irq_handler_entry"]
#[no_mangle]
pub unsafe extern "C" fn on_irq_handler_entry(cxt: *mut u64) -> i32 {
    let mut task: *mut task_struct;
    let _ = cxt;

    if enabled == 0 {
        return 0;
    }

    let cpu: __u32 = bpf_get_smp_processor_id();

    if cpu_is_filtered(cpu) != 0 {
        return 0;
    }

    let ts: __u64 = bpf_ktime_get_ns();

    task = bpf_get_current_task() as *mut task_struct;
    if task.is_null() {
        return 0;
    }

    let key = work_key {
        type_: kwork_class_type::KWORK_CLASS_IRQ as __u32,
        pid: (*task).pid,
        task_p: task as __u64,
    };

    let data = time_data { timestamp: ts };

    bpf_map_update_elem(
        &mut kwork_top_irq_time,
        &key as *const work_key as *const ::core::ffi::c_void,
        &data as *const time_data as *const ::core::ffi::c_void,
        BPF_ANY,
    );

    0
}

#[link_section = "tp_btf/irq_handler_exit"]
#[no_mangle]
pub unsafe extern "C" fn on_irq_handler_exit(cxt: *mut u64) -> i32 {
    let delta: __u64;
    let mut task: *mut task_struct;
    let mut pelem: *mut time_data;
    let _ = cxt;

    if enabled == 0 {
        return 0;
    }

    let cpu: __u32 = bpf_get_smp_processor_id();

    if cpu_is_filtered(cpu) != 0 {
        return 0;
    }

    let ts: __u64 = bpf_ktime_get_ns();

    task = bpf_get_current_task() as *mut task_struct;
    if task.is_null() {
        return 0;
    }

    let mut key = work_key {
        type_: kwork_class_type::KWORK_CLASS_IRQ as __u32,
        pid: (*task).pid,
        task_p: task as __u64,
    };

    pelem = bpf_map_lookup_elem(
        &mut kwork_top_irq_time,
        &key as *const work_key as *const ::core::ffi::c_void,
    ) as *mut time_data;
    if !pelem.is_null() && (*pelem).timestamp != 0 {
        delta = ts.wrapping_sub((*pelem).timestamp);
    } else {
        delta = ts.wrapping_sub(from_timestamp);
    }

    update_work(&mut key, delta);

    0
}

#[link_section = "tp_btf/softirq_entry"]
#[no_mangle]
pub unsafe extern "C" fn on_softirq_entry(cxt: *mut u64) -> i32 {
    let mut task: *mut task_struct;
    let _ = cxt;

    if enabled == 0 {
        return 0;
    }

    let cpu: __u32 = bpf_get_smp_processor_id();

    if cpu_is_filtered(cpu) != 0 {
        return 0;
    }

    let ts: __u64 = bpf_ktime_get_ns();

    task = bpf_get_current_task() as *mut task_struct;
    if task.is_null() {
        return 0;
    }

    let key = work_key {
        type_: kwork_class_type::KWORK_CLASS_SOFTIRQ as __u32,
        pid: (*task).pid,
        task_p: task as __u64,
    };

    let data = time_data { timestamp: ts };

    bpf_map_update_elem(
        &mut kwork_top_irq_time,
        &key as *const work_key as *const ::core::ffi::c_void,
        &data as *const time_data as *const ::core::ffi::c_void,
        BPF_ANY,
    );

    0
}

#[link_section = "tp_btf/softirq_exit"]
#[no_mangle]
pub unsafe extern "C" fn on_softirq_exit(cxt: *mut u64) -> i32 {
    let delta: __u64;
    let mut task: *mut task_struct;
    let mut pelem: *mut time_data;
    let _ = cxt;

    if enabled == 0 {
        return 0;
    }

    let cpu: __u32 = bpf_get_smp_processor_id();

    if cpu_is_filtered(cpu) != 0 {
        return 0;
    }

    let ts: __u64 = bpf_ktime_get_ns();

    task = bpf_get_current_task() as *mut task_struct;
    if task.is_null() {
        return 0;
    }

    let mut key = work_key {
        type_: kwork_class_type::KWORK_CLASS_SOFTIRQ as __u32,
        pid: (*task).pid,
        task_p: task as __u64,
    };

    pelem = bpf_map_lookup_elem(
        &mut kwork_top_irq_time,
        &key as *const work_key as *const ::core::ffi::c_void,
    ) as *mut time_data;
    if !pelem.is_null() {
        delta = ts.wrapping_sub((*pelem).timestamp);
    } else {
        delta = ts.wrapping_sub(from_timestamp);
    }

    update_work(&mut key, delta);

    0
}

#[link_section = "license"]
#[no_mangle]
pub static LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";
