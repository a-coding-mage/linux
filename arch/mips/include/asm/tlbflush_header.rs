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

#[repr(C)]
pub struct vm_area_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn local_flush_tlb_all();
    pub fn local_flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn local_flush_tlb_kernel_range(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn local_flush_tlb_page(
        vma: *mut vm_area_struct,
        page: ::core::ffi::c_ulong,
    );
    pub fn local_flush_tlb_one(vaddr: ::core::ffi::c_ulong);

    /* CONFIG_SMP declarations. */
    #[cfg(feature = "CONFIG_SMP")]
    pub fn flush_tlb_all();
    #[cfg(feature = "CONFIG_SMP")]
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    #[cfg(feature = "CONFIG_SMP")]
    pub fn flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    #[cfg(feature = "CONFIG_SMP")]
    pub fn flush_tlb_kernel_range(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    #[cfg(feature = "CONFIG_SMP")]
    pub fn flush_tlb_page(vma: *mut vm_area_struct, page: ::core::ffi::c_ulong);
    #[cfg(feature = "CONFIG_SMP")]
    pub fn flush_tlb_one(vaddr: ::core::ffi::c_ulong);

    /* Supplied by asm/mmu_context.h. */
    pub fn drop_mmu_context(mm: *mut mm_struct);
}

/* CONFIG_SMP is a build-time condition; these wrappers preserve the !CONFIG_SMP macros. */
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_tlb_all() {
    local_flush_tlb_all();
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    drop_mmu_context(mm);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_tlb_range(
    vma: *mut vm_area_struct,
    vmaddr: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    local_flush_tlb_range(vma, vmaddr, end);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_tlb_kernel_range(
    vmaddr: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    local_flush_tlb_kernel_range(vmaddr, end);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, page: ::core::ffi::c_ulong) {
    local_flush_tlb_page(vma, page);
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn flush_tlb_one(vaddr: ::core::ffi::c_ulong) {
    local_flush_tlb_one(vaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
