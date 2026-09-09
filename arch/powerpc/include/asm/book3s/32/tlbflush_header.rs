/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: linux/build_bug.h is supplied by the surrounding kernel translation.

pub const MMU_NO_CONTEXT: u32 = 0;

/*
 * TLB flushing for "classic" hash-MMU 32-bit CPUs, 6xx, 7xx, 7xxx
 */
unsafe extern "C" {
    pub fn hash__flush_tlb_mm(mm: *mut mm_struct);
    pub fn hash__flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
    pub fn hash__flush_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong);
    pub fn hash__flush_gather(tlb: *mut mmu_gather);
}

// CONFIG_SMP selects the externally supplied _tlbie implementation.
#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn _tlbie(address: c_ulong);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn _tlbie(address: c_ulong) {
    core::arch::asm!("tlbie {0}; sync", in(reg) address, options(nostack));
}

unsafe extern "C" {
    pub fn _tlbia();
    pub fn mmu_has_feature(feature: u32) -> bool;
}

// External kernel declarations supplied by the surrounding translation.
#[allow(non_camel_case_types)]
pub enum mm_struct {}
#[allow(non_camel_case_types)]
pub enum vm_area_struct {}
#[allow(non_camel_case_types)]
pub enum mmu_gather {}

unsafe extern "C" {
    pub static mut init_mm: mm_struct;
}

// External kernel constants supplied by the surrounding translation.
pub const MMU_FTR_HPTE_TABLE: u32 = 0; // Dependency placeholder: actual kernel value.
pub const PAGE_SIZE: c_ulong = 0; // Dependency placeholder: actual kernel value.
pub const PAGE_MASK: c_ulong = 0; // Dependency placeholder: actual kernel value.

pub type c_ulong = usize;

/*
 * Called at the end of a mmu_gather operation to make sure the
 * TLB flush is completely done.
 */
#[inline]
pub unsafe fn tlb_flush(tlb: *mut mmu_gather) {
    /* 603 needs to flush the whole TLB here since it doesn't use a hash table. */
    if mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        hash__flush_gather(tlb);
    } else {
        _tlbia();
    }
}

#[inline]
pub unsafe fn flush_range(mm: *mut mm_struct, mut start: c_ulong, end: c_ulong) {
    start &= PAGE_MASK;
    if mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        hash__flush_range(mm, start, end);
    } else if end.wrapping_sub(start) <= PAGE_SIZE {
        _tlbie(start);
    } else {
        _tlbia();
    }
}

#[inline]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    if mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        hash__flush_tlb_mm(mm);
    } else {
        _tlbia();
    }
}

#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    if mmu_has_feature(MMU_FTR_HPTE_TABLE) {
        hash__flush_tlb_page(vma, vmaddr);
    } else {
        _tlbie(vmaddr);
    }
}

#[inline]
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) {
    // Dependency intent: vma->vm_mm is supplied by the surrounding kernel translation.
    let mm = unsafe { *(vma as *mut *mut mm_struct) };
    flush_range(mm, start, end);
}

#[inline]
pub unsafe fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) {
    flush_range(&raw mut init_mm, start, end);
}

#[inline]
pub unsafe fn local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong) {
    flush_tlb_page(vma, vmaddr);
}

#[inline]
pub unsafe fn local_flush_tlb_page_psize(
    mm: *mut mm_struct,
    vmaddr: c_ulong,
    _psize: i32,
) {
    flush_range(mm, vmaddr, vmaddr);
}

#[inline]
pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    flush_tlb_mm(mm);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
