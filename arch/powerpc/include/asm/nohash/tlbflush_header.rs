/* SPDX-License-Identifier: GPL-2.0 */

/*
 * TLB flushing:
 *
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB's
 *  - flush_tlb_page(vma, vmaddr) flushes one page
 *  - local_flush_tlb_mm(mm, full) flushes the specified mm context on the
 *    local processor
 *  - local_flush_tlb_page(vma, vmaddr) flushes one page on the local processor
 *  - flush_tlb_range(vma, start, end) flushes a range of pages
 *  - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
 */

/* TLB flushing for software loaded TLB chips. */
/* TODO (CONFIG_PPC_85xx): determine if ranges are best implemented as tlbia
 * versus specific tlbie instructions. */

use core::ffi::{c_int, c_uint, c_ulong};

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

pub const MMU_NO_CONTEXT: c_uint = c_uint::MAX;

unsafe extern "C" {
    pub fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
}

/* CONFIG_PPC_8xx supplies these as local inline operations. */
#[cfg(CONFIG_PPC_8xx)]
#[inline]
pub unsafe fn local_flush_tlb_mm(mm: *mut mm_struct) {
    /* READ_ONCE(mm->context.id); the containing type is supplied externally. */
    let pid: c_uint = unsafe { core::ptr::read_volatile(mm.cast::<c_uint>()) };
    if pid != MMU_NO_CONTEXT {
        unsafe { core::arch::asm!("sync; tlbia; isync", options(nostack, preserves_flags)); }
    }
}

#[cfg(CONFIG_PPC_8xx)]
#[inline]
pub unsafe fn local_flush_tlb_page(_vma: *mut vm_area_struct, vmaddr: c_ulong) {
    unsafe { core::arch::asm!("tlbie {0}; sync", in(reg) vmaddr, options(nostack, preserves_flags)); }
}

#[cfg(CONFIG_PPC_8xx)]
#[inline]
pub unsafe fn local_flush_tlb_page_psize(_mm: *mut mm_struct, vmaddr: c_ulong, _psize: c_int) {
    unsafe { core::arch::asm!("tlbie {0}; sync", in(reg) vmaddr, options(nostack, preserves_flags)); }
}

#[cfg(CONFIG_PPC_8xx)]
#[inline]
pub unsafe fn flush_tlb_kernel_range(mut start: c_ulong, end: c_ulong) {
    /* PAGE_MASK, PAGE_SIZE are supplied by the including environment. */
    start &= PAGE_MASK;
    if end.wrapping_sub(start) <= PAGE_SIZE {
        unsafe { core::arch::asm!("tlbie {0}; sync", in(reg) start, options(nostack, preserves_flags)); }
    } else {
        unsafe { core::arch::asm!("sync; tlbia; isync", options(nostack, preserves_flags)); }
    }
}

#[cfg(not(CONFIG_PPC_8xx))]
unsafe extern "C" {
    pub fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong);
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn local_flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
    pub fn local_flush_tlb_page_psize(mm: *mut mm_struct, vmaddr: c_ulong, psize: c_int);
    pub fn __local_flush_tlb_page(mm: *mut mm_struct, vmaddr: c_ulong, tsize: c_int, ind: c_int);
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" {
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, vmaddr: c_ulong);
    pub fn __flush_tlb_page(mm: *mut mm_struct, vmaddr: c_ulong, tsize: c_int, ind: c_int);
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    unsafe { local_flush_tlb_mm(mm) }
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, addr: c_ulong) {
    unsafe { local_flush_tlb_page(vma, addr) }
}

#[cfg(not(CONFIG_SMP))]
#[inline]
pub unsafe fn __flush_tlb_page(mm: *mut mm_struct, addr: c_ulong, p: c_int, i: c_int) {
    unsafe { __local_flush_tlb_page(mm, addr, p, i) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
