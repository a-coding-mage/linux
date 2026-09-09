/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: asm/mmu_context.h

/* TSB flush operations. */

pub const TLB_BATCH_NR: usize = 192;

#[repr(C)]
pub struct tlb_batch {
    pub hugepage_shift: core::ffi::c_uint,
    pub mm: *mut mm_struct,
    pub tlb_nr: core::ffi::c_ulong,
    pub vaddrs: [core::ffi::c_ulong; TLB_BATCH_NR],
}

extern "C" {
    pub fn flush_tsb_kernel_range(start: core::ffi::c_ulong, end: core::ffi::c_ulong);
    pub fn flush_tsb_user(tb: *mut tlb_batch);
    pub fn flush_tsb_user_page(
        mm: *mut mm_struct,
        vaddr: core::ffi::c_ulong,
        hugepage_shift: core::ffi::c_uint,
    );
}

/* TLB flush operations. */

#[inline]
pub unsafe fn flush_tlb_mm(_mm: *mut mm_struct) {}

#[inline]
pub unsafe fn flush_tlb_page(_vma: *mut vm_area_struct, _vmaddr: core::ffi::c_ulong) {}

#[inline]
pub unsafe fn flush_tlb_range(
    _vma: *mut vm_area_struct,
    _start: core::ffi::c_ulong,
    _end: core::ffi::c_ulong,
) {
}

extern "C" {
    pub fn flush_tlb_kernel_range(start: core::ffi::c_ulong, end: core::ffi::c_ulong);

    pub fn flush_tlb_pending();
    pub fn arch_enter_lazy_mmu_mode();
    pub fn arch_flush_lazy_mmu_mode();
    pub fn arch_leave_lazy_mmu_mode();

    /* Local cpu only.  */
    pub fn __flush_tlb_all();
    pub fn __flush_tlb_page(context: core::ffi::c_ulong, vaddr: core::ffi::c_ulong);
    pub fn __flush_tlb_kernel_range(start: core::ffi::c_ulong, end: core::ffi::c_ulong);
}

// CONFIG_SMP is a build-time condition from the original header.
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn global_flush_tlb_page(mm: *mut mm_struct, vaddr: core::ffi::c_ulong) {
    __flush_tlb_page(CTX_HWBITS((*mm).context), vaddr);
}

// CONFIG_SMP branch from the original header.
#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn smp_flush_tlb_kernel_range(start: core::ffi::c_ulong, end: core::ffi::c_ulong);
    pub fn smp_flush_tlb_page(mm: *mut mm_struct, vaddr: core::ffi::c_ulong);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
pub unsafe fn global_flush_tlb_page(mm: *mut mm_struct, vaddr: core::ffi::c_ulong) {
    smp_flush_tlb_page(mm, vaddr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
