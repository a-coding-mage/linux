/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the Linux MM, CPU information, cache operations,
// and generic cache-flush interfaces are intentionally left external.

#[inline]
pub unsafe fn cache_present(cdesc: *const crate::cache_desc) -> bool {
    ((*cdesc).flags & CACHE_PRESENT) != 0
}

#[inline]
pub unsafe fn cache_private(cdesc: *const crate::cache_desc) -> bool {
    ((*cdesc).flags & CACHE_PRIVATE) != 0
}

#[inline]
pub unsafe fn cache_inclusive(cdesc: *const crate::cache_desc) -> bool {
    ((*cdesc).flags & CACHE_INCLUSIVE) != 0
}

#[inline]
pub unsafe fn cpu_last_level_cache_line_size() -> ::core::ffi::c_uint {
    let cache_present: ::core::ffi::c_int = boot_cpu_data.cache_leaves_present;
    boot_cpu_data.cache_leaves[(cache_present - 1) as usize].linesz
}

pub unsafe extern "C" {
    pub fn __flush_cache_all();
}

/*
 * LoongArch maintains ICache/DCache coherency by hardware,
 * we just need "ibar" to avoid instruction hazard here.
 */
#[inline]
pub unsafe fn local_flush_icache_all() {
    core::arch::asm!("ibar\t0", options(nostack));
}

#[inline]
pub unsafe fn local_flush_icache_range(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {
    core::arch::asm!("ibar\t0", options(nostack));
}

pub use local_flush_icache_all as flush_icache_all;
pub use local_flush_icache_range as flush_icache_range;
pub use local_flush_icache_range as flush_icache_user_range;

#[inline]
pub fn flush_cache_all() {}

#[inline]
pub fn flush_cache_mm(_mm: *mut crate::mm_struct) {}

#[inline]
pub fn flush_cache_dup_mm(_mm: *mut crate::mm_struct) {}

#[inline]
pub fn flush_cache_range(_vma: *mut crate::vm_area_struct, _start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {}

#[inline]
pub fn flush_cache_page(_vma: *mut crate::vm_area_struct, _vmaddr: ::core::ffi::c_ulong, _pfn: ::core::ffi::c_ulong) {}

#[inline]
pub fn flush_cache_vmap(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {}

#[inline]
pub fn flush_cache_vunmap(_start: ::core::ffi::c_ulong, _end: ::core::ffi::c_ulong) {}

#[inline]
pub fn flush_icache_user_page(_vma: *mut crate::vm_area_struct, _page: *mut crate::page, _addr: ::core::ffi::c_ulong, _len: ::core::ffi::c_ulong) {}

#[inline]
pub fn flush_dcache_mmap_lock(_mapping: *mut crate::address_space) {}

#[inline]
pub fn flush_dcache_mmap_unlock(_mapping: *mut crate::address_space) {}

#[inline]
pub unsafe fn cache_op(op: ::core::ffi::c_int, addr: ::core::ffi::c_ulong) {
    core::arch::asm!("cacop {0}, {1}", in(reg) op, in(reg) addr, options(nostack));
}

#[inline]
pub unsafe fn flush_cache_line(leaf: ::core::ffi::c_int, addr: ::core::ffi::c_ulong) {
    match leaf {
        Cache_LEAF0 => cache_op(Index_Writeback_Inv_LEAF0, addr),
        Cache_LEAF1 => cache_op(Index_Writeback_Inv_LEAF1, addr),
        Cache_LEAF2 => cache_op(Index_Writeback_Inv_LEAF2, addr),
        Cache_LEAF3 => cache_op(Index_Writeback_Inv_LEAF3, addr),
        Cache_LEAF4 => cache_op(Index_Writeback_Inv_LEAF4, addr),
        Cache_LEAF5 => cache_op(Index_Writeback_Inv_LEAF5, addr),
        _ => {}
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
