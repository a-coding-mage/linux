/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the surrounding kernel translation. */
use core::ffi::c_ulong;

extern "C" {
    fn __load_new_mm_context(mm: *mut mm_struct);
    fn tbi(mode: c_ulong, addr: c_ulong);
    fn tbia();
    fn smp_processor_id() -> usize;
}

/* Opaque types supplied by the surrounding kernel translation. */
#[repr(C)]
pub struct mm_struct {
    pub context: *mut c_ulong,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_flags: c_ulong,
    pub vm_mm: *mut mm_struct,
}

/* VM_EXEC is supplied by linux/mm.h. */
pub const VM_EXEC: c_ulong = 0;

extern "C" {
    static mut current: *mut task_struct;
}

#[repr(C)]
pub struct task_struct {
    pub active_mm: *mut mm_struct,
}

#[inline]
pub unsafe fn ev5_flush_tlb_current(mm: *mut mm_struct) {
    __load_new_mm_context(mm);
}

/* Flush just one page in the current TLB set.  We need to be very
   careful about the icache here, there is no way to invalidate a
   specific icache page.  */
#[inline]
pub unsafe fn ev5_flush_tlb_current_page(
    mm: *mut mm_struct,
    vma: *mut vm_area_struct,
    addr: c_ulong,
) {
    if (*vma).vm_flags & VM_EXEC != 0 {
        __load_new_mm_context(mm);
    } else {
        tbi(2, addr);
    }
}

pub use ev5_flush_tlb_current as flush_tlb_current;
pub use ev5_flush_tlb_current_page as flush_tlb_current_page;

/* Flush current user mapping.  */
#[inline]
pub unsafe fn flush_tlb() {
    flush_tlb_current((*current).active_mm);
}

/* Flush someone else's user mapping.  */
#[inline]
pub unsafe fn flush_tlb_other(mm: *mut mm_struct) {
    let mmc = (*mm).context.add(smp_processor_id());
    /* Check it's not zero first to avoid cacheline ping pong
       when possible.  */
    if core::ptr::read_volatile(mmc) != 0 {
        core::ptr::write_volatile(mmc, 0);
    }
}

/* CONFIG_SMP selects external implementations for these operations. */
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_all() {
    /* Flush everything (kernel mapping may also have changed
       due to vmalloc/vfree).  */
    tbia();
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    if mm == (*current).active_mm {
        flush_tlb_current(mm);
    } else {
        flush_tlb_other(mm);
    }
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, addr: c_ulong) {
    let mm = (*vma).vm_mm;
    if mm == (*current).active_mm {
        flush_tlb_current_page(mm, vma, addr);
    } else {
        flush_tlb_other(mm);
    }
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
pub unsafe fn flush_tlb_range(
    vma: *mut vm_area_struct,
    _start: c_ulong,
    _end: c_ulong,
) {
    flush_tlb_mm((*vma).vm_mm);
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_mm(mm: *mut mm_struct);
    pub fn flush_tlb_page(vma: *mut vm_area_struct, addr: c_ulong);
    pub fn flush_tlb_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong);
}

#[inline]
pub unsafe fn flush_tlb_kernel_range(_start: c_ulong, _end: c_ulong) {
    flush_tlb_all();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
