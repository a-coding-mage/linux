/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file provides wrappers with sanitizer instrumentation for bit
 * locking operations.
 *
 * The architecture-specific arch_* bit operations, instrumentation helpers,
 * and BIT_WORD are supplied by other translated dependencies.
 */

/**
 * clear_bit_unlock - Clear a bit in memory, for unlock
 * @nr: the bit to set
 * @addr: the address to start counting from
 *
 * This operation is atomic and provides release barrier semantics.
 */
#[inline]
pub unsafe fn clear_bit_unlock(nr: core::ffi::c_long, addr: *mut core::ffi::c_ulong) {
    kcsan_release();
    instrument_atomic_write(addr.add(BIT_WORD(nr) as usize), core::mem::size_of::<core::ffi::c_long>());
    arch_clear_bit_unlock(nr, addr);
}

/**
 * __clear_bit_unlock - Clears a bit in memory
 * @nr: Bit to clear
 * @addr: Address to start counting from
 *
 * This is a non-atomic operation but implies a release barrier before the
 * memory operation. It can be used for an unlock if no other CPUs can
 * concurrently modify other bits in the word.
 */
#[inline]
pub unsafe fn __clear_bit_unlock(nr: core::ffi::c_long, addr: *mut core::ffi::c_ulong) {
    kcsan_release();
    instrument_write(addr.add(BIT_WORD(nr) as usize), core::mem::size_of::<core::ffi::c_long>());
    arch___clear_bit_unlock(nr, addr);
}

/**
 * test_and_set_bit_lock - Set a bit and return its old value, for lock
 * @nr: Bit to set
 * @addr: Address to count from
 *
 * This operation is atomic and provides acquire barrier semantics if
 * the returned value is 0.
 * It can be used to implement bit locks.
 */
#[inline]
pub unsafe fn test_and_set_bit_lock(
    nr: core::ffi::c_long,
    addr: *mut core::ffi::c_ulong,
) -> bool {
    instrument_atomic_read_write(
        addr.add(BIT_WORD(nr) as usize),
        core::mem::size_of::<core::ffi::c_long>(),
    );
    arch_test_and_set_bit_lock(nr, addr)
}

/**
 * xor_unlock_is_negative_byte - XOR a single byte in memory and test if
 * it is negative, for unlock.
 * @mask: Change the bits which are set in this mask.
 * @addr: The address of the word containing the byte to change.
 *
 * Changes some of bits 0-6 in the word pointed to by @addr.
 * This operation is atomic and provides release barrier semantics.
 * Used to optimise some folio operations which are commonly paired
 * with an unlock or end of writeback.  Bit 7 is used as PG_waiters to
 * indicate whether anybody is waiting for the unlock.
 *
 * Return: Whether the top bit of the byte is set.
 */
#[inline]
pub unsafe fn xor_unlock_is_negative_byte(
    mask: core::ffi::c_ulong,
    addr: *mut core::ffi::c_ulong,
) -> bool {
    kcsan_release();
    instrument_atomic_write(addr, core::mem::size_of::<core::ffi::c_long>());
    arch_xor_unlock_is_negative_byte(mask, addr)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
