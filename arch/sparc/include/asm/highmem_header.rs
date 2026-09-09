/* SPDX-License-Identifier: GPL-2.0 */
/*
 * highmem.h: virtual kernel memory mappings for high memory
 *
 * Used in CONFIG_HIGHMEM systems for memory pages which
 * are not addressable by direct kernel virtual addresses.
 *
 * Copyright (C) 1999 Gerhard Wichert, Siemens AG
 *		      Gerhard.Wichert@pdb.siemens.de
 *
 *
 * Redesigned the x86 32-bit VM architecture to deal with 
 * up to 16 Terrabyte physical memory. With current x86 CPUs
 * we now support up to 64 Gigabytes physical RAM.
 *
 * Copyright (C) 1999 Ingo Molnar <mingo@redhat.com>
 */

/* C header dependencies: linux/interrupt.h, linux/pgtable.h,
 * asm/vaddrs.h, and asm/pgtsrmmu.h.  Their declarations are supplied
 * by the surrounding translation unit.
 */

/* The following declarations and macros are present only when __KERNEL__
 * is defined in the source build.
 */

/* declarations for highmem.c */
extern "C" {
    static mut highstart_pfn: ::core::ffi::c_ulong;
    static mut highend_pfn: ::core::ffi::c_ulong;
    static mut pkmap_page_table: *mut pte_t;
}

macro_rules! kmap_prot {
    () => {
        __pgprot(SRMMU_ET_PTE | SRMMU_PRIV | SRMMU_CACHE)
    };
}

/*
 * Right now we initialize only a single pte table. It can be extended
 * easily, subsequent pte tables have to be allocated in one physical
 * chunk of RAM.  Currently the simplest way to do this is to align the
 * pkmap region on a pagetable boundary (4MB).
 */
pub const LAST_PKMAP: usize = 1024;
pub const PKMAP_SIZE: usize = LAST_PKMAP << PAGE_SHIFT;
pub const PKMAP_BASE: usize = PMD_ALIGN(
    SRMMU_NOCACHE_VADDR + (SRMMU_MAX_NOCACHE_PAGES << PAGE_SHIFT),
);

pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

macro_rules! PKMAP_NR {
    ($virt:expr) => {
        (($virt - PKMAP_BASE) >> PAGE_SHIFT)
    };
}

macro_rules! PKMAP_ADDR {
    ($nr:expr) => {
        (PKMAP_BASE + (($nr) << PAGE_SHIFT))
    };
}

macro_rules! PKMAP_END {
    () => {
        PKMAP_ADDR!(LAST_PKMAP)
    };
}

macro_rules! flush_cache_kmaps {
    () => {
        flush_cache_all()
    };
}

/* FIXME: Use __flush_*_one(vaddr) instead of flush_*_all() -- Anton */
macro_rules! arch_kmap_local_pre_map {
    ($vaddr:expr, $pteval:expr) => {
        flush_cache_all()
    };
}

macro_rules! arch_kmap_local_pre_unmap {
    ($vaddr:expr) => {
        flush_cache_all()
    };
}

macro_rules! arch_kmap_local_post_map {
    ($vaddr:expr, $pteval:expr) => {
        flush_tlb_all()
    };
}

macro_rules! arch_kmap_local_post_unmap {
    ($vaddr:expr) => {
        flush_tlb_all()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
