/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015 Regents of the University of California
 */

/* C dependency: <linux/mm.h> */

use core::ffi::{c_char, c_ulong};

extern "C" {
    pub fn test_bit(nr: c_ulong, addr: *const c_ulong) -> bool;
    pub fn clear_bit(nr: c_ulong, addr: *mut c_ulong);
    pub fn bitmap_fill(dst: *mut c_ulong, nbits: c_ulong);
    pub fn is_vmalloc_or_module_addr(addr: *const core::ffi::c_void) -> bool;
    pub fn page_folio(page: *mut page) -> *mut folio;
    pub fn flush_icache_mm(mm: *mut mm_struct, local: bool);
    pub fn local_flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
}

#[repr(C)]
pub struct folio_flags {
    pub f: c_ulong,
}

#[repr(C)]
pub struct folio {
    pub flags: folio_flags,
}

#[repr(C)]
pub struct page;

#[repr(C)]
pub struct mm_struct;

#[repr(C)]
pub struct vm_area_struct {
    pub vm_flags: c_ulong,
    pub vm_mm: *mut mm_struct,
}

pub const VM_EXEC: c_ulong = 0;
pub const PG_arch_1: c_ulong = 0;
pub const PG_dcache_clean: c_ulong = PG_arch_1;

#[inline]
pub unsafe fn local_flush_icache_all() {
    core::arch::asm!("fence.i", options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn local_flush_icache_range(_start: c_ulong, _end: c_ulong) {
    local_flush_icache_all();
}

#[inline]
pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    if test_bit(PG_dcache_clean, &(*folio).flags.f) {
        clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
}

pub const ARCH_IMPLEMENTS_FLUSH_DCACHE_PAGE: i32 = 1;

#[inline]
pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

#[inline]
pub unsafe fn flush_icache_user_page(
    vma: *mut vm_area_struct,
    _pg: *mut page,
    _addr: c_ulong,
    _len: c_ulong,
) {
    if (*vma).vm_flags & VM_EXEC != 0 {
        flush_icache_mm((*vma).vm_mm, false);
    }
}

/* C condition: CONFIG_64BIT && CONFIG_MMU. */
#[cfg(all(target_pointer_width = "64", feature = "mmu"))]
extern "C" {
    pub static mut new_valid_map_cpus: *mut c_ulong;
    pub static mut _end: c_char;
}

#[cfg(all(target_pointer_width = "64", feature = "mmu"))]
#[inline]
pub unsafe fn mark_new_valid_map() {
    /* Concurrent resets are harmless because handle_exception() emits sfence.vma. */
    bitmap_fill(new_valid_map_cpus, 0);
}

#[cfg(all(target_pointer_width = "64", feature = "mmu"))]
#[inline]
pub unsafe fn flush_cache_vmap(start: c_ulong, end: c_ulong) {
    extern "C" {
        static VMEMMAP_START: c_ulong;
        static VMEMMAP_END: c_ulong;
    }
    if is_vmalloc_or_module_addr(start as *const core::ffi::c_void)
        || (start >= VMEMMAP_START && end <= VMEMMAP_END)
    {
        mark_new_valid_map();
    }
}

/* C macro: flush_cache_vmap_early(start, end) = local_flush_tlb_kernel_range(start, end). */
#[inline]
pub unsafe fn flush_cache_vmap_early(start: c_ulong, end: c_ulong) {
    local_flush_tlb_kernel_range(start, end);
}

/* C condition: !CONFIG_SMP. */
#[cfg(not(feature = "smp"))]
#[inline]
pub unsafe fn flush_icache_all() {
    local_flush_icache_all();
}

#[cfg(not(feature = "smp"))]
#[inline]
pub unsafe fn flush_icache_mm_local(_mm: *mut mm_struct, _local: bool) {
    flush_icache_all();
}

/* C condition: CONFIG_SMP. */
#[cfg(feature = "smp")]
extern "C" {
    pub fn flush_icache_all();
    pub fn flush_icache_mm(mm: *mut mm_struct, local: bool);
}

/* RISC-V has no instruction for partial instruction-cache flushing. */
#[inline]
pub unsafe fn flush_icache_range(_start: c_ulong, _end: c_ulong) {
    flush_icache_all();
}

extern "C" {
    pub static mut riscv_cbom_block_size: u32;
    pub static mut riscv_cboz_block_size: u32;
    pub static mut riscv_cbop_block_size: u32;
    pub fn riscv_init_cbo_blocksizes();
}

/* C condition: CONFIG_RISCV_DMA_NONCOHERENT. */
#[cfg(feature = "riscv_dma_noncoherent")]
extern "C" {
    pub fn riscv_noncoherent_supported();
    pub fn riscv_set_dma_cache_alignment();
}

#[cfg(not(feature = "riscv_dma_noncoherent"))]
#[inline]
pub unsafe fn riscv_noncoherent_supported() {}

#[cfg(not(feature = "riscv_dma_noncoherent"))]
#[inline]
pub unsafe fn riscv_set_dma_cache_alignment() {}

pub const SYS_RISCV_FLUSH_ICACHE_LOCAL: c_ulong = 1;
pub const SYS_RISCV_FLUSH_ICACHE_ALL: c_ulong = SYS_RISCV_FLUSH_ICACHE_LOCAL;

/* C dependency: <asm-generic/cacheflush.h> */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
