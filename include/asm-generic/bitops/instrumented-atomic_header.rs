/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file provides wrappers with sanitizer instrumentation for atomic bit
 * operations.
 *
 * The architecture-specific arch_* bit operations and instrumentation helpers
 * are supplied by other translation units.
 */

unsafe extern "C" {
    fn instrument_atomic_write(addr: *mut core::ffi::c_void, size: usize);
    fn instrument_atomic_read_write(addr: *mut core::ffi::c_void, size: usize);
    fn kcsan_mb();

    fn arch_set_bit(nr: i64, addr: *mut usize);
    fn arch_clear_bit(nr: i64, addr: *mut usize);
    fn arch_change_bit(nr: i64, addr: *mut usize);
    fn arch_test_and_set_bit(nr: i64, addr: *mut usize) -> bool;
    fn arch_test_and_clear_bit(nr: i64, addr: *mut usize) -> bool;
    fn arch_test_and_change_bit(nr: i64, addr: *mut usize) -> bool;
}

#[inline(always)]
unsafe fn bit_word_addr(addr: *mut usize, nr: i64) -> *mut usize {
    /* BIT_WORD(nr), expressed for Rust's unsigned-long representation. */
    addr.add((nr as usize) / (usize::BITS as usize))
}

/**
 * set_bit - Atomically set a bit in memory
 * @nr: the bit to set
 * @addr: the address to start counting from
 *
 * This is a relaxed atomic operation (no implied memory barriers).
 */
#[inline(always)]
pub unsafe fn set_bit(nr: i64, addr: *mut usize) {
    instrument_atomic_write(
        bit_word_addr(addr, nr).cast::<core::ffi::c_void>(),
        core::mem::size_of::<usize>(),
    );
    arch_set_bit(nr, addr);
}

/** clear_bit - Clears a bit in memory */
#[inline(always)]
pub unsafe fn clear_bit(nr: i64, addr: *mut usize) {
    instrument_atomic_write(
        bit_word_addr(addr, nr).cast::<core::ffi::c_void>(),
        core::mem::size_of::<usize>(),
    );
    arch_clear_bit(nr, addr);
}

/** change_bit - Toggle a bit in memory */
#[inline(always)]
pub unsafe fn change_bit(nr: i64, addr: *mut usize) {
    instrument_atomic_write(
        bit_word_addr(addr, nr).cast::<core::ffi::c_void>(),
        core::mem::size_of::<usize>(),
    );
    arch_change_bit(nr, addr);
}

/** test_and_set_bit - Set a bit and return its old value */
#[inline(always)]
pub unsafe fn test_and_set_bit(nr: i64, addr: *mut usize) -> bool {
    kcsan_mb();
    instrument_atomic_read_write(
        bit_word_addr(addr, nr).cast::<core::ffi::c_void>(),
        core::mem::size_of::<usize>(),
    );
    arch_test_and_set_bit(nr, addr)
}

/** test_and_clear_bit - Clear a bit and return its old value */
#[inline(always)]
pub unsafe fn test_and_clear_bit(nr: i64, addr: *mut usize) -> bool {
    kcsan_mb();
    instrument_atomic_read_write(
        bit_word_addr(addr, nr).cast::<core::ffi::c_void>(),
        core::mem::size_of::<usize>(),
    );
    arch_test_and_clear_bit(nr, addr)
}

/** test_and_change_bit - Change a bit and return its old value */
#[inline(always)]
pub unsafe fn test_and_change_bit(nr: i64, addr: *mut usize) -> bool {
    kcsan_mb();
    instrument_atomic_read_write(
        bit_word_addr(addr, nr).cast::<core::ffi::c_void>(),
        core::mem::size_of::<usize>(),
    );
    arch_test_and_change_bit(nr, addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
