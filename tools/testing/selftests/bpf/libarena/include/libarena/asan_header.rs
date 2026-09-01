// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

use core::ffi::c_void;

#[repr(C)]
pub struct asan_init_args {
    pub arena_all_pages: u64,
    pub arena_globals_pages: u64,
}

unsafe extern "C" {
    pub fn asan_init(args: *mut asan_init_args) -> i32;

    /* C declarations used volatile-qualified objects. Use volatile reads/writes at access sites. */
    pub static mut __asan_shadow_memory_dynamic_address: u64;
    pub static mut asan_reported: u32;
    pub static mut asan_inited: bool;
    pub static mut asan_report_once: bool;
}

/* C conditional: #ifdef __BPF__ */
#[cfg(__BPF__)]
pub const ASAN_SHADOW_SHIFT: u32 = 3;
#[cfg(__BPF__)]
pub const ASAN_SHADOW_SCALE: u64 = 1_u64 << ASAN_SHADOW_SHIFT;
#[cfg(__BPF__)]
pub const ASAN_GRANULE_MASK: u64 = (1_u64 << ASAN_SHADOW_SHIFT) - 1;

#[cfg(__BPF__)]
#[inline]
pub fn ASAN_GRANULE<T>(addr: *const T) -> i8 {
    ((addr as u64 as u32 as u64) & ASAN_GRANULE_MASK) as i8
}

/* C macro: #define __noasan __attribute__((no_sanitize("address"))) */

/* C conditional: #ifdef BPF_ARENA_ASAN */
#[cfg(all(__BPF__, BPF_ARENA_ASAN))]
#[inline]
pub unsafe fn mem_to_shadow(addr: *mut c_void) -> *mut i8 {
    /*
     * C used BPF's __arena address-space qualifier:
     *   s8 __arena *mem_to_shadow(void __arena *addr)
     */
    (((addr as u64 as u32) >> ASAN_SHADOW_SHIFT) as u64
        + unsafe { core::ptr::read_volatile(core::ptr::addr_of!(__asan_shadow_memory_dynamic_address)) })
        as *mut i8
}

#[cfg(all(__BPF__, BPF_ARENA_ASAN))]
/* C attributes: __weak __noasan */
pub unsafe extern "C" fn asan_ready() -> bool {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!(__asan_shadow_memory_dynamic_address)) != 0 }
}

#[cfg(all(__BPF__, BPF_ARENA_ASAN))]
unsafe extern "C" {
    pub fn asan_poison(addr: *mut c_void, val: i8, size: usize) -> i32;
    pub fn asan_unpoison(addr: *mut c_void, size: usize) -> i32;
    pub fn asan_shadow_set(addr: *mut c_void) -> bool;

    /*
     * Dummy calls to ensure the ASAN runtime's BTF information is present
     * in every object file when compiling the runtime and local BPF code
     * separately. The runtime calls are injected into the LLVM IR file.
     */
    pub fn __asan_store1(addr: isize);
    pub fn __asan_store1_noabort(addr: isize);
    pub fn __asan_load1(addr: isize);
    pub fn __asan_load1_noabort(addr: isize);
    pub fn __asan_report_store1(addr: isize);
    pub fn __asan_report_store1_noabort(addr: isize);
    pub fn __asan_report_load1(addr: isize);
    pub fn __asan_report_load1_noabort(addr: isize);

    pub fn __asan_store2(addr: isize);
    pub fn __asan_store2_noabort(addr: isize);
    pub fn __asan_load2(addr: isize);
    pub fn __asan_load2_noabort(addr: isize);
    pub fn __asan_report_store2(addr: isize);
    pub fn __asan_report_store2_noabort(addr: isize);
    pub fn __asan_report_load2(addr: isize);
    pub fn __asan_report_load2_noabort(addr: isize);

    pub fn __asan_store4(addr: isize);
    pub fn __asan_store4_noabort(addr: isize);
    pub fn __asan_load4(addr: isize);
    pub fn __asan_load4_noabort(addr: isize);
    pub fn __asan_report_store4(addr: isize);
    pub fn __asan_report_store4_noabort(addr: isize);
    pub fn __asan_report_load4(addr: isize);
    pub fn __asan_report_load4_noabort(addr: isize);

    pub fn __asan_store8(addr: isize);
    pub fn __asan_store8_noabort(addr: isize);
    pub fn __asan_load8(addr: isize);
    pub fn __asan_load8_noabort(addr: isize);
    pub fn __asan_report_store8(addr: isize);
    pub fn __asan_report_store8_noabort(addr: isize);
    pub fn __asan_report_load8(addr: isize);
    pub fn __asan_report_load8_noabort(addr: isize);

    pub fn __asan_storeN(addr: isize, size: isize);
    pub fn __asan_storeN_noabort(addr: isize, size: isize);
    pub fn __asan_loadN(addr: isize, size: isize);
    pub fn __asan_loadN_noabort(addr: isize, size: isize);
}

/*
 * Force LLVM to emit BTF information for the stubs,
 * because the ASAN pass in LLVM by itself doesn't.
 */
#[cfg(all(__BPF__, BPF_ARENA_ASAN))]
#[used]
static __asan_btf_anchors: [unsafe extern "C" fn(isize); 32] = [
    __asan_store1,
    __asan_store1_noabort,
    __asan_load1,
    __asan_load1_noabort,
    __asan_report_store1,
    __asan_report_store1_noabort,
    __asan_report_load1,
    __asan_report_load1_noabort,
    __asan_store2,
    __asan_store2_noabort,
    __asan_load2,
    __asan_load2_noabort,
    __asan_report_store2,
    __asan_report_store2_noabort,
    __asan_report_load2,
    __asan_report_load2_noabort,
    __asan_store4,
    __asan_store4_noabort,
    __asan_load4,
    __asan_load4_noabort,
    __asan_report_store4,
    __asan_report_store4_noabort,
    __asan_report_load4,
    __asan_report_load4_noabort,
    __asan_store8,
    __asan_store8_noabort,
    __asan_load8,
    __asan_load8_noabort,
    __asan_report_store8,
    __asan_report_store8_noabort,
    __asan_report_load8,
    __asan_report_load8_noabort,
];

/* C conditional: #else */ /* BPF_ARENA_ASAN */
#[cfg(all(__BPF__, not(BPF_ARENA_ASAN)))]
#[inline]
pub unsafe fn asan_poison(_addr: *mut c_void, _val: i8, _size: usize) -> i32 {
    0
}

#[cfg(all(__BPF__, not(BPF_ARENA_ASAN)))]
#[inline]
pub unsafe fn asan_unpoison(_addr: *mut c_void, _size: usize) -> i32 {
    0
}

#[cfg(all(__BPF__, not(BPF_ARENA_ASAN)))]
#[inline]
pub unsafe fn asan_shadow_set(_addr: *mut c_void) -> bool {
    false
}

#[cfg(all(__BPF__, not(BPF_ARENA_ASAN)))]
/* C attribute: __weak */
pub unsafe extern "C" fn asan_ready() -> bool {
    true
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
