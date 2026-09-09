/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/cacheflush.h
 *
 * Copyright (C) 1999-2002 Russell King.
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the corresponding kernel headers are intentionally
// left external to this translation.

/* This flag indicates that the page pointed to by a pte is clean. */
pub const PG_dcache_clean: _ = PG_arch_1;

extern "C" {
    pub fn caches_clean_inval_pou(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn icache_inval_pou(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_clean_inval_poc(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_inval_poc(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_clean_poc(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_inval_poc_nosync(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_clean_poc_nosync(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_clean_pop(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn dcache_clean_pou(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);
    pub fn caches_clean_inval_user_pou(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_long;
    pub fn sync_icache_aliases(start: ::core::ffi::c_ulong, end: ::core::ffi::c_ulong);

    pub fn copy_to_user_page(
        vma: *mut vm_area_struct,
        page: *mut page,
        vaddr: ::core::ffi::c_ulong,
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        len: ::core::ffi::c_ulong,
    );
    pub fn flush_dcache_page(page: *mut page);
    pub fn flush_dcache_folio(folio: *mut folio);
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline(always)]
pub unsafe fn flush_icache_range(
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    caches_clean_inval_pou(start, end);

    /*
     * KGDB performs cache maintenance with interrupts disabled.  KGDB uses
     * IPIs during patching, so no additional IPIs are needed here.
     */
    if in_dbg_master() {
        return;
    }

    kick_all_cpus_sync();
}

#[inline(always)]
pub unsafe fn icache_inval_all_pou() {
    if alternative_has_cap_unlikely(ARM64_HAS_CACHE_DIC) {
        return;
    }

    ::core::arch::asm!("ic ialluis");
    dsb(ish);
}

// The following names are provided by the included kernel headers.
extern "C" {
    fn in_dbg_master() -> bool;
    fn kick_all_cpus_sync();
    fn alternative_has_cap_unlikely(cap: _ ) -> bool;
    fn dsb(domain: _);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
