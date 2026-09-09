/* SPDX-License-Identifier: GPL-2.0 */

/*
 * TLB flushing:
 *
 *  - flush_tlb_all() flushes all processes TLBs
 *  - flush_tlb_mm(mm) flushes the specified mm context TLB's
 *  - flush_tlb_page(vma, vmaddr) flushes one page
 *  - flush_tlb_range(vma, start, end) flushes a range of pages
 *  - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
 */

#[repr(C)]
pub struct mm_struct;

#[repr(C)]
pub struct vm_area_struct;

unsafe extern "C" {
    pub fn local_flush_tlb_all();
    pub fn local_flush_tlb_mm(mm: *mut mm_struct);
    pub fn local_flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn local_flush_tlb_page(
        vma: *mut vm_area_struct,
        page: ::core::ffi::c_ulong,
    );
    pub fn local_flush_tlb_kernel_range(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn local_flush_tlb_one(
        asid: ::core::ffi::c_ulong,
        page: ::core::ffi::c_ulong,
    );

    pub fn __flush_tlb_global();
}

/* CONFIG_SMP build-time condition. */
#[cfg(feature = "CONFIG_SMP")]
unsafe extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_range(
        vma: *mut vm_area_struct,
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn flush_tlb_page(
        vma: *mut vm_area_struct,
        page: ::core::ffi::c_ulong,
    );
    pub fn flush_tlb_kernel_range(
        start: ::core::ffi::c_ulong,
        end: ::core::ffi::c_ulong,
    );
    pub fn flush_tlb_one(
        asid: ::core::ffi::c_ulong,
        page: ::core::ffi::c_ulong,
    );
}

/* Non-SMP build: the C macros forward directly to the local implementations. */
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_all() {
    unsafe { local_flush_tlb_all() }
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    unsafe { local_flush_tlb_mm(mm) }
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, page: ::core::ffi::c_ulong) {
    unsafe { local_flush_tlb_page(vma, page) }
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_one(asid: ::core::ffi::c_ulong, page: ::core::ffi::c_ulong) {
    unsafe { local_flush_tlb_one(asid, page) }
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_range(
    vma: *mut vm_area_struct,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    unsafe { local_flush_tlb_range(vma, start, end) }
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_kernel_range(
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    unsafe { local_flush_tlb_kernel_range(start, end) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
