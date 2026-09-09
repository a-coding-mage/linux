/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009 Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2012 Regents of the University of California
 */

// Dependency intent: linux/mm_types.h, asm/smp.h, and asm/errata_list.h.

pub const FLUSH_TLB_MAX_SIZE: usize = usize::MAX;
pub const FLUSH_TLB_NO_ASID: usize = usize::MAX;

// CONFIG_MMU
#[inline]
pub unsafe fn get_mm_asid(mm: *mut mm_struct) -> usize {
    if !mm.is_null() {
        cntx2asid(atomic_long_read(&(*mm).context.id))
    } else {
        FLUSH_TLB_NO_ASID
    }
}

#[inline]
pub unsafe fn local_sfence_inval_ir() {
    core::arch::asm!(SFENCE_INVAL_IR!(), options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn local_sfence_w_inval() {
    core::arch::asm!(SFENCE_W_INVAL!(), options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn local_sinval_vma(vma: usize, asid: usize) {
    if asid != FLUSH_TLB_NO_ASID {
        core::arch::asm!(SINVAL_VMA!({0}, {1}), in(reg) vma, in(reg) asid);
    } else {
        core::arch::asm!(SINVAL_VMA!({0}, zero), in(reg) vma);
    }
}

#[inline]
pub unsafe fn local_flush_tlb_all() {
    core::arch::asm!("sfence.vma", options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn local_flush_tlb_all_asid(asid: usize) {
    if asid != FLUSH_TLB_NO_ASID {
        ALT_SFENCE_VMA_ASID!(asid);
    } else {
        local_flush_tlb_all();
    }
}

/* Flush one page from local TLB */
#[inline]
pub unsafe fn local_flush_tlb_page(addr: usize) {
    ALT_SFENCE_VMA_ADDR!(addr);
}

#[inline]
pub unsafe fn local_flush_tlb_page_asid(addr: usize, asid: usize) {
    if asid != FLUSH_TLB_NO_ASID {
        ALT_SFENCE_VMA_ADDR_ASID!(addr, asid);
    } else {
        local_flush_tlb_page(addr);
    }
}

unsafe extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_mm_range(mm: *mut mm_struct, start: usize, end: usize, page_size: u32);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, addr: usize);
    pub fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn flush_tlb_kernel_range(start: usize, end: usize);
    pub fn local_flush_tlb_kernel_range(start: usize, end: usize);

    // CONFIG_TRANSPARENT_HUGEPAGE
    pub fn flush_pmd_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn flush_pud_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);

    pub fn arch_tlbbatch_should_defer(mm: *mut mm_struct) -> bool;
    pub fn arch_tlbbatch_add_pending(
        batch: *mut arch_tlbflush_unmap_batch,
        mm: *mut mm_struct,
        start: usize,
        end: usize,
    );
    pub fn arch_tlbbatch_flush(batch: *mut arch_tlbflush_unmap_batch);

    pub static mut tlb_flush_all_threshold: usize;
}

// CONFIG_MMU disabled: local_flush_tlb_all() is an empty operation.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
