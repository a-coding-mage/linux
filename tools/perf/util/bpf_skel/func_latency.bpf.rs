// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2021 Google
// Dependencies from the original C source:
// "vmlinux.h", <bpf/bpf_helpers.h>, and <bpf/bpf_tracing.h>

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type __s64 = i64;

const BPF_MAP_TYPE_HASH: __u32 = 1;
const BPF_MAP_TYPE_PERCPU_ARRAY: __u32 = 5;
const BPF_ANY: __u64 = 0;

// This should be in sync with "util/ftrace.h"
const NUM_BUCKET: __u32 = 22;

#[repr(C)]
pub struct bpf_map_def {
    pub type_: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
}

// SEC(".maps")
#[no_mangle]
pub static mut functime: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u64>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
    max_entries: 10000,
};

// SEC(".maps")
#[no_mangle]
pub static mut cpu_filter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u8>() as __u32,
    max_entries: 1,
};

// SEC(".maps")
#[no_mangle]
pub static mut task_filter: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u8>() as __u32,
    max_entries: 1,
};

// SEC(".maps")
#[no_mangle]
pub static mut latency: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<__u32>() as __u32,
    value_size: core::mem::size_of::<__u64>() as __u32,
    max_entries: NUM_BUCKET,
};

#[no_mangle]
pub static mut enabled: i32 = 0;

// stats
#[no_mangle]
pub static mut total: __s64 = 0;
#[no_mangle]
pub static mut count: __s64 = 0;
#[no_mangle]
pub static mut max: __s64 = 0;
#[no_mangle]
pub static mut min: __s64 = 0;

#[no_mangle]
pub static has_cpu: i32 = 0;
#[no_mangle]
pub static has_task: i32 = 0;
#[no_mangle]
pub static use_nsec: i32 = 0;
#[no_mangle]
pub static bucket_range: u32 = 0;
#[no_mangle]
pub static min_latency: u32 = 0;
#[no_mangle]
pub static max_latency: u32 = 0;
#[no_mangle]
pub static bucket_num: u32 = NUM_BUCKET;

extern "C" {
    fn bpf_get_smp_processor_id() -> __u32;
    fn bpf_get_current_pid_tgid() -> __u64;
    fn bpf_ktime_get_ns() -> __u64;
    fn bpf_map_lookup_elem(map: *mut bpf_map_def, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: __u64,
    ) -> i64;
    fn bpf_map_delete_elem(map: *mut bpf_map_def, key: *const core::ffi::c_void) -> i64;
}

unsafe fn can_record() -> bool {
    if core::ptr::read_volatile(&has_cpu) != 0 {
        let cpu: __u32 = bpf_get_smp_processor_id();
        let ok: *mut __u8;

        ok = bpf_map_lookup_elem(
            &mut cpu_filter,
            &cpu as *const __u32 as *const core::ffi::c_void,
        ) as *mut __u8;
        if ok.is_null() {
            return false;
        }
    }

    if core::ptr::read_volatile(&has_task) != 0 {
        let pid: __u32 = bpf_get_current_pid_tgid() as __u32;
        let ok: *mut __u8;

        ok = bpf_map_lookup_elem(
            &mut task_filter,
            &pid as *const __u32 as *const core::ffi::c_void,
        ) as *mut __u8;
        if ok.is_null() {
            return false;
        }
    }
    true
}

unsafe fn update_latency(delta: __s64) {
    let mut val: __u64 = delta as __u64;
    let mut key: __u32 = 0;
    let hist: *mut __u64;
    let cmp_base: __u64 = if core::ptr::read_volatile(&use_nsec) != 0 {
        1
    } else {
        1000
    };

    if delta < 0 {
        return;
    }

    if core::ptr::read_volatile(&bucket_range) != 0 {
        val = (delta as __u64) / cmp_base;

        if core::ptr::read_volatile(&min_latency) > 0 {
            if val > core::ptr::read_volatile(&min_latency) as __u64 {
                val = val.wrapping_sub(core::ptr::read_volatile(&min_latency) as __u64);
            } else {
                hist = bpf_map_lookup_elem(
                    &mut latency,
                    &key as *const __u32 as *const core::ffi::c_void,
                ) as *mut __u64;
                if hist.is_null() {
                    return;
                }

                core::intrinsics::atomic_xadd_relaxed(hist, 1);

                core::intrinsics::atomic_xadd_relaxed(&mut total, delta);
                core::intrinsics::atomic_xadd_relaxed(&mut count, 1);

                if delta > max {
                    max = delta;
                }
                if delta < min {
                    min = delta;
                }
                return;
            }
        }

        // Less than 1 unit (ms or ns), or, in the future,
        // than the min latency desired.
        if val > 0 {
            // 1st entry: [ 1 unit .. bucket_range units )
            key = (val / core::ptr::read_volatile(&bucket_range) as __u64 + 1) as __u32;
            if key >= core::ptr::read_volatile(&bucket_num) {
                key = core::ptr::read_volatile(&bucket_num) - 1;
            }
        }
    } else {
        // calculate index using delta
        key = 0;
        while key < core::ptr::read_volatile(&bucket_num) - 1 {
            if (delta as __u64) < (cmp_base << key) {
                break;
            }
            key += 1;
        }
    }

    hist = bpf_map_lookup_elem(
        &mut latency,
        &key as *const __u32 as *const core::ffi::c_void,
    ) as *mut __u64;
    if hist.is_null() {
        return;
    }

    core::intrinsics::atomic_xadd_relaxed(hist, 1);

    core::intrinsics::atomic_xadd_relaxed(&mut total, delta); // always in nsec
    core::intrinsics::atomic_xadd_relaxed(&mut count, 1);

    if delta > max {
        max = delta;
    }
    if delta < min {
        min = delta;
    }
}

// SEC("kprobe/func")
#[no_mangle]
pub unsafe extern "C" fn func_begin() -> i32 {
    let key: __u64;
    let now: __u64;

    if enabled == 0 || !can_record() {
        return 0;
    }

    key = bpf_get_current_pid_tgid();
    now = bpf_ktime_get_ns();

    // overwrite timestamp for nested functions
    bpf_map_update_elem(
        &mut functime,
        &key as *const __u64 as *const core::ffi::c_void,
        &now as *const __u64 as *const core::ffi::c_void,
        BPF_ANY,
    );
    0
}

// SEC("kretprobe/func")
#[no_mangle]
pub unsafe extern "C" fn func_end() -> i32 {
    let tid: __u64;
    let start: *mut __u64;

    if enabled == 0 {
        return 0;
    }

    tid = bpf_get_current_pid_tgid();

    start = bpf_map_lookup_elem(
        &mut functime,
        &tid as *const __u64 as *const core::ffi::c_void,
    ) as *mut __u64;
    if !start.is_null() {
        update_latency((bpf_ktime_get_ns() - *start) as __s64);
        bpf_map_delete_elem(
            &mut functime,
            &tid as *const __u64 as *const core::ffi::c_void,
        );
    }

    0
}

// SEC("raw_tp")
#[no_mangle]
pub unsafe extern "C" fn event_begin() -> i32 {
    let key: __u64;
    let now: __u64;

    if enabled == 0 || !can_record() {
        return 0;
    }

    key = bpf_get_current_pid_tgid();
    now = bpf_ktime_get_ns();

    // overwrite timestamp for nested events
    bpf_map_update_elem(
        &mut functime,
        &key as *const __u64 as *const core::ffi::c_void,
        &now as *const __u64 as *const core::ffi::c_void,
        BPF_ANY,
    );
    0
}

// SEC("raw_tp")
#[no_mangle]
pub unsafe extern "C" fn event_end() -> i32 {
    let tid: __u64;
    let start: *mut __u64;

    if enabled == 0 {
        return 0;
    }

    tid = bpf_get_current_pid_tgid();

    start = bpf_map_lookup_elem(
        &mut functime,
        &tid as *const __u64 as *const core::ffi::c_void,
    ) as *mut __u64;
    if !start.is_null() {
        update_latency((bpf_ktime_get_ns() - *start) as __s64);
        bpf_map_delete_elem(
            &mut functime,
            &tid as *const __u64 as *const core::ffi::c_void,
        );
    }

    0
}
