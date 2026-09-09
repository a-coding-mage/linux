// Translated from tracex6.bpf.c.
// C includes provide the kernel, BPF helper, tracing, and CO-RE definitions
// used below; they are intentionally not reimplemented in this file.

use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_perf_event_value {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map_def {
    // __uint/__type map metadata is supplied by the BPF toolchain.
    _private: [u8; 0],
}

// BPF_MAP_TYPE_PERF_EVENT_ARRAY, BPF_MAP_TYPE_HASH, BPF_NOEXIST,
// LINUX_VERSION_CODE, and enum bpf_map_type are supplied by the included
// kernel/BPF headers.
extern "C" {
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_perf_event_read(map: *mut bpf_map_def, key: u32) -> u64;
    fn bpf_perf_event_read_value(
        map: *mut bpf_map_def,
        key: u32,
        buf: *mut bpf_perf_event_value,
        size: u32,
    ) -> i32;
    fn bpf_map_lookup_elem(map: *mut bpf_map_def, key: *const u32) -> *mut u64;
    fn bpf_map_update_elem(
        map: *mut bpf_map_def,
        key: *const u32,
        value: *const c_void,
        flags: u64,
    ) -> i32;
}

#[repr(C)]
pub struct counters_map {
    // __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    // __uint(key_size, sizeof(int));
    // __uint(value_size, sizeof(u32));
    // __uint(max_entries, 64);
    _private: [u8; 0],
}

#[link_section = ".maps"]
pub static mut counters: counters_map = counters_map { _private: [] };

#[repr(C)]
pub struct values_map {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, int);
    // __type(value, u64);
    // __uint(max_entries, 64);
    _private: [u8; 0],
}

#[link_section = ".maps"]
pub static mut values: values_map = values_map { _private: [] };

#[repr(C)]
pub struct values2_map {
    // __uint(type, BPF_MAP_TYPE_HASH);
    // __type(key, int);
    // __type(value, struct bpf_perf_event_value);
    // __uint(max_entries, 64);
    _private: [u8; 0],
}

#[link_section = ".maps"]
pub static mut values2: values2_map = values2_map { _private: [] };

// SEC("kprobe/htab_map_get_next_key")
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    let key: u32 = bpf_get_smp_processor_id();
    let count: u64;
    let val: *mut u64;
    let error: i64;

    count = bpf_perf_event_read(
        &mut counters as *mut counters_map as *mut bpf_map_def,
        key,
    );
    error = count as i64;
    if error <= -2 && error >= -22 {
        return 0;
    }

    val = bpf_map_lookup_elem(
        &mut values as *mut values_map as *mut bpf_map_def,
        &key,
    );
    if !val.is_null() {
        *val = count;
    } else {
        bpf_map_update_elem(
            &mut values as *mut values_map as *mut bpf_map_def,
            &key,
            &count as *const u64 as *const c_void,
            1, // BPF_NOEXIST
        );
    }

    0
}

/*
 * Since *_map_lookup_elem can't be expected to trigger bpf programs
 * due to potential deadlocks (bpf_disable_instrumentation), this bpf
 * program will be attached to bpf_map_copy_value (which is called
 * from map_lookup_elem) and will only filter the hashtable type.
 */
// SEC("kprobe/bpf_map_copy_value")
// BPF_KPROBE(bpf_prog2, struct bpf_map *map)
pub unsafe extern "C" fn bpf_prog2(map: *mut bpf_map) -> i32 {
    let key: u32 = bpf_get_smp_processor_id();
    let val: *mut bpf_perf_event_value;
    let mut buf: bpf_perf_event_value;
    let type_: i32;
    let error: i32;

    // BPF_CORE_READ(map, map_type);
    type_ = *(map as *mut i32);
    // BPF_MAP_TYPE_HASH is supplied by the included kernel headers.
    if type_ != 1 {
        return 0;
    }

    error = bpf_perf_event_read_value(
        &mut counters as *mut counters_map as *mut bpf_map_def,
        key,
        &mut buf,
        core::mem::size_of::<bpf_perf_event_value>() as u32,
    );
    if error != 0 {
        return 0;
    }

    // The value type of values2 is struct bpf_perf_event_value.
    val = bpf_map_lookup_elem(
        &mut values2 as *mut values2_map as *mut bpf_map_def,
        &key,
    ) as *mut bpf_perf_event_value;
    if !val.is_null() {
        *val = buf;
    } else {
        bpf_map_update_elem(
            &mut values2 as *mut values2_map as *mut bpf_map_def,
            &key,
            &buf as *const bpf_perf_event_value as *const c_void,
            1, // BPF_NOEXIST
        );
    }

    0
}

#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "version"]
pub static mut _version: u32 = 0; // LINUX_VERSION_CODE

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
