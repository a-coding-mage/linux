/* Copyright (c) 2013-2015 PLUMgrid, http://plumgrid.com
 * Copyright (c) 2015 BMW Car IT GmbH
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// C dependencies supplied by the kernel/BPF environment:
// linux/version.h, linux/ptrace.h, uapi/linux/bpf.h, bpf/bpf_helpers.h

use core::ffi::c_void;

pub const MAX_ENTRIES: usize = 20;
pub const MAX_CPU: usize = 4;

/* We need to stick to static allocated memory (an array instead of
 * hash table) because managing dynamic memory from the
 * trace_preempt_[on|off] tracepoints hooks is not supported.
 */

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BpfMap {
    _private: [u8; 0],
}

// BPF map declarations corresponding to the C SEC(".maps") definitions.
#[no_mangle]
#[link_section = ".maps"]
pub static mut my_map: BpfMap = BpfMap { _private: [] };

#[no_mangle]
#[link_section = ".maps"]
pub static mut my_lat: BpfMap = BpfMap { _private: [] };

extern "C" {
    fn bpf_get_smp_processor_id() -> i32;
    fn bpf_map_lookup_elem(map: *mut BpfMap, key: *const c_void) -> *mut c_void;
    fn bpf_ktime_get_ns() -> u64;
}

#[no_mangle]
#[link_section = "kprobe/trace_preempt_off"]
pub unsafe extern "C" fn bpf_prog1(_ctx: *mut pt_regs) -> i32 {
    let cpu: i32 = bpf_get_smp_processor_id();
    let ts: *mut u64 = bpf_map_lookup_elem(
        &mut my_map,
        &cpu as *const i32 as *const c_void,
    ) as *mut u64;

    if !ts.is_null() {
        *ts = bpf_ktime_get_ns();
    }

    0
}

unsafe fn log2(mut v: u32) -> u32 {
    let mut r: u32;
    let mut shift: u32;

    r = ((v > 0xFFFF) as u32) << 4;
    v >>= r;
    shift = ((v > 0xFF) as u32) << 3;
    v >>= shift;
    r |= shift;
    shift = ((v > 0xF) as u32) << 2;
    v >>= shift;
    r |= shift;
    shift = ((v > 0x3) as u32) << 1;
    v >>= shift;
    r |= shift;
    r |= v >> 1;

    r
}

unsafe fn log2l(v: u64) -> u32 {
    let hi: u32 = (v >> 32) as u32;

    if hi != 0 {
        log2(hi).wrapping_add(32)
    } else {
        log2(v as u32)
    }
}

#[no_mangle]
#[link_section = "kprobe/trace_preempt_on"]
pub unsafe extern "C" fn bpf_prog2(_ctx: *mut pt_regs) -> i32 {
    let mut ts: *mut u64;
    let mut cur_ts: u64;
    let mut delta: u32;
    let mut key: i32;
    let mut cpu: i32;
    let val: *mut i64;

    cpu = bpf_get_smp_processor_id();
    ts = bpf_map_lookup_elem(
        &mut my_map,
        &cpu as *const i32 as *const c_void,
    ) as *mut u64;
    if ts.is_null() {
        return 0;
    }

    cur_ts = bpf_ktime_get_ns();
    delta = log2l(cur_ts.wrapping_sub(*ts));

    if delta > (MAX_ENTRIES - 1) as u32 {
        delta = (MAX_ENTRIES - 1) as u32;
    }

    key = cpu * MAX_ENTRIES as i32 + delta as i32;
    val = bpf_map_lookup_elem(
        &mut my_lat,
        &key as *const i32 as *const c_void,
    ) as *mut i64;
    if !val.is_null() {
        // C: __sync_fetch_and_add((long *)val, 1)
        core::ptr::write_volatile(val, core::ptr::read_volatile(val).wrapping_add(1));
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// LINUX_VERSION_CODE is supplied by linux/version.h at build time.
#[no_mangle]
#[link_section = "version"]
pub static _version: u32 = LINUX_VERSION_CODE;

extern "C" {
    static LINUX_VERSION_CODE: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
