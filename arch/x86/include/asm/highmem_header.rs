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

/* Contents of this header are conditional on __KERNEL__. */

/* declarations for highmem.c */
extern "C" {
    pub static mut highstart_pfn: ::core::ffi::c_ulong;
    pub static mut highend_pfn: ::core::ffi::c_ulong;
}

/*
 * Right now we initialize only a single pte table. It can be extended
 * easily, subsequent pte tables have to be allocated in one physical
 * chunk of RAM.
 */
/*
 * Ordering is:
 *
 * high memory on:                              high_memory off:
 *    FIXADDR_TOP                                FIXADDR_TOP
 *        fixed addresses                            fixed addresses
 *    FIXADDR_START                              FIXADDR_START
 *        temp fixed addresses/persistent kmap area VMALLOC_END
 *    PKMAP_BASE                                 temp fixed addresses/vmalloc area
 *    VMALLOC_END                                VMALLOC_START
 *        vmalloc area                           high_memory
 *    VMALLOC_START
 *    high_memory
 *
 * The temp fixed area is only used during boot for early_ioremap(), and
 * it is unused when the ioremap() is functional. vmalloc/pkmap area become
 * available after early boot so the temp fixed area is available for re-use.
 */
pub const LAST_PKMAP_MASK: ::core::ffi::c_ulong = LAST_PKMAP - 1;

#[inline]
pub const fn PKMAP_NR(virt: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    (virt - PKMAP_BASE) >> PAGE_SHIFT
}

#[inline]
pub const fn PKMAP_ADDR(nr: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    PKMAP_BASE + (nr << PAGE_SHIFT)
}

#[inline]
pub fn flush_cache_kmaps() {}

#[inline]
pub unsafe fn arch_kmap_local_post_map(
    _vaddr: ::core::ffi::c_ulong,
    _pteval: ::core::ffi::c_ulong,
) {
    arch_flush_lazy_mmu_mode();
}

#[inline]
pub unsafe fn arch_kmap_local_post_unmap(vaddr: ::core::ffi::c_ulong) {
    flush_tlb_one_kernel(vaddr);
    arch_flush_lazy_mmu_mode();
}

extern "C" {
    fn arch_flush_lazy_mmu_mode();
    fn flush_tlb_one_kernel(vaddr: ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
