/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/*
 * Hint encoding:
 *
 * Bit4: ordering or completion (0: completion, 1: ordering)
 * Bit3: barrier for previous read (0: true, 1: false)
 * Bit2: barrier for previous write (0: true, 1: false)
 * Bit1: barrier for succeeding read (0: true, 1: false)
 * Bit0: barrier for succeeding write (0: true, 1: false)
 *
 * Hint 0x700: barrier for "read after read" from the same address
 */

#[inline(always)]
pub unsafe fn dbar<const HINT: i32>() {
    core::arch::asm!("dbar {0}", const HINT, options(nostack));
}

pub const CRWRW: i32 = 0b00000;
pub const CR_R_: i32 = 0b00101;
pub const C_W_W: i32 = 0b01010;

pub const ORWRW: i32 = 0b10000;
pub const OR_R_: i32 = 0b10101;
pub const O_W_W: i32 = 0b11010;

pub const ORW_W: i32 = 0b10010;
pub const OR_RW: i32 = 0b10100;

#[inline(always)] pub unsafe fn c_sync() { dbar::<CRWRW>(); }
#[inline(always)] pub unsafe fn c_rsync() { dbar::<CR_R_>(); }
#[inline(always)] pub unsafe fn c_wsync() { dbar::<C_W_W>(); }

#[inline(always)] pub unsafe fn o_sync() { dbar::<ORWRW>(); }
#[inline(always)] pub unsafe fn o_rsync() { dbar::<OR_R_>(); }
#[inline(always)] pub unsafe fn o_wsync() { dbar::<O_W_W>(); }

#[inline(always)] pub unsafe fn ldacq_mb() { dbar::<OR_RW>(); }
#[inline(always)] pub unsafe fn strel_mb() { dbar::<ORW_W>(); }

#[inline(always)] pub unsafe fn mb() { c_sync(); }
#[inline(always)] pub unsafe fn rmb() { c_rsync(); }
#[inline(always)] pub unsafe fn wmb() { c_wsync(); }
#[inline(always)] pub unsafe fn iob() { c_sync(); }
#[inline(always)] pub unsafe fn wbflush() { c_sync(); }

#[inline(always)] pub unsafe fn __smp_mb() { o_sync(); }
#[inline(always)] pub unsafe fn __smp_rmb() { o_rsync(); }
#[inline(always)] pub unsafe fn __smp_wmb() { o_wsync(); }

/* CONFIG_SMP controls whether the weak LL/SC barrier emits dbar 0x700. */
#[cfg(feature = "CONFIG_SMP")]
pub const __WEAK_LLSC_MB: &str = "\tdbar 0x700\n";
#[cfg(not(feature = "CONFIG_SMP"))]
pub const __WEAK_LLSC_MB: &str = "\t\n";

/* Supplied by the generic barrier implementation. */
extern "Rust" {
    pub fn barrier();
}

#[inline(always)] pub unsafe fn __smp_mb__before_atomic() { barrier(); }
#[inline(always)] pub unsafe fn __smp_mb__after_atomic() { barrier(); }

/**
 * array_index_mask_nospec() - generate a ~0 mask when index < size, 0 otherwise
 * @index: array element index
 * @size: number of elements in array
 *
 * Returns:
 *     0 - (@index < @size)
 */
#[inline]
pub fn array_index_mask_nospec(index: usize, size: usize) -> usize {
    if index < size { usize::MAX } else { 0 }
}

/* __smp_load_acquire and __smp_store_release use READ_ONCE/WRITE_ONCE,
 * compiletime_assert_atomic_type, and the acquire/release barriers supplied
 * by the surrounding kernel translation. */

#[inline(always)]
pub unsafe fn __smp_load_acquire<T: Copy>(p: *const T) -> T {
    let value = core::ptr::read_volatile(p);
    ldacq_mb();
    value
}

#[inline(always)]
pub unsafe fn __smp_store_release<T>(p: *mut T, v: T) {
    strel_mb();
    core::ptr::write_volatile(p, v);
}

/* __smp_store_mb performs an atomic store with a full memory barrier, with
 * byte, halfword, word, and doubleword cases matching the C implementation. */
#[inline(always)]
pub unsafe fn __smp_store_mb<T>(p: *mut T, v: T) {
    core::ptr::write_volatile(p, v);
    __smp_mb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
