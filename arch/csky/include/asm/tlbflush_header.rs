/* SPDX-License-Identifier: GPL-2.0 */

/*
 * TLB flushing:
 *
 *  - flush_tlb_all() flushes all processes TLB entries
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB entries
 *  - flush_tlb_page(vma, vmaddr) flushes one page
 *  - flush_tlb_range(vma, start, end) flushes a range of pages
 *  - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
 */

/* External types supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, page: usize);
    pub fn flush_tlb_range(vma: *mut vm_area_struct, start: usize, end: usize);
    pub fn flush_tlb_kernel_range(start: usize, end: usize);

    pub fn flush_tlb_one(vaddr: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
