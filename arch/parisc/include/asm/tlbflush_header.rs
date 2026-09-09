/* SPDX-License-Identifier: GPL-2.0 */

/* TLB flushing routines.... */

/* Dependencies supplied by the surrounding kernel translation unit:
 * linux/mm.h, linux/sched.h, and asm/mmu_context.h.
 */

unsafe extern "C" {
    pub fn flush_tlb_all();
    pub fn flush_tlb_all_local(arg: *mut core::ffi::c_void);

    pub fn __flush_tlb_range(sid: c_ulong, start: c_ulong, end: c_ulong) -> c_int;
}

/* C's unsigned long and int declarations are preserved here. */
type c_ulong = usize;
type c_int = i32;

#[inline(always)]
pub unsafe fn smp_flush_tlb_all() {
    flush_tlb_all();
}

/* Equivalent to flush_tlb_range(vma, start, end). */
#[macro_export]
macro_rules! flush_tlb_range {
    ($vma:expr, $start:expr, $end:expr) => {
        unsafe {
            $crate::__flush_tlb_range(
                (*(*$vma).vm_mm).context.space_id,
                $start,
                $end,
            )
        }
    };
}

#[inline(always)]
pub unsafe fn flush_tlb_kernel_range(start: c_ulong, end: c_ulong) -> c_int {
    __flush_tlb_range(0, start, end)
}

/*
 * flush_tlb_mm()
 *
 * The code to switch to a new context is NOT valid for processes
 * which play with the space id's.  Thus, we have to preserve the
 * space and just flush the entire tlb.  However, the compilers,
 * dynamic linker, etc, do not manipulate space id's, so there
 * could be a significant performance benefit in switching contexts
 * and not flushing the whole tlb.
 */

#[inline(always)]
pub unsafe fn flush_tlb_mm(mm: *mut mm_struct) {
    /* BUG_ON(mm == &init_mm); Should never happen. */
    if mm == core::ptr::addr_of_mut!(init_mm) {
        panic!("BUG_ON(mm == &init_mm)");
    }

    /* CONFIG_SMP (and the unconditional source-side #if 1) path. */
    flush_tlb_all();
}

#[inline(always)]
pub unsafe fn flush_tlb_page(vma: *mut vm_area_struct, addr: c_ulong) {
    purge_tlb_entries((*vma).vm_mm, addr);
}

/* Minimal field-bearing views required by the declarations above. */
#[repr(C)]
pub struct mm_context {
    pub space_id: c_ulong,
}

#[repr(C)]
pub struct mm_struct {
    pub context: mm_context,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_mm: *mut mm_struct,
}

unsafe extern "C" {
    static mut init_mm: mm_struct;
    fn purge_tlb_entries(mm: *mut mm_struct, addr: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
