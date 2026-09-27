/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file provides wrappers with sanitizer instrumentation for atomic bit
 * operations.
 *
 * To use this functionality, an arch's bitops.h file needs to define each of
 * the below bit operations with an arch_ prefix (e.g. arch_set_bit(),
 * arch___set_bit(), etc.).
 */

/* Depends on: linux/instrumented.h */

/**
 * set_bit - Atomically set a bit in memory
 * @nr: the bit to set
 * @addr: the address to start counting from
 *
 * This is a relaxed atomic operation (no implied memory barriers).
 *
 * Note that @nr may be almost arbitrarily large; this function is not
 * restricted to acting on a single-word quantity.
 */
#[inline(always)]
pub unsafe fn set_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        instrument_atomic_write(
            addr.wrapping_offset(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_set_bit(nr, addr);
    }
}

/**
 * clear_bit - Clears a bit in memory
 * @nr: Bit to clear
 * @addr: Address to start counting from
 *
 * This is a relaxed atomic operation (no implied memory barriers).
 */
#[inline(always)]
pub unsafe fn clear_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        instrument_atomic_write(
            addr.wrapping_offset(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_clear_bit(nr, addr);
    }
}

/**
 * change_bit - Toggle a bit in memory
 * @nr: Bit to change
 * @addr: Address to start counting from
 *
 * This is a relaxed atomic operation (no implied memory barriers).
 *
 * Note that @nr may be almost arbitrarily large; this function is not
 * restricted to acting on a single-word quantity.
 */
#[inline(always)]
pub unsafe fn change_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        instrument_atomic_write(
            addr.wrapping_offset(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_change_bit(nr, addr);
    }
}

/**
 * test_and_set_bit - Set a bit and return its old value
 * @nr: Bit to set
 * @addr: Address to count from
 *
 * This is an atomic fully-ordered operation (implied full memory barrier).
 */
#[inline(always)]
pub unsafe fn test_and_set_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) -> bool {
    kcsan_mb!();
    unsafe {
        instrument_atomic_read_write(
            addr.wrapping_offset(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_test_and_set_bit(nr, addr)
    }
}

/**
 * test_and_clear_bit - Clear a bit and return its old value
 * @nr: Bit to clear
 * @addr: Address to count from
 *
 * This is an atomic fully-ordered operation (implied full memory barrier).
 */
#[inline(always)]
pub unsafe fn test_and_clear_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) -> bool {
    kcsan_mb!();
    unsafe {
        instrument_atomic_read_write(
            addr.wrapping_offset(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_test_and_clear_bit(nr, addr)
    }
}

/**
 * test_and_change_bit - Change a bit and return its old value
 * @nr: Bit to change
 * @addr: Address to count from
 *
 * This is an atomic fully-ordered operation (implied full memory barrier).
 */
#[inline(always)]
pub unsafe fn test_and_change_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) -> bool {
    kcsan_mb!();
    unsafe {
        instrument_atomic_read_write(
            addr.wrapping_offset(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_test_and_change_bit(nr, addr)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
