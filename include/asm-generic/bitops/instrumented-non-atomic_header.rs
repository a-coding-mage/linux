/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file provides wrappers with sanitizer instrumentation for non-atomic
 * bit operations.
 *
 * To use this functionality, an arch's bitops.h file needs to define each of
 * the below bit operations with an arch_ prefix (e.g. arch_set_bit(),
 * arch___set_bit(), etc.).
 */

/* Depends on: linux/instrumented.h */

/**
 * ___set_bit - Set a bit in memory
 * @nr: the bit to set
 * @addr: the address to start counting from
 *
 * Unlike set_bit(), this function is non-atomic. If it is called on the same
 * region of memory concurrently, the effect may be that only one operation
 * succeeds.
 */
#[inline(always)]
pub unsafe fn ___set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        instrument_write(
            addr.wrapping_add(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch___set_bit(nr, addr);
    }
}

/**
 * ___clear_bit - Clears a bit in memory
 * @nr: the bit to clear
 * @addr: the address to start counting from
 *
 * Unlike clear_bit(), this function is non-atomic. If it is called on the same
 * region of memory concurrently, the effect may be that only one operation
 * succeeds.
 */
#[inline(always)]
pub unsafe fn ___clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        instrument_write(
            addr.wrapping_add(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch___clear_bit(nr, addr);
    }
}

/**
 * ___change_bit - Toggle a bit in memory
 * @nr: the bit to change
 * @addr: the address to start counting from
 *
 * Unlike change_bit(), this function is non-atomic. If it is called on the same
 * region of memory concurrently, the effect may be that only one operation
 * succeeds.
 */
#[inline(always)]
pub unsafe fn ___change_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        instrument_write(
            addr.wrapping_add(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch___change_bit(nr, addr);
    }
}

#[inline(always)]
pub unsafe fn __instrument_read_write_bitop(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    let word = addr.wrapping_offset(BIT_WORD!(nr));
    if cfg!(CONFIG_KCSAN_ASSUME_PLAIN_WRITES_ATOMIC) {
        /*
         * We treat non-atomic read-write bitops a little more special.
         * Given the operations here only modify a single bit, assuming
         * non-atomicity of the writer is sufficient may be reasonable
         * for certain usage (and follows the permissible nature of the
         * assume-plain-writes-atomic rule):
         * 1. report read-modify-write races -> check read;
         * 2. do not report races with marked readers, but do report
         *    races with unmarked readers -> check "atomic" write.
         */
        kcsan_check_read!(word, core::mem::size_of::<kernel::ffi::c_long>());
        /*
         * Use generic write instrumentation, in case other sanitizers
         * or tools are enabled alongside KCSAN.
         */
        unsafe { instrument_write(word.cast(), core::mem::size_of::<kernel::ffi::c_long>()) };
    } else {
        unsafe { instrument_read_write(word.cast(), core::mem::size_of::<kernel::ffi::c_long>()) };
    }
}

/**
 * ___test_and_set_bit - Set a bit and return its old value
 * @nr: Bit to set
 * @addr: Address to count from
 *
 * This operation is non-atomic. If two instances of this operation race, one
 * can appear to succeed but actually fail.
 */
#[inline(always)]
pub unsafe fn ___test_and_set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe {
        __instrument_read_write_bitop(nr as kernel::ffi::c_long, addr);
        arch___test_and_set_bit(nr, addr)
    }
}

/**
 * ___test_and_clear_bit - Clear a bit and return its old value
 * @nr: Bit to clear
 * @addr: Address to count from
 *
 * This operation is non-atomic. If two instances of this operation race, one
 * can appear to succeed but actually fail.
 */
#[inline(always)]
pub unsafe fn ___test_and_clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe {
        __instrument_read_write_bitop(nr as kernel::ffi::c_long, addr);
        arch___test_and_clear_bit(nr, addr)
    }
}

/**
 * ___test_and_change_bit - Change a bit and return its old value
 * @nr: Bit to change
 * @addr: Address to count from
 *
 * This operation is non-atomic. If two instances of this operation race, one
 * can appear to succeed but actually fail.
 */
#[inline(always)]
pub unsafe fn ___test_and_change_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe {
        __instrument_read_write_bitop(nr as kernel::ffi::c_long, addr);
        arch___test_and_change_bit(nr, addr)
    }
}

/**
 * _test_bit - Determine whether a bit is set
 * @nr: bit number to test
 * @addr: Address to start counting from
 */
#[inline(always)]
pub unsafe fn _test_bit(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool {
    unsafe {
        instrument_atomic_read(
            addr.wrapping_add(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_test_bit(nr, addr)
    }
}

/**
 * _test_bit_acquire - Determine, with acquire semantics, whether a bit is set
 * @nr: bit number to test
 * @addr: Address to start counting from
 */
#[inline(always)]
pub unsafe fn _test_bit_acquire(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool {
    unsafe {
        instrument_atomic_read(
            addr.wrapping_add(BIT_WORD!(nr)).cast(),
            core::mem::size_of::<kernel::ffi::c_long>(),
        );
        arch_test_bit_acquire(nr, addr)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
