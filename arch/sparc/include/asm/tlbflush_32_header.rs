/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations from <asm/cachetlb_32.h> are supplied externally.

macro_rules! flush_tlb_all {
    () => {
        unsafe { (*sparc32_cachetlb_ops).tlb_all() }
    };
}

macro_rules! flush_tlb_mm {
    ($mm:expr) => {
        unsafe { (*sparc32_cachetlb_ops).tlb_mm($mm) }
    };
}

macro_rules! flush_tlb_range {
    ($vma:expr, $start:expr, $end:expr) => {
        unsafe { (*sparc32_cachetlb_ops).tlb_range($vma, $start, $end) }
    };
}

macro_rules! flush_tlb_page {
    ($vma:expr, $addr:expr) => {
        unsafe { (*sparc32_cachetlb_ops).tlb_page($vma, $addr) }
    };
}

/*
 * This is a kludge, until I know better. --zaitcev XXX
 */
#[inline]
unsafe fn flush_tlb_kernel_range(start: ::core::ffi::c_ulong,
                                 end: ::core::ffi::c_ulong)
{
    let _ = (start, end);
    flush_tlb_all!();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
