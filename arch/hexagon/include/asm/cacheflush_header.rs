/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Cache flush operations for the Hexagon architecture
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/* Dependency: linux/mm_types.h */

/* Cache flushing:
 *
 *  - flush_cache_all() flushes entire cache
 *  - flush_cache_mm(mm) flushes the specified mm context's cache lines
 *  - flush_cache_page(mm, vmaddr, pfn) flushes a single page
 *  - flush_cache_range(vma, start, end) flushes a range of pages
 *  - flush_icache_range(start, end) flush a range of instructions
 *  - flush_dcache_page(pg) flushes(wback&invalidates) a page for dcache
 *  - flush_icache_pages(vma, pg, nr) flushes(invalidates) nr pages for icache
 *
 *  Need to doublecheck which one is really needed for ptrace stuff to work.
 */
pub const LINESIZE: usize = 32;
pub const LINEBITS: usize = 5;

/* Flush Dcache range through current map. */
extern "C" {
    pub fn flush_dcache_range(start: libc::c_ulong, end: libc::c_ulong);
}

/* Flush Icache range through current map. */
extern "C" {
    pub fn flush_icache_range(start: libc::c_ulong, end: libc::c_ulong);
}

/*
 * Memory-management related flushes are there to ensure in non-physically
 * indexed cache schemes that stale lines belonging to a given ASID aren't
 * in the cache to confuse things.  The prototype Hexagon Virtual Machine
 * only uses a single ASID for all user-mode maps, which should
 * mean that they aren't necessary.  A brute-force, flush-everything
 * implementation, with the name xxxxx_hexagon() is present
 * in arch/hexagon/mm/cache.c, but let's not wire it up until we know
 * it is needed.
 */
extern "C" {
    pub fn flush_cache_all_hexagon();
}

/* Opaque types supplied by linux/mm_types.h. */
pub enum vm_fault {}
pub enum vm_area_struct {}
pub enum page {}
pub enum pte_t {}

/*
 * This may or may not ever have to be non-null, depending on the
 * virtual machine MMU.  For a native kernel, it's definitiely  a no-op
 *
 * This is also the place where deferred cache coherency stuff seems
 * to happen, classically...  but instead we do it like ia64 and
 * clean the cache when the PTE is set.
 *
 */
#[inline]
pub unsafe fn update_mmu_cache_range(
    _vmf: *mut vm_fault,
    _vma: *mut vm_area_struct,
    _address: libc::c_ulong,
    _ptep: *mut pte_t,
    _nr: libc::c_uint,
) {
    /* generic_ptrace_pokedata doesn't wind up here, does it? */
}

#[inline]
pub unsafe fn update_mmu_cache(
    vma: *mut vm_area_struct,
    addr: libc::c_ulong,
    ptep: *mut pte_t,
) {
    update_mmu_cache_range(core::ptr::null_mut(), vma, addr, ptep, 1);
}

extern "C" {
    pub fn copy_to_user_page(
        vma: *mut vm_area_struct,
        page: *mut page,
        vaddr: libc::c_ulong,
        dst: *mut libc::c_void,
        src: *mut libc::c_void,
        len: libc::c_int,
    );
}

#[inline]
pub unsafe fn copy_from_user_page(
    _vma: *mut vm_area_struct,
    _page: *mut page,
    _vaddr: libc::c_ulong,
    dst: *mut libc::c_void,
    src: *const libc::c_void,
    len: usize,
) {
    core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, len);
}

extern "C" {
    pub fn hexagon_inv_dcache_range(start: libc::c_ulong, end: libc::c_ulong);
    pub fn hexagon_clean_dcache_range(start: libc::c_ulong, end: libc::c_ulong);
}

/* Dependency: asm-generic/cacheflush.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
