/* Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Supplied by vmlinux.h, linux/version.h, bpf_helpers.h, and bpf_tracing.h.

#[repr(C)]
pub struct start_key {
    pub dev: dev_t,
    pub _pad: u32,
    pub sector: sector_t,
}

// BPF_MAP_TYPE_HASH, key = long, value = u64, max_entries = 4096.
#[repr(C)]
pub struct my_map_t {
    _private: [u8; 0],
}

extern "C" {
    pub static mut my_map: my_map_t;
}

// from /sys/kernel/tracing/events/block/block_io_start/format
#[link_section = "tracepoint/block/block_io_start"]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut trace_event_raw_block_rq) -> i32 {
    let val: u64 = bpf_ktime_get_ns();
    let key = start_key {
        dev: (*ctx).dev,
        _pad: 0,
        sector: (*ctx).sector,
    };

    bpf_map_update_elem(
        &mut my_map as *mut my_map_t as *mut _,
        &key as *const start_key as *const _,
        &val as *const u64 as *const _,
        BPF_ANY,
    );
    0
}

#[inline]
unsafe fn log2l(mut n: u64) -> u32 {
    let mut i: i32 = -((n == 0) as i32);
    if n >= (1u64 << 32) { i += 32; n >>= 32; }
    if n >= (1u64 << 16) { i += 16; n >>= 16; }
    if n >= (1u64 << 8) { i += 8; n >>= 8; }
    if n >= (1u64 << 4) { i += 4; n >>= 4; }
    if n >= (1u64 << 2) { i += 2; n >>= 2; }
    if n >= (1u64 << 1) { i += 1; n >>= 1; }
    i as u32
}

pub const SLOTS: u32 = 100;

// BPF_MAP_TYPE_PERCPU_ARRAY, key_size = sizeof(u32),
// value_size = sizeof(u64), max_entries = SLOTS.
#[repr(C)]
pub struct lat_map_t {
    _private: [u8; 0],
}

extern "C" {
    pub static mut lat_map: lat_map_t;
}

// from /sys/kernel/tracing/events/block/block_io_done/format
#[link_section = "tracepoint/block/block_io_done"]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut trace_event_raw_block_rq) -> i32 {
    let key = start_key {
        dev: (*ctx).dev,
        _pad: 0,
        sector: (*ctx).sector,
    };

    let value: *mut u64;
    let l: u64;
    let base: u64;
    let index: u32;

    value = bpf_map_lookup_elem(
        &mut my_map as *mut my_map_t as *mut _,
        &key as *const start_key as *const _,
    ) as *mut u64;
    if value.is_null() {
        return 0;
    }

    let cur_time: u64 = bpf_ktime_get_ns();
    let delta: u64 = cur_time.wrapping_sub(*value);

    bpf_map_delete_elem(
        &mut my_map as *mut my_map_t as *mut _,
        &key as *const start_key as *const _,
    );

    /* the lines below are computing index = log10(delta)*10
     * using integer arithmetic
     * index = 29 ~ 1 usec
     * index = 59 ~ 1 msec
     * index = 89 ~ 1 sec
     * index = 99 ~ 10sec or more
     * log10(x)*10 = log2(x)*10/log2(10) = log2(x)*3
     */
    l = log2l(delta) as u64;
    base = 1u64 << l;
    index = (((l * 64).wrapping_add(
        (delta.wrapping_sub(base)).wrapping_mul(64) / base,
    )) * 3 / 64) as u32;

    let index = if index >= SLOTS { SLOTS - 1 } else { index };

    let value = bpf_map_lookup_elem(
        &mut lat_map as *mut lat_map_t as *mut _,
        &index as *const u32 as *const _,
    ) as *mut u64;
    if !value.is_null() {
        *value = (*value).wrapping_add(1);
    }

    0
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[link_section = "version"]
#[no_mangle]
pub static mut _version: u32 = LINUX_VERSION_CODE;

extern "C" {
    fn bpf_ktime_get_ns() -> u64;
    fn bpf_map_update_elem(map: *mut _, key: *const _, value: *const _, flags: u64) -> i64;
    fn bpf_map_lookup_elem(map: *mut _, key: *const _) -> *mut _;
    fn bpf_map_delete_elem(map: *mut _, key: *const _) -> i64;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
