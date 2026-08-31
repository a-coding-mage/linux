// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2020 Facebook
// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub type __u32 = u32;
pub type __u64 = u64;
pub type u32 = __u32;
pub type u64 = __u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_perf_event_value___local {
    pub counter: __u64,
    pub enabled: __u64,
    pub running: __u64,
}

#[repr(C)]
pub struct bpf_map_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
}

unsafe extern "C" {
    static BPF_MAP_TYPE_PERF_EVENT_ARRAY: u32;
    static BPF_MAP_TYPE_PERCPU_ARRAY: u32;

    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_map_lookup_elem(map: *const core::ffi::c_void, key: *const core::ffi::c_void)
        -> *mut core::ffi::c_void;
    fn bpf_perf_event_read_value(
        map: *const core::ffi::c_void,
        flags: u64,
        buf: *mut core::ffi::c_void,
        buf_size: u32,
    ) -> i32;
}

/* map of perf event fds, num_cpu * num_metric entries */
// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
// __uint(key_size, sizeof(u32));
// __uint(value_size, sizeof(int));
#[unsafe(link_section = ".maps")]
pub static mut events: bpf_map_def = bpf_map_def {
    type_: unsafe { BPF_MAP_TYPE_PERF_EVENT_ARRAY },
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<i32>() as u32,
};

/* readings at fentry */
// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(key_size, sizeof(u32));
// __uint(value_size, sizeof(struct bpf_perf_event_value___local));
#[unsafe(link_section = ".maps")]
pub static mut fentry_readings: bpf_map_def = bpf_map_def {
    type_: unsafe { BPF_MAP_TYPE_PERCPU_ARRAY },
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<bpf_perf_event_value___local>() as u32,
};

/* accumulated readings */
// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(key_size, sizeof(u32));
// __uint(value_size, sizeof(struct bpf_perf_event_value___local));
#[unsafe(link_section = ".maps")]
pub static mut accum_readings: bpf_map_def = bpf_map_def {
    type_: unsafe { BPF_MAP_TYPE_PERCPU_ARRAY },
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<bpf_perf_event_value___local>() as u32,
};

/* sample counts, one per cpu */
// SEC(".maps")
// __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
// __uint(key_size, sizeof(u32));
// __uint(value_size, sizeof(u64));
#[unsafe(link_section = ".maps")]
pub static mut counts: bpf_map_def = bpf_map_def {
    type_: unsafe { BPF_MAP_TYPE_PERCPU_ARRAY },
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
};

// const volatile __u32
pub static mut num_cpu: __u32 = 1;
// const volatile __u32
pub static mut num_metric: __u32 = 1;
pub const MAX_NUM_METRICS: usize = 4;

// SEC("fentry/XXX")
// int BPF_PROG(fentry_XXX)
#[unsafe(link_section = "fentry/XXX")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fentry_XXX() -> i32 {
    let mut ptrs: [*mut bpf_perf_event_value___local; MAX_NUM_METRICS] =
        [core::ptr::null_mut(); MAX_NUM_METRICS];
    let mut key: u32 = unsafe { bpf_get_smp_processor_id() };
    let mut i: u32;

    /* look up before reading, to reduce error */
    i = 0;
    while i < unsafe { core::ptr::read_volatile(core::ptr::addr_of!(num_metric)) }
        && (i as usize) < MAX_NUM_METRICS
    {
        let mut flag: u32 = i;

        ptrs[i as usize] = unsafe {
            bpf_map_lookup_elem(
                core::ptr::addr_of!(fentry_readings) as *const core::ffi::c_void,
                (&mut flag as *mut u32).cast::<core::ffi::c_void>(),
            ) as *mut bpf_perf_event_value___local
        };
        if ptrs[i as usize].is_null() {
            return 0;
        }
        i = i.wrapping_add(1);
    }

    i = 0;
    while i < unsafe { core::ptr::read_volatile(core::ptr::addr_of!(num_metric)) }
        && (i as usize) < MAX_NUM_METRICS
    {
        let mut reading = bpf_perf_event_value___local {
            counter: 0,
            enabled: 0,
            running: 0,
        };
        let err: i32;

        err = unsafe {
            bpf_perf_event_read_value(
                core::ptr::addr_of!(events) as *const core::ffi::c_void,
                key as u64,
                (&mut reading as *mut bpf_perf_event_value___local).cast::<core::ffi::c_void>(),
                core::mem::size_of_val(&reading) as u32,
            )
        };
        if err != 0 {
            return 0;
        }
        unsafe {
            *ptrs[i as usize] = reading;
        }
        key = key.wrapping_add(unsafe { core::ptr::read_volatile(core::ptr::addr_of!(num_cpu)) });
        i = i.wrapping_add(1);
    }

    return 0;
}

#[inline]
unsafe fn fexit_update_maps(id: u32, after: *mut bpf_perf_event_value___local) {
    let before: *mut bpf_perf_event_value___local;
    let mut diff: bpf_perf_event_value___local = bpf_perf_event_value___local {
        counter: 0,
        enabled: 0,
        running: 0,
    };

    before = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of!(fentry_readings) as *const core::ffi::c_void,
            (&id as *const u32).cast::<core::ffi::c_void>(),
        ) as *mut bpf_perf_event_value___local
    };
    /* only account samples with a valid fentry_reading */
    if !before.is_null() && unsafe { (*before).counter } != 0 {
        let accum: *mut bpf_perf_event_value___local;

        diff.counter = unsafe { (*after).counter.wrapping_sub((*before).counter) };
        diff.enabled = unsafe { (*after).enabled.wrapping_sub((*before).enabled) };
        diff.running = unsafe { (*after).running.wrapping_sub((*before).running) };

        accum = unsafe {
            bpf_map_lookup_elem(
                core::ptr::addr_of!(accum_readings) as *const core::ffi::c_void,
                (&id as *const u32).cast::<core::ffi::c_void>(),
            ) as *mut bpf_perf_event_value___local
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

// SEC("fexit/XXX")
// int BPF_PROG(fexit_XXX)
#[unsafe(link_section = "fexit/XXX")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fexit_XXX() -> i32 {
    let mut readings: [bpf_perf_event_value___local; MAX_NUM_METRICS] =
        [bpf_perf_event_value___local {
            counter: 0,
            enabled: 0,
            running: 0,
        }; MAX_NUM_METRICS];
    let cpu: u32 = unsafe { bpf_get_smp_processor_id() };
    let mut i: u32;
    let mut zero: u32 = 0;
    let mut err: i32;
    let count: *mut u64;

    /* read all events before updating the maps, to reduce error */
    i = 0;
    while i < unsafe { core::ptr::read_volatile(core::ptr::addr_of!(num_metric)) }
        && (i as usize) < MAX_NUM_METRICS
    {
        err = unsafe {
            bpf_perf_event_read_value(
                core::ptr::addr_of!(events) as *const core::ffi::c_void,
                cpu.wrapping_add(i.wrapping_mul(unsafe {
                    core::ptr::read_volatile(core::ptr::addr_of!(num_cpu))
                })) as u64,
                readings.as_mut_ptr().add(i as usize).cast::<core::ffi::c_void>(),
                core::mem::size_of::<bpf_perf_event_value___local>() as u32,
            )
        };
        if err != 0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    count = unsafe {
        bpf_map_lookup_elem(
            core::ptr::addr_of!(counts) as *const core::ffi::c_void,
            (&mut zero as *mut u32).cast::<core::ffi::c_void>(),
        ) as *mut u64
    };
    if !count.is_null() {
        unsafe {
            *count = (*count).wrapping_add(1);
        }
        i = 0;
        while i < unsafe { core::ptr::read_volatile(core::ptr::addr_of!(num_metric)) }
            && (i as usize) < MAX_NUM_METRICS
        {
            unsafe {
                fexit_update_maps(i, readings.as_mut_ptr().add(i as usize));
            }
            i = i.wrapping_add(1);
        }
    }
    return 0;
}

// SEC("license")
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut LICENSE: [u8; 13] = *b"Dual BSD/GPL\0";
