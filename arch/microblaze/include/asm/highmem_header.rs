/* SPDX-License-Identifier: GPL-2.0 */
/*
 * highmem.h: virtual kernel memory mappings for high memory
 *
 * Used in CONFIG_HIGHMEM systems for memory pages which
 * are not addressable by direct kernel virtual addresses.
 *
 * Copyright (C) 1999 Gerhard Wichert, Siemens AG
 *                      Gerhard.Wichert@pdb.siemens.de
 *
 * Redesigned the x86 32-bit VM architecture to deal with
 * up to 16 Terabyte physical memory. With current x86 CPUs
 * we now support up to 64 Gigabytes physical RAM.
 *
 * Copyright (C) 1999 Ingo Molnar <mingo@redhat.com>
 */

// The original declarations are active only when __KERNEL__ is defined.

extern "C" {
    pub static mut pkmap_page_table: *mut pte_t;
}

/*
 * Right now we initialize only a single pte table. It can be extended
 * easily, subsequent pte tables have to be allocated in one physical
 * chunk of RAM.
 */
/*
 * We use one full pte table with 4K pages. And with 16K/64K/256K pages pte
 * table covers enough memory (32MB/512MB/2GB resp.), so that both FIXMAP
 * and PKMAP can be placed in a single pte table. We use 512 pages for PKMAP
 * in case of 16K/64K/256K page sizes.
 */

pub const PKMAP_ORDER: usize = PTE_SHIFT;
pub const LAST_PKMAP: usize = 1usize << PKMAP_ORDER;

pub const PKMAP_BASE: usize =
    (FIXADDR_START - PAGE_SIZE * (LAST_PKMAP + 1)) & PMD_MASK;

pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

#[inline]
pub const fn PKMAP_NR(virt: usize) -> usize {
    (virt - PKMAP_BASE) >> PAGE_SHIFT
}

#[inline]
pub const fn PKMAP_ADDR(nr: usize) -> usize {
    PKMAP_BASE + (nr << PAGE_SHIFT)
}

#[inline]
pub unsafe fn flush_cache_kmaps() {
    flush_icache();
    flush_dcache();
}

#[inline]
pub unsafe fn arch_kmap_local_post_map(vaddr: usize, _pteval: usize) {
    local_flush_tlb_page(core::ptr::null_mut(), vaddr);
}

#[inline]
pub unsafe fn arch_kmap_local_post_unmap(vaddr: usize) {
    local_flush_tlb_page(core::ptr::null_mut(), vaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
