/* SPDX-License-Identifier: GPL-2.0 */
/*
 * highmem.h: virtual kernel memory mappings for high memory
 *
 * Used in CONFIG_HIGHMEM systems for memory pages which
 * are not addressable by direct kernel virtual addresses.
 *
 * Copyright (C) 2025 Loongson Technology Corporation Limited
 */

/* C header guard: _ASM_HIGHMEM_H */

/* These declarations are present only when __KERNEL__ is defined. */

/* Dependency supplied by asm/kmap_size.h. */

/* These declarations are absent when __ASSEMBLER__ is defined. */
extern "C" {
    pub static mut pkmap_page_table: *mut pte_t;

    pub fn kmap_flush_tlb(addr: ::core::ffi::c_ulong);
}

pub const ARCH_HAS_KMAP_FLUSH_TLB: bool = true;

/*
 * Right now we initialize only a single pte table. It can be extended
 * easily, subsequent pte tables have to be allocated in one physical
 * chunk of RAM.
 */
pub const LAST_PKMAP: usize = 1024;
pub const LAST_PKMAP_MASK: usize = LAST_PKMAP - 1;

#[inline(always)]
pub const fn PKMAP_NR(virt: usize) -> usize {
    (virt - PKMAP_BASE) >> PAGE_SHIFT
}

#[inline(always)]
pub const fn PKMAP_ADDR(nr: usize) -> usize {
    PKMAP_BASE + (nr << PAGE_SHIFT)
}

#[inline(always)]
pub const fn flush_cache_kmaps() {}

#[inline(always)]
pub unsafe fn arch_kmap_local_post_map(vaddr: usize, _pteval: usize) {
    local_flush_tlb_one(vaddr);
}

#[inline(always)]
pub unsafe fn arch_kmap_local_post_unmap(vaddr: usize) {
    local_flush_tlb_one(vaddr);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
