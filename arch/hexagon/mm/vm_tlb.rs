// SPDX-License-Identifier: GPL-2.0-only
/*
 * Hexagon Virtual Machine TLB functions
 *
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

/*
 * The Hexagon Virtual Machine conceals the real workings of
 * the TLB, but there are one or two functions that need to
 * be instantiated for it, differently from a native build.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

extern "C" {
    fn __vmclrmap(addr: *mut c_void, size: usize);
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct mm_context {
    pub ptbase: usize,
}

#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_mm: *mut mm_struct,
}

#[repr(C)]
pub struct task_struct {
    pub active_mm: *mut mm_struct,
}

const PAGE_SIZE: usize = 4096;

/*
 * Initial VM implementation has only one map active at a time, with
 * TLB purgings on changes.  So either we're nuking the current map,
 * or it's a no-op.  This operation is messy on true SMPs where other
 * processors must be induced to flush the copies in their local TLBs,
 * but Hexagon thread-based virtual processors share the same MMU.
 */
pub unsafe fn flush_tlb_range(vma: *mut vm_area_struct, start: usize,
                              end: usize)
{
    let mm: *mut mm_struct = (*vma).vm_mm;

    if (*mm).context.ptbase == (*(*current).active_mm).context.ptbase {
        __vmclrmap(start as *mut c_void, end.wrapping_sub(start));
    }
}

/*
 * Flush a page from the kernel virtual map - used by highmem
 */
pub unsafe fn flush_tlb_one(vaddr: usize)
{
    __vmclrmap(vaddr as *mut c_void, PAGE_SIZE);
}

/*
 * Flush all TLBs across all CPUs, virtual or real.
 * A single Hexagon core has 6 thread contexts but
 * only one TLB.
 */
pub unsafe fn tlb_flush_all()
{
    /*  should probably use that fixaddr end or whateve label  */
    __vmclrmap(0 as *mut c_void, 0xffff0000);
}

/*
 * Flush TLB entries associated with a given mm_struct mapping.
 */
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct)
{
    /* Current Virtual Machine has only one map active at a time */
    if (*(*current).active_mm).context.ptbase == (*mm).context.ptbase {
        tlb_flush_all();
    }
}

/*
 * Flush TLB state associated with a page of a vma.
 */
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, vaddr: usize)
{
    let mm: *mut mm_struct = (*vma).vm_mm;

    if (*mm).context.ptbase == (*(*current).active_mm).context.ptbase {
        __vmclrmap(vaddr as *mut c_void, PAGE_SIZE);
    }
}

/*
 * Flush TLB entries associated with a kernel address range.
 * Like flush range, but without the check on the vma->vm_mm.
 */
pub unsafe fn flush_tlb_kernel_range(start: usize, end: usize)
{
    __vmclrmap(start as *mut c_void, end.wrapping_sub(start));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
