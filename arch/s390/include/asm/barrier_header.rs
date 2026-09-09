/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 1999, 2009
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Dependencies supplied by asm/alternative.h, asm/march.h, and
// asm-generic/barrier.h remain external to this translation.

/*
 * Force strict CPU ordering.
 * And yes, this is required on UP too when we're talking
 * to devices.
 */
#[inline(always)]
pub unsafe fn bcr_serialize() {
    // ALTERNATIVE("bcr 15,0", "bcr 14,0", ALT_FACILITY(45))
    core::arch::asm!("bcr 15,0", options(nostack));
}

// #define __mb() bcr_serialize()
#[inline(always)]
pub unsafe fn __mb() { bcr_serialize(); }

// #define __rmb() barrier()
#[inline(always)]
pub unsafe fn __rmb() { barrier(); }

// #define __wmb() barrier()
#[inline(always)]
pub unsafe fn __wmb() { barrier(); }

// #define __dma_rmb() __mb()
#[inline(always)]
pub unsafe fn __dma_rmb() { __mb(); }

// #define __dma_wmb() __mb()
#[inline(always)]
pub unsafe fn __dma_wmb() { __mb(); }

// #define __smp_mb() __mb()
#[inline(always)]
pub unsafe fn __smp_mb() { __mb(); }

// #define __smp_rmb() __rmb()
#[inline(always)]
pub unsafe fn __smp_rmb() { __rmb(); }

// #define __smp_wmb() __wmb()
#[inline(always)]
pub unsafe fn __smp_wmb() { __wmb(); }

// __smp_store_release(p, v)
#[macro_export]
macro_rules! __smp_store_release {
    ($p:expr, $v:expr) => {{
        compiletime_assert_atomic_type!(*$p);
        barrier();
        WRITE_ONCE!(*$p, $v);
    }};
}

// __smp_load_acquire(p)
#[macro_export]
macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        let ___p1 = READ_ONCE!(*$p);
        compiletime_assert_atomic_type!(*$p);
        barrier();
        ___p1
    }};
}

// #define __smp_mb__before_atomic() barrier()
#[inline(always)]
pub unsafe fn __smp_mb__before_atomic() { barrier(); }

// #define __smp_mb__after_atomic() barrier()
#[inline(always)]
pub unsafe fn __smp_mb__after_atomic() { barrier(); }

/**
 * array_index_mask_nospec - generate a mask for array_idx() that is
 * ~0UL when the bounds check succeeds and 0 otherwise
 * @index: array element index
 * @size: number of elements in array
 */
#[inline(always)]
pub unsafe fn array_index_mask_nospec(index: usize, size: usize) -> usize {
    let mut mask: usize;

    if size > 0 {
        core::arch::asm!(
            "clgr {index},{size_minus_one}\n\tslbgr {mask},{mask}",
            mask = out(reg) mask,
            size_minus_one = in(reg) size - 1,
            index = in(reg) index,
            options(nostack)
        );
        return mask;
    }
    core::arch::asm!(
        "clgr {index},{size}\n\tslbgr {mask},{mask}",
        mask = out(reg) mask,
        size = in(reg) size,
        index = in(reg) index,
        options(nostack)
    );
    !mask
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
