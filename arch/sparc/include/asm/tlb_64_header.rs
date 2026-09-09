/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/swap.h, linux/pagemap.h, asm/tlbflush.h, asm/mmu_context.h,
// and asm-generic/tlb.h.

use core::ffi::c_ulong;

#[repr(C)]
pub struct mm_struct;

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    pub fn smp_flush_tlb_pending(
        mm: *mut mm_struct,
        start: c_ulong,
        end: *mut c_ulong,
    );

    pub fn smp_flush_tlb_mm(mm: *mut mm_struct);
}

#[cfg(feature = "CONFIG_SMP")]
#[inline(always)]
pub unsafe fn do_flush_tlb_mm(mm: *mut mm_struct) {
    smp_flush_tlb_mm(mm);
}

// Non-SMP configuration preserves the original direct call:
// __flush_tlb_mm(CTX_HWBITS(mm->context), SECONDARY_CONTEXT)
#[cfg(not(feature = "CONFIG_SMP"))]
#[inline(always)]
pub unsafe fn do_flush_tlb_mm(mm: *mut mm_struct) {
    // TODO: translate mm->context through the supplied mm_struct definition.
    let _ = mm;
    unimplemented!("__flush_tlb_mm(CTX_HWBITS(mm->context), SECONDARY_CONTEXT)");
}

extern "C" {
    pub fn __flush_tlb_pending(
        start: c_ulong,
        end: c_ulong,
        pages: *mut c_ulong,
    );
    pub fn flush_tlb_pending();
    pub fn __flush_tlb_mm(context: c_ulong, secondary_context: c_ulong);
}

// #define tlb_flush(tlb) flush_tlb_pending()
#[macro_export]
macro_rules! tlb_flush {
    ($tlb:expr) => {{
        let _ = &$tlb;
        unsafe { $crate::flush_tlb_pending() }
    }};
}

/*
 * SPARC64's hardware TLB fill does not use the Linux page-tables
 * and therefore we don't need a TLBI when freeing page-table pages.
 */

#[cfg(feature = "CONFIG_MMU_GATHER_RCU_TABLE_FREE")]
#[inline(always)]
pub const fn tlb_needs_table_invalidate() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
