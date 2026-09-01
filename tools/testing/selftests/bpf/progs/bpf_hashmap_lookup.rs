// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Isovalent */

/*
 * C dependencies removed from executable Rust:
 * - "vmlinux.h"
 * - <bpf/bpf_helpers.h>
 * - "bpf_misc.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;
type __u32 = core::ffi::c_uint;

const BPF_MAP_TYPE_HASH: u32 = 1;

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [core::ffi::c_char; 4] = [b'G' as core::ffi::c_char, b'P' as core::ffi::c_char, b'L' as core::ffi::c_char, 0];

#[repr(C)]
pub struct hash_map_bench_map {
    /*
     * Direct translation of __uint(type, BPF_MAP_TYPE_HASH), which is
     * conventionally a pointer to an array sized by the map type value.
     */
    pub type_: *mut [core::ffi::c_int; BPF_MAP_TYPE_HASH as usize],
}

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".maps")]
pub static mut hash_map_bench: hash_map_bench_map = hash_map_bench_map {
    type_: core::ptr::null_mut(),
};

/* The number of slots to store times */
const NR_SLOTS: usize = 32;
const NR_CPUS: usize = 256;
const CPU_MASK: u32 = (NR_CPUS as u32) - 1;

/* Configured by userspace */
#[unsafe(no_mangle)]
pub static mut nr_entries: u64 = 0;
#[unsafe(no_mangle)]
pub static mut nr_loops: u64 = 0;
#[repr(align(8))]
pub struct aligned_key(pub [u32; NR_CPUS]);
#[unsafe(no_mangle)]
pub static mut key: aligned_key = aligned_key([0; NR_CPUS]);

/* Filled by us */
#[repr(align(256))]
pub struct aligned_percpu_times_index(pub [u64; NR_CPUS]);
#[unsafe(no_mangle)]
pub static mut percpu_times_index: aligned_percpu_times_index = aligned_percpu_times_index([0; NR_CPUS]);
#[repr(align(256))]
pub struct aligned_percpu_times(pub [[u64; NR_SLOTS]; NR_CPUS]);
#[unsafe(no_mangle)]
pub static mut percpu_times: aligned_percpu_times = aligned_percpu_times([[0; NR_SLOTS]; NR_CPUS]);

unsafe extern "C" {
    fn bpf_map_lookup_elem(map: *mut core::ffi::c_void, key: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn bpf_loop(
        nr_loops: u64,
        callback_fn: unsafe extern "C" fn(__u32, *mut u32) -> core::ffi::c_int,
        callback_ctx: *mut core::ffi::c_void,
        flags: u64,
    ) -> core::ffi::c_int;
    fn bpf_get_smp_processor_id() -> u32;
    fn bpf_ktime_get_ns() -> u64;
}

#[inline(always)]
unsafe fn patch_key(i: u32) {
    #[cfg(target_endian = "little")]
    {
        unsafe {
            key.0[0] = i.wrapping_add(1);
        }
    }
    #[cfg(not(target_endian = "little"))]
    {
        unsafe {
            key.0[0] = i.wrapping_add(1).swap_bytes();
        }
    }
    /* the rest of key is random and is configured by userspace */
}

unsafe extern "C" fn lookup_callback(index: __u32, unused: *mut u32) -> core::ffi::c_int {
    let _ = unused;
    unsafe {
        patch_key(index);
        if !bpf_map_lookup_elem(
            core::ptr::addr_of_mut!(hash_map_bench) as *mut core::ffi::c_void,
            core::ptr::addr_of!(key) as *const core::ffi::c_void,
        )
        .is_null()
        {
            0
        } else {
            1
        }
    }
}

unsafe extern "C" fn loop_lookup_callback(index: __u32, unused: *mut u32) -> core::ffi::c_int {
    let _ = index;
    let _ = unused;
    unsafe {
        if bpf_loop(nr_entries, lookup_callback, core::ptr::null_mut(), 0) != 0 {
            0
        } else {
            1
        }
    }
}

/*
 * Original section expression was:
 * SEC("fentry/" SYS_PREFIX "sys_getpgid")
 * SYS_PREFIX is supplied by bpf_misc.h outside this isolated file.
 */
#[unsafe(no_mangle)]
#[unsafe(link_section = "fentry/sys_getpgid")]
pub unsafe extern "C" fn benchmark(ctx: *mut core::ffi::c_void) -> core::ffi::c_int {
    let _ = ctx;
    unsafe {
        let cpu: u32 = bpf_get_smp_processor_id();
        let times_index: u32;
        let start_time: u64;

        times_index = (percpu_times_index.0[(cpu & CPU_MASK) as usize] % NR_SLOTS as u64) as u32;
        start_time = bpf_ktime_get_ns();
        bpf_loop(nr_loops, loop_lookup_callback, core::ptr::null_mut(), 0);
        percpu_times.0[(cpu & CPU_MASK) as usize][times_index as usize] =
            bpf_ktime_get_ns().wrapping_sub(start_time);
        percpu_times_index.0[(cpu & CPU_MASK) as usize] =
            percpu_times_index.0[(cpu & CPU_MASK) as usize].wrapping_add(1);
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
