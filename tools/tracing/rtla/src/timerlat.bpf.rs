// SPDX-License-Identifier: GPL-2.0
// C includes removed: <linux/bpf.h>, <bpf/bpf_tracing.h>, <stdbool.h>, "timerlat_bpf.h".
// The BPF helpers, SEC placement, map declaration macros, and SUMMARY_* constants are
// expected to be supplied by the surrounding build/bindings.

pub const MAX_ENTRIES_DEFAULT: u32 = 4096;

#[used]
#[link_section = "license"]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct trace_event_raw_timerlat_sample {
    pub timer_latency: u64,
    pub context: i32,
}

// Original C map declarations used libbpf BTF macros:
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(max_entries, MAX_ENTRIES_DEFAULT);
// __type(key, unsigned int);
// __type(value, unsigned long long);
#[link_section = ".maps"]
pub static mut hist_irq: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: MAX_ENTRIES_DEFAULT,
    map_flags: 0,
};

#[link_section = ".maps"]
pub static mut hist_thread: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: MAX_ENTRIES_DEFAULT,
    map_flags: 0,
};

#[link_section = ".maps"]
pub static mut hist_user: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: MAX_ENTRIES_DEFAULT,
    map_flags: 0,
};

// Original C map declarations used libbpf BTF macros:
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(max_entries, SUMMARY_FIELD_N);
// __type(key, unsigned int);
// __type(value, unsigned long long);
#[link_section = ".maps"]
pub static mut summary_irq: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: SUMMARY_FIELD_N,
    map_flags: 0,
};

#[link_section = ".maps"]
pub static mut summary_thread: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: SUMMARY_FIELD_N,
    map_flags: 0,
};

#[link_section = ".maps"]
pub static mut summary_user: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PERCPU_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: SUMMARY_FIELD_N,
    map_flags: 0,
};

// Original C map declaration used libbpf BTF macros:
// __uint(type, BPF_MAP_TYPE_ARRAY);
// __uint(max_entries, 1);
// __type(key, unsigned int);
// __type(value, unsigned long long);
#[link_section = ".maps"]
pub static mut stop_tracing: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
    max_entries: 1,
    map_flags: 0,
};

// Original C map declaration used libbpf BTF macros:
// __uint(type, BPF_MAP_TYPE_RINGBUF);
// __uint(max_entries, 1);
#[link_section = ".maps"]
pub static mut signal_stop_tracing: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_RINGBUF,
    key_size: 0,
    value_size: 0,
    max_entries: 1,
    map_flags: 0,
};

// Original C map declaration used libbpf BTF macros:
// __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
// __uint(key_size, sizeof(unsigned int));
// __uint(max_entries, 1);
// __array(values, unsigned int (void *));
#[link_section = ".maps"]
pub static mut bpf_action: bpf_map_def = bpf_map_def {
    type_: BPF_MAP_TYPE_PROG_ARRAY,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u32>() as u32,
    max_entries: 1,
    map_flags: 0,
};

/* Params to be set by rtla */
pub static mut bucket_size: i32 = 1;
pub static mut output_divisor: i32 = 1000;
pub static mut entries: i32 = 256;
pub static mut irq_threshold: i32 = 0;
pub static mut thread_threshold: i32 = 0;
pub static mut aa_only: bool = false;

#[inline(always)]
pub unsafe fn map_get(map: *mut core::ffi::c_void, key: u32) -> u64 {
    let value_ptr: *mut u64;

    value_ptr = bpf_map_lookup_elem(map, &key as *const u32 as *const core::ffi::c_void) as *mut u64;

    if value_ptr.is_null() {
        0
    } else {
        *value_ptr
    }
}

#[inline(always)]
pub unsafe fn map_set(map: *mut core::ffi::c_void, key: u32, value: u64) {
    bpf_map_update_elem(
        map,
        &key as *const u32 as *const core::ffi::c_void,
        &value as *const u64 as *const core::ffi::c_void,
        BPF_ANY as u64,
    );
}

#[inline(always)]
pub unsafe fn map_increment(map: *mut core::ffi::c_void, key: u32) {
    map_set(map, key, map_get(map, key).wrapping_add(1));
}

#[inline(always)]
pub unsafe fn update_main_hist(map: *mut core::ffi::c_void, bucket: i32) {
    if core::ptr::read_volatile(core::ptr::addr_of!(entries)) == 0 {
        /* No histogram */
        return;
    }

    if bucket >= core::ptr::read_volatile(core::ptr::addr_of!(entries)) {
        /* Overflow */
        return;
    }

    map_increment(map, bucket as u32);
}

#[inline(always)]
pub unsafe fn update_summary(map: *mut core::ffi::c_void, latency: u64, bucket: i32) {
    if core::ptr::read_volatile(core::ptr::addr_of!(aa_only)) {
        /* Auto-analysis only, nothing to be done here */
        return;
    }

    map_set(map, SUMMARY_CURRENT, latency);

    if bucket >= core::ptr::read_volatile(core::ptr::addr_of!(entries)) {
        /* Overflow */
        map_increment(map, SUMMARY_OVERFLOW);
    }

    if latency > map_get(map, SUMMARY_MAX) {
        map_set(map, SUMMARY_MAX, latency);
    }

    if latency < map_get(map, SUMMARY_MIN) || map_get(map, SUMMARY_COUNT) == 0 {
        map_set(map, SUMMARY_MIN, latency);
    }

    map_increment(map, SUMMARY_COUNT);
    map_set(map, SUMMARY_SUM, map_get(map, SUMMARY_SUM).wrapping_add(latency));
}

#[inline(always)]
pub unsafe fn set_stop_tracing(tp_args: *mut trace_event_raw_timerlat_sample) {
    let value: i32 = 0;

    /* Suppress further sample processing */
    map_set(
        core::ptr::addr_of_mut!(stop_tracing) as *mut core::ffi::c_void,
        0,
        1,
    );

    /* Signal to userspace */
    bpf_ringbuf_output(
        core::ptr::addr_of_mut!(signal_stop_tracing) as *mut core::ffi::c_void,
        &value as *const i32 as *const core::ffi::c_void,
        core::mem::size_of_val(&value) as u64,
        0,
    );

    /*
     * Call into BPF action program, if attached.
     * Otherwise, just silently fail.
     */
    bpf_tail_call(
        tp_args as *mut core::ffi::c_void,
        core::ptr::addr_of_mut!(bpf_action) as *mut core::ffi::c_void,
        0,
    );
}

#[link_section = "tp/osnoise/timerlat_sample"]
pub unsafe extern "C" fn handle_timerlat_sample(
    tp_args: *mut trace_event_raw_timerlat_sample,
) -> i32 {
    let latency: u64;
    let latency_us: u64;
    let bucket: i32;

    if map_get(core::ptr::addr_of_mut!(stop_tracing) as *mut core::ffi::c_void, 0) != 0 {
        return 0;
    }

    latency = (*tp_args).timer_latency / core::ptr::read_volatile(core::ptr::addr_of!(output_divisor)) as u64;
    latency_us = (*tp_args).timer_latency / 1000;
    bucket = (latency / core::ptr::read_volatile(core::ptr::addr_of!(bucket_size)) as u64) as i32;

    if (*tp_args).context == 0 {
        update_main_hist(
            core::ptr::addr_of_mut!(hist_irq) as *mut core::ffi::c_void,
            bucket,
        );
        update_summary(
            core::ptr::addr_of_mut!(summary_irq) as *mut core::ffi::c_void,
            latency,
            bucket,
        );

        if core::ptr::read_volatile(core::ptr::addr_of!(irq_threshold)) != 0
            && latency_us >= core::ptr::read_volatile(core::ptr::addr_of!(irq_threshold)) as u64
        {
            set_stop_tracing(tp_args);
        }
    } else if (*tp_args).context == 1 {
        update_main_hist(
            core::ptr::addr_of_mut!(hist_thread) as *mut core::ffi::c_void,
            bucket,
        );
        update_summary(
            core::ptr::addr_of_mut!(summary_thread) as *mut core::ffi::c_void,
            latency,
            bucket,
        );

        if core::ptr::read_volatile(core::ptr::addr_of!(thread_threshold)) != 0
            && latency_us >= core::ptr::read_volatile(core::ptr::addr_of!(thread_threshold)) as u64
        {
            set_stop_tracing(tp_args);
        }
    } else {
        update_main_hist(
            core::ptr::addr_of_mut!(hist_user) as *mut core::ffi::c_void,
            bucket,
        );
        update_summary(
            core::ptr::addr_of_mut!(summary_user) as *mut core::ffi::c_void,
            latency,
            bucket,
        );

        if core::ptr::read_volatile(core::ptr::addr_of!(thread_threshold)) != 0
            && latency_us >= core::ptr::read_volatile(core::ptr::addr_of!(thread_threshold)) as u64
        {
            set_stop_tracing(tp_args);
        }
    }

    0
}
