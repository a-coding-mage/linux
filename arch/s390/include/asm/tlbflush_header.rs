/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translated kernel sources:
// linux::mm::{mm_struct, vm_area_struct, mm_cpumask, ...}
// linux::sched::{preempt_disable, preempt_enable, barrier}
// linux::cpufeature::machine_has_tlb_guest
// asm::{processor, machine::{IDTE_PTOA, IDTE_GUEST_ASCE, cspg}}

/*
 * Flush all TLB entries on the local CPU.
 */
#[inline]
pub unsafe fn __tlb_flush_local() {
    // C: asm volatile("ptlb" : : : "memory");
    core::arch::asm!("ptlb", options(nostack, preserves_flags));
}

/*
 * Flush TLB entries for a specific ASCE on all CPUs
 */
#[inline]
pub unsafe fn __tlb_flush_idte(asce: ::core::ffi::c_ulong) {
    let mut opt: ::core::ffi::c_ulong;

    opt = IDTE_PTOA as ::core::ffi::c_ulong;
    if machine_has_tlb_guest() {
        opt |= IDTE_GUEST_ASCE as ::core::ffi::c_ulong;
    }
    /* Global TLB flush for the mm */
    // C: asm volatile("idte 0,%1,%0" : : "a" (opt), "a" (asce) : "cc");
    core::arch::asm!(
        "idte 0, {asce}, {opt}",
        opt = in(reg) opt,
        asce = in(reg) asce,
        options(nostack)
    );
}

/*
 * Flush all TLB entries on all CPUs.
 */
#[inline]
pub unsafe fn __tlb_flush_global() {
    let mut dummy: ::core::ffi::c_ulong = 0;

    cspg(&mut dummy, 0, 0);
}

/*
 * Flush TLB entries for a specific mm on all CPUs (in case gmap is used
 * this implicates multiple ASCEs!).
 */
#[inline]
pub unsafe fn __tlb_flush_mm(mm: *mut mm_struct) {
    let mut gmap_asce: ::core::ffi::c_ulong;

    preempt_disable();
    atomic_inc(&mut (*mm).context.flush_count);
    /* Reset TLB flush mask */
    cpumask_copy(mm_cpumask(mm), &(*mm).context.cpu_attach_mask);
    barrier();
    gmap_asce = core::ptr::read_volatile(&(*mm).context.gmap_asce);
    if gmap_asce != !0 as ::core::ffi::c_ulong {
        if gmap_asce != 0 {
            __tlb_flush_idte(gmap_asce);
        }
        __tlb_flush_idte((*mm).context.asce);
    } else {
        /* Global TLB flush */
        __tlb_flush_global();
    }
    atomic_dec(&mut (*mm).context.flush_count);
    preempt_enable();
}

#[inline]
pub unsafe fn __tlb_flush_kernel() {
    __tlb_flush_idte(init_mm.context.asce);
}

#[inline]
pub unsafe fn __tlb_flush_mm_lazy(mm: *mut mm_struct) {
    spin_lock(&mut (*mm).context.lock);
    if (*mm).context.flush_mm != 0 {
        (*mm).context.flush_mm = 0;
        __tlb_flush_mm(mm);
    }
    spin_unlock(&mut (*mm).context.lock);
}

/*
 * TLB flushing:
 *  flush_tlb_all() - flushes all processes TLBs
 *  flush_tlb_mm(mm) - flushes the specified mm context TLB's
 *  flush_tlb_page(vma, vmaddr) - flushes one page
 *  flush_tlb_range(vma, start, end) - flushes a range of pages
 *  flush_tlb_kernel_range(start, end) - flushes a range of kernel pages
 */

/*
 * flush_tlb_mm goes together with ptep_set_wrprotect for the
 * copy_page_range operation and flush_tlb_range is related to
 * ptep_get_and_clear for change_protection. ptep_set_wrprotect and
 * ptep_get_and_clear do not flush the TLBs directly if the mm has
 * only one user. At the end of the update the flush_tlb_mm and
 * flush_tlb_range functions need to do the flush.
 */
#[inline]
pub fn flush_tlb_all() {}

#[inline]
pub fn flush_tlb_page<T>(_vma: *mut T, _addr: ::core::ffi::c_ulong) {}

#[inline]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    __tlb_flush_mm_lazy(mm);
}

#[inline]
pub unsafe fn flush_tlb_range(
    vma: *mut vm_area_struct,
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
    __tlb_flush_mm_lazy((*vma).vm_mm);
}

#[inline]
pub unsafe fn flush_tlb_kernel_range(
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
    __tlb_flush_kernel();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
