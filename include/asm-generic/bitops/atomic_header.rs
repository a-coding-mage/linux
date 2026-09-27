/* SPDX-License-Identifier: GPL-2.0 */
// Translated from asm-generic/bitops/atomic.h.
// Dependencies supplied by the original C headers are expected externally:
// linux/atomic.h, linux/compiler.h, asm/barrier.h, and
// asm-generic/bitops/instrumented-atomic.h.

/*
 * Implementation of atomic bitops using atomic-fetch ops.
 * See Documentation/atomic_bitops.txt for details.
 */

extern "C" {
    fn raw_atomic_long_or(mask: kernel::ffi::c_ulong, v: *mut atomic_long_t);
    fn raw_atomic_long_andnot(mask: kernel::ffi::c_ulong, v: *mut atomic_long_t);
    fn raw_atomic_long_xor(mask: kernel::ffi::c_ulong, v: *mut atomic_long_t);
    fn raw_atomic_long_fetch_or(
        mask: kernel::ffi::c_ulong,
        v: *mut atomic_long_t,
    ) -> kernel::ffi::c_long;
    fn raw_atomic_long_fetch_andnot(
        mask: kernel::ffi::c_ulong,
        v: *mut atomic_long_t,
    ) -> kernel::ffi::c_long;
    fn raw_atomic_long_fetch_xor(
        mask: kernel::ffi::c_ulong,
        v: *mut atomic_long_t,
    ) -> kernel::ffi::c_long;
}

#[inline(always)]
unsafe fn arch_set_bit(nr: kernel::ffi::c_uint, mut p: *mut kernel::ffi::c_ulong) {
    p = p.add(BIT_WORD(nr) as usize);
    raw_atomic_long_or(BIT_MASK(nr), p as *mut atomic_long_t);
}

#[inline(always)]
unsafe fn arch_clear_bit(nr: kernel::ffi::c_uint, mut p: *mut kernel::ffi::c_ulong) {
    p = p.add(BIT_WORD(nr) as usize);
    raw_atomic_long_andnot(BIT_MASK(nr), p as *mut atomic_long_t);
}

#[inline(always)]
unsafe fn arch_change_bit(nr: kernel::ffi::c_uint, mut p: *mut kernel::ffi::c_ulong) {
    p = p.add(BIT_WORD(nr) as usize);
    raw_atomic_long_xor(BIT_MASK(nr), p as *mut atomic_long_t);
}

#[inline(always)]
unsafe fn arch_test_and_set_bit(
    nr: kernel::ffi::c_uint,
    mut p: *mut kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    let mask: kernel::ffi::c_ulong = BIT_MASK(nr);
    p = p.add(BIT_WORD(nr) as usize);
    let old: kernel::ffi::c_long = raw_atomic_long_fetch_or(mask, p as *mut atomic_long_t);
    ((old as kernel::ffi::c_ulong & mask) != 0) as kernel::ffi::c_int
}

#[inline(always)]
unsafe fn arch_test_and_clear_bit(
    nr: kernel::ffi::c_uint,
    mut p: *mut kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    let mask: kernel::ffi::c_ulong = BIT_MASK(nr);
    p = p.add(BIT_WORD(nr) as usize);
    let old: kernel::ffi::c_long = raw_atomic_long_fetch_andnot(mask, p as *mut atomic_long_t);
    ((old as kernel::ffi::c_ulong & mask) != 0) as kernel::ffi::c_int
}

#[inline(always)]
unsafe fn arch_test_and_change_bit(
    nr: kernel::ffi::c_uint,
    mut p: *mut kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    let mask: kernel::ffi::c_ulong = BIT_MASK(nr);
    p = p.add(BIT_WORD(nr) as usize);
    let old: kernel::ffi::c_long = raw_atomic_long_fetch_xor(mask, p as *mut atomic_long_t);
    ((old as kernel::ffi::c_ulong & mask) != 0) as kernel::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
