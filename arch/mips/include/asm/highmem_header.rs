/* SPDX-License-Identifier: GPL-2.0 */
/*
 * highmem.h: virtual kernel memory mappings for high memory
 *
 * Used in CONFIG_HIGHMEM systems for memory pages which
 * are not addressable by direct kernel virtual addresses.
 *
 * Copyright (C) 1999 Gerhard Wichert, Siemens AG
 *                    Gerhard.Wichert@pdb.siemens.de
 *
 *
 * Redesigned the x86 32-bit VM architecture to deal with
 * up to 16 Terabyte physical memory. With current x86 CPUs
 * we now support up to 64 Gigabytes physical RAM.
 *
 * Copyright (C) 1999 Ingo Molnar <mingo@redhat.com>
 */

/* C header dependencies are supplied by other translated units. */

/* declarations for highmem.c */
extern "C" {
    static mut highstart_pfn: ::core::ffi::c_ulong;
    static mut highend_pfn: ::core::ffi::c_ulong;

    static mut pkmap_page_table: *mut pte_t;
}

/*
 * Right now we initialize only a single pte table. It can be extended
 * easily, subsequent pte tables have to be allocated in one physical
 * chunk of RAM.
 *
 * The CONFIG_* condition is retained as Rust conditional compilation.
 */
#[cfg(any(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_MIPS_HUGE_TLB_SUPPORT))]
pub const LAST_PKMAP: usize = 512;
#[cfg(not(any(CONFIG_PHYS_ADDR_T_64BIT, CONFIG_MIPS_HUGE_TLB_SUPPORT)))]
pub const LAST_PKMAP: usize = 1024;

pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

#[inline(always)]
pub const unsafe fn PKMAP_NR(virt: usize) -> usize {
    (virt - PKMAP_BASE) >> PAGE_SHIFT
}

#[inline(always)]
pub const unsafe fn PKMAP_ADDR(nr: usize) -> usize {
    PKMAP_BASE + (nr << PAGE_SHIFT)
}

/* ARCH_HAS_KMAP_FLUSH_TLB */
extern "C" {
    pub fn kmap_flush_tlb(addr: ::core::ffi::c_ulong);
}

#[inline(always)]
pub unsafe fn flush_cache_kmaps() {
    BUG_ON(cpu_has_dc_aliases);
}

#[inline(always)]
pub unsafe fn arch_kmap_local_set_pte(
    _mm: *mut ::core::ffi::c_void,
    _vaddr: usize,
    ptep: *mut pte_t,
    ptev: pte_t,
) {
    set_pte(ptep, ptev);
}

#[inline(always)]
pub unsafe fn arch_kmap_local_post_map(vaddr: usize, _pteval: usize) {
    local_flush_tlb_one(vaddr);
}

#[inline(always)]
pub unsafe fn arch_kmap_local_post_unmap(vaddr: usize) {
    local_flush_tlb_one(vaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
