// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2022, Huawei

// Translated from C eBPF source. Original includes:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

pub type __u8 = u8;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s64 = i64;

pub const KWORK_COUNT: usize = 100;
pub const MAX_KWORKNAME: usize = 128;

/*
 * This should be in sync with "util/kwork.h"
 */
pub const KWORK_CLASS_IRQ: kwork_class_type = 0;
pub const KWORK_CLASS_SOFTIRQ: kwork_class_type = 1;
pub const KWORK_CLASS_WORKQUEUE: kwork_class_type = 2;
pub const KWORK_CLASS_MAX: kwork_class_type = 3;
pub type kwork_class_type = u32;

#[repr(C)]
pub struct work_key {
    pub type_: __u32,
    pub cpu: __u32,
    pub id: __u64,
}

#[repr(C)]
pub struct report_data {
    pub nr: __u64,
    pub total_time: __u64,
    pub max_time: __u64,
    pub max_time_start: __u64,
    pub max_time_end: __u64,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

extern "C" {
    static BPF_MAP_TYPE_HASH: __u32;
    static BPF_MAP_TYPE_ARRAY: __u32;
    static BPF_NOEXIST: __u64;
    static BPF_ANY: __u64;
    static NR_SOFTIRQS: __u32;

    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_ktime_get_ns() -> __u64;
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i32;
    fn bpf_map_delete_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> i32;
    fn bpf_probe_read_kernel_str(
        dst: *mut core::ffi::c_void,
        size: __u32,
        unsafe_ptr: *const core::ffi::c_void,
    ) -> i32;
    fn bpf_snprintf(
        str_: *mut core::ffi::c_char,
        str_size: __u32,
        fmt: *const core::ffi::c_char,
        data: *const core::ffi::c_void,
        data_len: __u32,
    ) -> i32;
}

#[repr(C)]
pub struct trace_event_raw_irq_handler_entry {
    pub irq: __u32,
    pub __data_loc_name: __u32,
}

#[repr(C)]
pub struct trace_event_raw_irq_handler_exit {
    pub irq: __u32,
}

#[repr(C)]
pub struct trace_event_raw_softirq {
    pub vec: __u32,
}

#[repr(C)]
pub struct trace_event_raw_workqueue_execute_start {
    pub work: *mut core::ffi::c_void,
    pub function: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct trace_event_raw_workqueue_execute_end {
    pub work: *mut core::ffi::c_void,
    pub function: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct trace_event_raw_workqueue_activate_work {
    pub work: *mut core::ffi::c_void,
}

// SEC(".maps")
#[no_mangle]
pub static mut perf_kwork_names: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_HASH
    key_size: core::mem::size_of::<work_key>() as __u32,
    value_size: MAX_KWORKNAME as __u32,
    max_entries: KWORK_COUNT as __u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut perf_kwork_time: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_HASH
    key_size: core::mem::size_of::<work_key>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
    max_entries: KWORK_COUNT as __u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut perf_kwork_report: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_HASH
    key_size: core::mem::size_of::<work_key>() as __u32,
    value_size: core::mem::size_of::<report_data>() as __u32,
    max_entries: KWORK_COUNT as __u32,
};

// SEC(".maps")
#[no_mangle]
pub static mut perf_kwork_cpu_filter: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_HASH
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u8>() as __u32,
    max_entries: 1,
};

// SEC(".maps")
#[no_mangle]
pub static mut perf_kwork_name_filter: bpf_map_def = bpf_map_def {
    type_: 0, // BPF_MAP_TYPE_ARRAY
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: MAX_KWORKNAME as __u32,
    max_entries: 1,
};

#[no_mangle]
pub static mut enabled: i32 = 0;

#[no_mangle]
pub static mut has_cpu_filter: i32 = 0;
#[no_mangle]
pub static mut has_name_filter: i32 = 0;

#[inline(always)]
unsafe fn local_strncmp(
    s1: *const core::ffi::c_char,
    sz: u32,
    s2: *const core::ffi::c_char,
) -> i32 {
    let mut ret: i32 = 0;
    let mut i: u32 = 0;

    while i < sz {
        ret = (*s1.add(i as usize) as u8 as i32) - (*s2.add(i as usize) as u8 as i32);
        if ret != 0 || *s1.add(i as usize) == 0 {
            break;
        }
        i += 1;
    }

    ret
}

#[inline(always)]
unsafe fn trace_event_match(_key: *mut work_key, name: *mut core::ffi::c_char) -> i32 {
    let mut cpu_val: *mut __u8;
    let mut name_val: *mut core::ffi::c_char;
    let zero: __u32 = 0;
    let cpu: __u32 = bpf_get_smp_processor_id();

    if enabled == 0 {
        return 0;
    }

    if has_cpu_filter != 0 {
        cpu_val = bpf_map_lookup_elem(
            &mut perf_kwork_cpu_filter as *mut _ as *mut core::ffi::c_void,
            &cpu as *const _ as *const core::ffi::c_void,
        ) as *mut __u8;
        if cpu_val.is_null() {
            return 0;
        }
    }

    if has_name_filter != 0 && !name.is_null() {
        name_val = bpf_map_lookup_elem(
            &mut perf_kwork_name_filter as *mut _ as *mut core::ffi::c_void,
            &zero as *const _ as *const core::ffi::c_void,
        ) as *mut core::ffi::c_char;
        if !name_val.is_null() && local_strncmp(name_val, MAX_KWORKNAME as u32, name) != 0 {
            return 0;
        }
    }

    1
}

#[inline(always)]
unsafe fn do_update_time(
    map: *mut core::ffi::c_void,
    key: *mut work_key,
    time_start: __u64,
    time_end: __u64,
) {
    let mut zero: report_data = core::mem::zeroed();
    let mut data: *mut report_data;
    let delta: __s64 = time_end.wrapping_sub(time_start) as __s64;

    if delta < 0 {
        return;
    }

    data = bpf_map_lookup_elem(map, key as *const core::ffi::c_void) as *mut report_data;
    if data.is_null() {
        core::ptr::write_bytes(
            &mut zero as *mut _ as *mut u8,
            0,
            core::mem::size_of::<report_data>(),
        );
        bpf_map_update_elem(
            map,
            key as *const core::ffi::c_void,
            &zero as *const _ as *const core::ffi::c_void,
            BPF_NOEXIST,
        );
        data = bpf_map_lookup_elem(map, key as *const core::ffi::c_void) as *mut report_data;
        if data.is_null() {
            return;
        }
    }

    if (delta as __u64 > (*data).max_time) || ((*data).max_time == 0) {
        (*data).max_time = delta as __u64;
        (*data).max_time_start = time_start;
        (*data).max_time_end = time_end;
    }

    (*data).total_time = (*data).total_time.wrapping_add(delta as __u64);
    (*data).nr = (*data).nr.wrapping_add(1);
}

#[inline(always)]
unsafe fn do_update_timestart(map: *mut core::ffi::c_void, key: *mut work_key) {
    let ts: __u64 = bpf_ktime_get_ns();

    bpf_map_update_elem(
        map,
        key as *const core::ffi::c_void,
        &ts as *const _ as *const core::ffi::c_void,
        BPF_ANY,
    );
}

#[inline(always)]
unsafe fn do_update_timeend(
    report_map: *mut core::ffi::c_void,
    time_map: *mut core::ffi::c_void,
    key: *mut work_key,
) {
    let time: *mut __u64 =
        bpf_map_lookup_elem(time_map, key as *const core::ffi::c_void) as *mut __u64;

    if !time.is_null() {
        bpf_map_delete_elem(time_map, key as *const core::ffi::c_void);
        do_update_time(report_map, key, *time, bpf_ktime_get_ns());
    }
}

#[inline(always)]
unsafe fn do_update_name(
    map: *mut core::ffi::c_void,
    key: *mut work_key,
    name: *mut core::ffi::c_char,
) {
    if bpf_map_lookup_elem(map, key as *const core::ffi::c_void).is_null() {
        bpf_map_update_elem(
            map,
            key as *const core::ffi::c_void,
            name as *const core::ffi::c_void,
            BPF_ANY,
        );
    }
}

#[inline(always)]
unsafe fn update_timestart(map: *mut core::ffi::c_void, key: *mut work_key) -> i32 {
    if trace_event_match(key, core::ptr::null_mut()) == 0 {
        return 0;
    }

    do_update_timestart(map, key);
    0
}

#[inline(always)]
unsafe fn update_timestart_and_name(
    time_map: *mut core::ffi::c_void,
    names_map: *mut core::ffi::c_void,
    key: *mut work_key,
    name: *mut core::ffi::c_char,
) -> i32 {
    if trace_event_match(key, name) == 0 {
        return 0;
    }

    do_update_timestart(time_map, key);
    do_update_name(names_map, key, name);

    0
}

#[inline(always)]
unsafe fn update_timeend(
    report_map: *mut core::ffi::c_void,
    time_map: *mut core::ffi::c_void,
    key: *mut work_key,
) -> i32 {
    if trace_event_match(key, core::ptr::null_mut()) == 0 {
        return 0;
    }

    do_update_timeend(report_map, time_map, key);

    0
}

#[inline(always)]
unsafe fn update_timeend_and_name(
    report_map: *mut core::ffi::c_void,
    time_map: *mut core::ffi::c_void,
    names_map: *mut core::ffi::c_void,
    key: *mut work_key,
    name: *mut core::ffi::c_char,
) -> i32 {
    if trace_event_match(key, name) == 0 {
        return 0;
    }

    do_update_timeend(report_map, time_map, key);
    do_update_name(names_map, key, name);

    0
}

// SEC("tracepoint/irq/irq_handler_entry")
#[no_mangle]
pub unsafe extern "C" fn report_irq_handler_entry(
    ctx: *mut trace_event_raw_irq_handler_entry,
) -> i32 {
    let mut name: [core::ffi::c_char; MAX_KWORKNAME] = [0; MAX_KWORKNAME];
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_IRQ,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).irq as __u64,
    };
    let name_addr: *mut core::ffi::c_void =
        (ctx as *mut core::ffi::c_void as *mut u8).add(((*ctx).__data_loc_name & 0xffff) as usize)
            as *mut core::ffi::c_void;

    bpf_probe_read_kernel_str(
        name.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&name) as __u32,
        name_addr as *const core::ffi::c_void,
    );

    update_timestart_and_name(
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_names as *mut _ as *mut core::ffi::c_void,
        &mut key,
        name.as_mut_ptr(),
    )
}

// SEC("tracepoint/irq/irq_handler_exit")
#[no_mangle]
pub unsafe extern "C" fn report_irq_handler_exit(
    ctx: *mut trace_event_raw_irq_handler_exit,
) -> i32 {
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_IRQ,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).irq as __u64,
    };

    update_timeend(
        &mut perf_kwork_report as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut key,
    )
}

static mut softirq_name_list: [[core::ffi::c_char; MAX_KWORKNAME]; 10] = [
    c_array_128(b"HI\0"),
    c_array_128(b"TIMER\0"),
    c_array_128(b"NET_TX\0"),
    c_array_128(b"NET_RX\0"),
    c_array_128(b"BLOCK\0"),
    c_array_128(b"IRQ_POLL\0"),
    c_array_128(b"TASKLET\0"),
    c_array_128(b"SCHED\0"),
    c_array_128(b"HRTIMER\0"),
    c_array_128(b"RCU\0"),
];

const fn c_array_128(src: &[u8]) -> [core::ffi::c_char; MAX_KWORKNAME] {
    let mut out = [0 as core::ffi::c_char; MAX_KWORKNAME];
    let mut i = 0;
    while i < src.len() {
        out[i] = src[i] as core::ffi::c_char;
        i += 1;
    }
    out
}

// SEC("tracepoint/irq/softirq_entry")
#[no_mangle]
pub unsafe extern "C" fn report_softirq_entry(ctx: *mut trace_event_raw_softirq) -> i32 {
    let vec: u32 = (*ctx).vec;
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_SOFTIRQ,
        cpu: bpf_get_smp_processor_id(),
        id: vec as __u64,
    };

    if vec < NR_SOFTIRQS {
        return update_timestart_and_name(
            &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
            &mut perf_kwork_names as *mut _ as *mut core::ffi::c_void,
            &mut key,
            softirq_name_list[vec as usize].as_mut_ptr(),
        );
    }

    0
}

// SEC("tracepoint/irq/softirq_exit")
#[no_mangle]
pub unsafe extern "C" fn report_softirq_exit(ctx: *mut trace_event_raw_softirq) -> i32 {
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_SOFTIRQ,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).vec as __u64,
    };

    update_timeend(
        &mut perf_kwork_report as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut key,
    )
}

// SEC("tracepoint/irq/softirq_raise")
#[no_mangle]
pub unsafe extern "C" fn latency_softirq_raise(ctx: *mut trace_event_raw_softirq) -> i32 {
    let vec: u32 = (*ctx).vec;
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_SOFTIRQ,
        cpu: bpf_get_smp_processor_id(),
        id: vec as __u64,
    };

    if vec < NR_SOFTIRQS {
        return update_timestart_and_name(
            &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
            &mut perf_kwork_names as *mut _ as *mut core::ffi::c_void,
            &mut key,
            softirq_name_list[vec as usize].as_mut_ptr(),
        );
    }

    0
}

// SEC("tracepoint/irq/softirq_entry")
#[no_mangle]
pub unsafe extern "C" fn latency_softirq_entry(ctx: *mut trace_event_raw_softirq) -> i32 {
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_SOFTIRQ,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).vec as __u64,
    };

    update_timeend(
        &mut perf_kwork_report as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut key,
    )
}

// SEC("tracepoint/workqueue/workqueue_execute_start")
#[no_mangle]
pub unsafe extern "C" fn report_workqueue_execute_start(
    ctx: *mut trace_event_raw_workqueue_execute_start,
) -> i32 {
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_WORKQUEUE,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).work as __u64,
    };

    update_timestart(
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut key,
    )
}

// SEC("tracepoint/workqueue/workqueue_execute_end")
#[no_mangle]
pub unsafe extern "C" fn report_workqueue_execute_end(
    ctx: *mut trace_event_raw_workqueue_execute_end,
) -> i32 {
    let mut name: [core::ffi::c_char; MAX_KWORKNAME] = [0; MAX_KWORKNAME];
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_WORKQUEUE,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).work as __u64,
    };
    let func_addr: u64 = (*ctx).function as u64;

    core::ptr::write_bytes(
        name.as_mut_ptr() as *mut u8,
        0,
        core::mem::size_of_val(&name),
    );
    bpf_snprintf(
        name.as_mut_ptr(),
        core::mem::size_of_val(&name) as __u32,
        b"%ps\0".as_ptr() as *const core::ffi::c_char,
        &func_addr as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&func_addr) as __u32,
    );

    update_timeend_and_name(
        &mut perf_kwork_report as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_names as *mut _ as *mut core::ffi::c_void,
        &mut key,
        name.as_mut_ptr(),
    )
}

// SEC("tracepoint/workqueue/workqueue_activate_work")
#[no_mangle]
pub unsafe extern "C" fn latency_workqueue_activate_work(
    ctx: *mut trace_event_raw_workqueue_activate_work,
) -> i32 {
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_WORKQUEUE,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).work as __u64,
    };

    update_timestart(
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut key,
    )
}

// SEC("tracepoint/workqueue/workqueue_execute_start")
#[no_mangle]
pub unsafe extern "C" fn latency_workqueue_execute_start(
    ctx: *mut trace_event_raw_workqueue_execute_start,
) -> i32 {
    let mut name: [core::ffi::c_char; MAX_KWORKNAME] = [0; MAX_KWORKNAME];
    let mut key: work_key = work_key {
        type_: KWORK_CLASS_WORKQUEUE,
        cpu: bpf_get_smp_processor_id(),
        id: (*ctx).work as __u64,
    };
    let func_addr: u64 = (*ctx).function as u64;

    core::ptr::write_bytes(
        name.as_mut_ptr() as *mut u8,
        0,
        core::mem::size_of_val(&name),
    );
    bpf_snprintf(
        name.as_mut_ptr(),
        core::mem::size_of_val(&name) as __u32,
        b"%ps\0".as_ptr() as *const core::ffi::c_char,
        &func_addr as *const _ as *const core::ffi::c_void,
        core::mem::size_of_val(&func_addr) as __u32,
    );

    update_timeend_and_name(
        &mut perf_kwork_report as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_time as *mut _ as *mut core::ffi::c_void,
        &mut perf_kwork_names as *mut _ as *mut core::ffi::c_void,
        &mut key,
        name.as_mut_ptr(),
    )
}

// SEC("license")
#[no_mangle]
pub static mut LICENSE: [core::ffi::c_char; 13] = c_array_13(b"Dual BSD/GPL\0");

const fn c_array_13(src: &[u8]) -> [core::ffi::c_char; 13] {
    let mut out = [0 as core::ffi::c_char; 13];
    let mut i = 0;
    while i < src.len() {
        out[i] = src[i] as core::ffi::c_char;
        i += 1;
    }
    out
}
